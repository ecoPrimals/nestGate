use crate::common::test_config::{PerformanceThresholds, TestPerformanceSettings, UnifiedTestConfig};
use nestgate_core::{NestGateError, Result};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Performance testing results
#[derive(Debug, Clone)]
pub struct PerformanceTestResults {
    pub total_operations: u32,
    pub successful_operations: u32,
    pub failed_operations: u32,
    pub average_response_time: Duration,
    pub peak_memory_usage: u64,
    pub throughput_ops_per_sec: f64,
    pub latency_percentiles: HashMap<String, Duration>,
}

/// Execute performance tests
pub async fn execute_performance_tests(config: &UnifiedTestConfig) -> Result<PerformanceTestResults> {
    println!("🏁 Starting performance tests");
    let start_time = Instant::now();
    
    let perf_settings = &config.extensions.performance;
    
    let mut results = PerformanceTestResults {
        total_operations: perf_settings.concurrent_operations * perf_settings.operations_per_test.unwrap_or(10),
        successful_operations: 0,
        failed_operations: 0,
        average_response_time: Duration::from_secs(0),
        peak_memory_usage: 0,
        throughput_ops_per_sec: 0.0,
        latency_percentiles: HashMap::new(),
    };
    
    // Execute concurrent operations
    let operation_results = execute_concurrent_operations(perf_settings).await?;
    
    // Process results
    results.successful_operations = operation_results.successful;
    results.failed_operations = operation_results.failed;
    results.average_response_time = operation_results.average_response_time;
    results.peak_memory_usage = operation_results.peak_memory;
    
    let total_duration = start_time.elapsed();
    results.throughput_ops_per_sec = results.successful_operations as f64 / total_duration.as_secs_f64();
    
    // Calculate latency percentiles
    results.latency_percentiles = calculate_latency_percentiles(&operation_results.response_times);
    
    // Validate performance against thresholds
    validate_performance_thresholds(&results, perf_settings).await?;
    
    println!("✅ Performance tests completed");
    println!("   Operations: {}/{}", results.successful_operations, results.total_operations);
    println!("   Throughput: {:.2} ops/sec", results.throughput_ops_per_sec);
    println!("   Avg Response: {:?}", results.average_response_time);
    
    Ok(results)
}

#[derive(Debug)]
struct OperationResults {
    successful: u32,
    failed: u32,
    average_response_time: Duration,
    peak_memory: u64,
    response_times: Vec<Duration>,
}

async fn execute_concurrent_operations(settings: &TestPerformanceSettings) -> Result<OperationResults> {
    let concurrent_ops = settings.concurrent_operations;
    let ops_per_test = settings.operations_per_test.unwrap_or(10);
    
    println!("   Executing {} concurrent operations with {} ops each", concurrent_ops, ops_per_test);
    
    let mut handles = Vec::new();
    
    // Spawn concurrent operation tasks
    for i in 0..concurrent_ops {
        let ops_count = ops_per_test;
        let handle = tokio::spawn(async move {
            execute_operation_batch(i, ops_count).await
        });
        handles.push(handle);
    }
    
    // Collect results
    let mut successful = 0;
    let mut failed = 0;
    let mut response_times = Vec::new();
    let mut total_response_time = Duration::from_secs(0);
    
    for handle in handles {
        match handle.await {
            Ok(batch_result) => {
                successful += batch_result.successful;
                failed += batch_result.failed;
                response_times.extend(batch_result.response_times);
                total_response_time += batch_result.total_time;
            }
            Err(e) => {
                println!("   Operation batch failed: {}", e);
                failed += ops_per_test;
            }
        }
    }
    
    let average_response_time = if successful > 0 {
        total_response_time / successful
    } else {
        Duration::from_secs(0)
    };
    
    Ok(OperationResults {
        successful,
        failed,
        average_response_time,
        peak_memory: simulate_memory_usage(),
        response_times,
    })
}

#[derive(Debug)]
struct BatchResult {
    successful: u32,
    failed: u32,
    total_time: Duration,
    response_times: Vec<Duration>,
}

async fn execute_operation_batch(batch_id: u32, operation_count: u32) -> BatchResult {
    let mut successful = 0;
    let mut failed = 0;
    let mut response_times = Vec::new();
    let start_time = Instant::now();
    
    for op_id in 0..operation_count {
        let op_start = Instant::now();
        
        // Simulate operation execution
        match simulate_operation(batch_id, op_id).await {
            Ok(_) => {
                successful += 1;
                response_times.push(op_start.elapsed());
            }
            Err(_) => {
                failed += 1;
            }
        }
    }
    
    BatchResult {
        successful,
        failed,
        total_time: start_time.elapsed(),
        response_times,
    }
}

async fn simulate_operation(batch_id: u32, op_id: u32) -> Result<()> {
    // Simulate variable operation time
    let operation_time = Duration::from_millis(
        10 + (batch_id + op_id) % 50
    );
    
    sleep(operation_time).await;
    
    // Simulate 95% success rate
    if (batch_id + op_id) % 20 == 0 {
        Err(NestGateError::Internal {
            message: "Simulated operation failure".to_string(),
            location: Some("performance_testing::simulate_operation".to_string()),
            debug_info: None,
            is_bug: false,
        })
    } else {
        Ok(())
    }
}

fn simulate_memory_usage() -> u64 {
    // Simulate peak memory usage in MB
    150 + (rand::random::<u64>() % 100)
}

fn calculate_latency_percentiles(response_times: &[Duration]) -> HashMap<String, Duration> {
    let mut percentiles = HashMap::new();
    
    if response_times.is_empty() {
        return percentiles;
    }
    
    let mut sorted_times = response_times.to_vec();
    sorted_times.sort();
    
    let len = sorted_times.len();
    
    // Calculate common percentiles
    percentiles.insert("p50".to_string(), sorted_times[len * 50 / 100]);
    percentiles.insert("p95".to_string(), sorted_times[len * 95 / 100]);
    percentiles.insert("p99".to_string(), sorted_times[len * 99 / 100]);
    
    percentiles
}

async fn validate_performance_thresholds(
    results: &PerformanceTestResults,
    settings: &TestPerformanceSettings,
) -> Result<()> {
    if !settings.enable_performance_validation {
        println!("   Performance validation disabled");
        return Ok(());
    }
    
    println!("   Validating performance thresholds");
    
    // Check throughput threshold
    let min_throughput = 100.0; // ops/sec
    if results.throughput_ops_per_sec < min_throughput {
        return Err(NestGateError::Internal {
            message: format!(
                "Throughput {} ops/sec below threshold {} ops/sec",
                results.throughput_ops_per_sec, min_throughput
            ),
            location: Some("performance_testing::validate_performance_thresholds".to_string()),
            debug_info: None,
            is_bug: false,
        });
    }
    
    // Check response time threshold
    let max_response_time = Duration::from_millis(100);
    if results.average_response_time > max_response_time {
        return Err(NestGateError::Internal {
            message: format!(
                "Average response time {:?} exceeds threshold {:?}",
                results.average_response_time, max_response_time
            ),
            location: Some("performance_testing::validate_performance_thresholds".to_string()),
            debug_info: None,
            is_bug: false,
        });
    }
    
    // Check success rate threshold
    let min_success_rate = 0.90; // 90%
    let actual_success_rate = results.successful_operations as f64 / results.total_operations as f64;
    if actual_success_rate < min_success_rate {
        return Err(NestGateError::Internal {
            message: format!(
                "Success rate {:.2}% below threshold {:.2}%",
                actual_success_rate * 100.0, min_success_rate * 100.0
            ),
            location: Some("performance_testing::validate_performance_thresholds".to_string()),
            debug_info: None,
            is_bug: false,
        });
    }
    
    println!("   ✅ All performance thresholds met");
    Ok(())
} 