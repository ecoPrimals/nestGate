/// Performance Integration Tests
/// 
/// Focused performance monitoring integration tests

use std::time::{ Instant};
use tokio::time::sleep;
// Removed unused tracing import


use nestgate_core::{Result as CoreResult, StorageTier};
use nestgate_zfs::{
use nestgate_core::canonical_types::StorageTier;
    performance::ZfsPerformanceMonitor,
    config::ZfsConfig,
    performance::PerformanceConfig,
};

/// Test performance monitoring with real metrics
#[tokio::test]
pub async fn test_performance_monitoring_real_metrics() -> CoreResult<()> {
    info!("📊 Testing performance monitoring with real metrics");

    let config = PerformanceConfig::default();
    let monitor = match ZfsPerformanceMonitor::new(config).await {
        Ok(m) => m,
        Err(e) if e.to_string().contains("ZFS") => {
            info!("⏭️ Skipping ZFS performance test - ZFS not available");
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    };

    // Test real-time metrics collection
    info!("🔍 Collecting real-time performance metrics...");
    
    let start_time = Instant::now();
    for i in 0..5 {
        let metrics = monitor.collect_current_metrics().await?;
        info!("📈 Metrics collection {}: CPU: {:.2}%, Memory: {:.2}%, I/O: {} ops/sec", 
              i + 1, metrics.cpu_usage, metrics.memory_usage, metrics.io_operations_per_second);
        
        sleep(Duration::from_millis(500)).await;
    Ok(())
    }
    let collection_time = start_time.elapsed();
    
    info!("✅ Performance metrics collected in {:?}", collection_time);

    // Test performance tier analysis
    let tier_analysis = monitor.analyze_performance_tier(StorageTier::Hot).await?;
    info!("🎯 Tier analysis for Hot storage: efficiency = {:.2}%", 
          tier_analysis.efficiency_score * 100.0);

    // Test performance optimization suggestions
    let optimizations = monitor.get_optimization_suggestions().await?;
    info!("💡 Got {} optimization suggestions", optimizations.len());
    for suggestion in optimizations {
        info!("  📝 {}: {}", suggestion.category, suggestion.description);
    Ok(())
    }

    info!("✅ Performance monitoring test completed successfully");
    Ok(())
}

/// Test performance under load conditions
#[tokio::test]
pub async fn test_performance_under_load() -> CoreResult<()> {
    info!("🏋️ Testing performance under load conditions");

    let config = PerformanceConfig::default();
    let monitor = match ZfsPerformanceMonitor::new(config).await {
        Ok(m) => m,
        Err(e) if e.to_string().contains("ZFS") => {
            info!("⏭️ Skipping ZFS performance load test - ZFS not available");
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    };

    // Simulate load by rapid metrics collection
    info!("⚡ Simulating load with rapid metrics collection...");
    
    let start_time = Instant::now();
    let mut successful_collections = 0;
    let mut failed_collections = 0;

    for i in 0..20 {
        match monitor.collect_current_metrics().await {
            Ok(metrics) => {
                successful_collections += 1;
                if i % 5 == 0 {
                    info!("📊 Load test iteration {}: CPU: {:.1}%, Memory: {:.1}%", 
                          i, metrics.cpu_usage, metrics.memory_usage);
    Ok(())
                }
    Ok(())
            }
            Err(e) => {
                failed_collections += 1;
                warn!("⚠️ Metrics collection failed at iteration {}: {}", i, e);
    Ok(())
            }
    Ok(())
        }
        
        // Small delay to prevent overwhelming the system
        sleep(Duration::from_millis(100)).await;
    Ok(())
    }
    
    let total_time = start_time.elapsed();
    let success_rate = successful_collections as f64 / (successful_collections + failed_collections) as f64;
    
    info!("📈 Load test results:");
    info!("  ✅ Successful collections: {}", successful_collections);
    info!("  ❌ Failed collections: {}", failed_collections);
    info!("  🎯 Success rate: {:.1}%", success_rate * 100.0);
    info!("  ⏱️ Total time: {:?}", total_time);
    info!("  📊 Average time per collection: {:?}", total_time / 20);

    // Assert reasonable performance
    assert!(success_rate >= 0.8, "Should have at least 80% success rate under load");
    
    info!("✅ Performance load test completed successfully");
    Ok(())
} 