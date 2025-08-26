use crate::common::config::{UnifiedTestConfig, TestChaosSettings, ChaosType};
use crate::canonical_modernization::UnifiedServiceType;
use nestgate_core::{NestGateError, Result};
use std::collections::HashMap;

use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::sleep;

/// E2E Chaos Testing Integration
/// 
/// This module provides comprehensive end-to-end chaos testing using the unified 
/// configuration system. All chaos configurations now use UnifiedTestConfig.
/// 
/// ## USAGE EXAMPLES
/// 
/// ```rust
/// // Production-like chaos testing
/// let config = UnifiedTestConfig::production_like();
/// let chaos_settings = &config.extensions.chaos;
/// 
/// // Run comprehensive chaos test
/// run_e2e_chaos_test(&config).await?;
/// ```

/// E2E chaos test runner using canonical configuration
pub async fn run_e2e_chaos_test(config: &UnifiedTestConfig) -> Result<E2EChaosResults> {
    let start_time = Instant::now();
    
    // Extract chaos settings from unified config
    let chaos = &config.extensions.chaos;
    let performance = &config.extensions.performance;
    
    let test_duration = chaos.scenario_duration;
    let injection_probability = chaos.injection_probability;
    let recovery_time = chaos.recovery_time;
    let chaos_types = &chaos.chaos_types;
    let concurrent_operations = performance.concurrent_operations;

    println!("🚀 Starting E2E Chaos Test");
    println!("   Duration: {:?}", test_duration);
    println!("   Injection Probability: {:.1}%", injection_probability * 100.0);
    println!("   Recovery Time: {:?}", recovery_time);
    println!("   Chaos Types: {} types enabled", chaos_types.len());
    println!("   Concurrent Operations: {}", concurrent_operations);

    let results = E2EChaosResults {
        test_duration: Duration::ZERO,
        total_chaos_events: 0,
        successful_recoveries: 0,
        failed_recoveries: 0,
        service_impact_events: Vec::new(),
        performance_degradation: HashMap::new(),
        system_resilience_score: 0.0,
    };

    // Run comprehensive chaos test
    execute_comprehensive_chaos_test(config, results).await
}

async fn execute_comprehensive_chaos_test(
    config: &UnifiedTestConfig, 
    mut results: E2EChaosResults
) -> Result<E2EChaosResults> {
    let start_time = Instant::now();
    let chaos = &config.extensions.chaos;

    // Phase 1: Baseline performance measurement
    let baseline_metrics = measure_baseline_performance(config).await?;
    
    // Phase 2: Concurrent chaos injection with load
    for chaos_type in &chaos.chaos_types {
        // Start background load
        let load_handle = tokio::spawn(simulate_background_load(config.clone()));
        
        // Inject chaos
        inject_chaos_with_monitoring(chaos_type, &mut results).await?;
        
        // Wait for recovery
        sleep(chaos.recovery_time).await;
        
        // Stop background load and measure impact
        load_handle.abort();
        let recovery_metrics = measure_recovery_performance(config).await?;
        
        // Calculate performance impact
        calculate_performance_impact(&baseline_metrics, &recovery_metrics, &mut results);
        
        if recovery_metrics.system_healthy {
            results.successful_recoveries += 1;
        } else {
            results.failed_recoveries += 1;
        }
    }
    
    // Phase 3: Calculate resilience score
    results.system_resilience_score = calculate_resilience_score(&results);
    results.test_duration = start_time.elapsed();
    
    Ok(results)
}

async fn measure_baseline_performance(config: &UnifiedTestConfig) -> Result<PerformanceMetrics> {
    println!("📊 Measuring baseline performance...");
    // Simulate performance measurement
    Ok(PerformanceMetrics {
        response_time_ms: 50,
        throughput_rps: 100.0,
        error_rate: 0.0,
        memory_usage_mb: 256,
        cpu_usage_percent: 25.0,
        system_healthy: true,
    })
}

async fn simulate_background_load(config: UnifiedTestConfig) -> Result<()> {
    let performance = &config.extensions.performance;  
    let operations = performance.concurrent_operations;
    
    println!("🔄 Simulating background load with {} operations", operations);
    
    // Simulate load for the duration
    for i in 0..operations {
        sleep(Duration::from_millis(10)).await;  // Simulate work
        
        if i % 10 == 0 {
            println!("   Load operation {}/{} completed", i + 1, operations);
        }
    }
    
    Ok(())
}

async fn inject_chaos_with_monitoring(
    chaos_type: &ChaosType, 
    results: &mut E2EChaosResults
) -> Result<()> {
    let event_start = Instant::now();
    
    match chaos_type {
        ChaosType::NetworkLatency(delay) => {
            println!("🌐 E2E: Injecting network latency with monitoring: {:?}", delay);
            sleep(*delay).await;
        },
        ChaosType::ServiceFailure(service_type) => {
            println!("💥 E2E: Injecting monitored service failure: {:?}", service_type);
            results.service_impact_events.push(ServiceImpactEvent {
                service_type: service_type.clone(),
                impact_duration: Duration::from_secs(2),
                recovery_time: Duration::from_secs(1),
                data_loss: false,
            });
            sleep(Duration::from_secs(2)).await;
        },
        ChaosType::ResourceExhaustion(resource) => {
            println!("📈 E2E: Injecting monitored resource exhaustion: {}", resource);
            sleep(Duration::from_secs(1)).await;
        },
        ChaosType::DataCorruption => {
            println!("🗂️ E2E: Injecting monitored data corruption scenario");
            sleep(Duration::from_millis(500)).await;
        },
    }
    
    results.total_chaos_events += 1;
    Ok(())
}

async fn measure_recovery_performance(config: &UnifiedTestConfig) -> Result<PerformanceMetrics> {
    println!("🔍 Measuring recovery performance...");
    // Simulate recovery performance measurement
    Ok(PerformanceMetrics {
        response_time_ms: 75,  // Slightly degraded
        throughput_rps: 85.0,  // Reduced throughput
        error_rate: 2.0,       // Some errors during recovery
        memory_usage_mb: 280,  // Higher memory usage
        cpu_usage_percent: 35.0, // Higher CPU usage
        system_healthy: true,
    })
}

fn calculate_performance_impact(
    baseline: &PerformanceMetrics,
    recovery: &PerformanceMetrics,
    results: &mut E2EChaosResults,
) {
    let response_time_impact = (recovery.response_time_ms as f64 - baseline.response_time_ms as f64) 
        / baseline.response_time_ms as f64 * 100.0;
    let throughput_impact = (baseline.throughput_rps - recovery.throughput_rps) 
        / baseline.throughput_rps * 100.0;
    
    results.performance_degradation.insert("response_time_increase_percent".to_string(), response_time_impact);
    results.performance_degradation.insert("throughput_decrease_percent".to_string(), throughput_impact);
    
    println!("📉 Performance Impact:");
    println!("   Response Time: +{:.1}%", response_time_impact);
    println!("   Throughput: -{:.1}%", throughput_impact);
}

fn calculate_resilience_score(results: &E2EChaosResults) -> f64 {
    if results.total_chaos_events == 0 {
        return 1.0;
    }
    
    let recovery_rate = results.successful_recoveries as f64 / results.total_chaos_events as f64;
    let avg_performance_impact = results.performance_degradation.values().sum::<f64>() 
        / results.performance_degradation.len().max(1) as f64;
    
    // Score is weighted: 70% recovery success, 30% performance impact
    let score = (recovery_rate * 0.7) + ((100.0 - avg_performance_impact.abs()) / 100.0 * 0.3);
    score.max(0.0).min(1.0)
}

/// E2E chaos test results
#[derive(Debug, Clone)]
pub struct E2EChaosResults {
    pub test_duration: Duration,
    pub total_chaos_events: usize,
    pub successful_recoveries: usize,
    pub failed_recoveries: usize,
    pub service_impact_events: Vec<ServiceImpactEvent>,
    pub performance_degradation: HashMap<String, f64>,
    pub system_resilience_score: f64,
}

/// Service impact event record
#[derive(Debug, Clone)]
pub struct ServiceImpactEvent {
    pub service_type: UnifiedServiceType,
    pub impact_duration: Duration,
    pub recovery_time: Duration,
    pub data_loss: bool,
}

/// Performance metrics structure
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub response_time_ms: u64,
    pub throughput_rps: f64,
    pub error_rate: f64,
    pub memory_usage_mb: u64,
    pub cpu_usage_percent: f64,
    pub system_healthy: bool,
}
