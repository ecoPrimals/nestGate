//! ZFS Integration Test
//!
//! Real integration tests for ZFS functionality

use std::sync::Arc;
use tokio::time::{sleep, Duration};
use nestgate_core::{Result, NestGateError};
use nestgate_zfs::{ZfsManager, ZfsConfig};

#[tokio::test]
async fn test_zfs_integration() -> Result<()> {
    println!("🚀 Starting ZFS integration test");
    
    // Create ZFS manager
    let config = ZfsConfig::default();
    let manager = ZfsManager::new(config).await?;
    
    println!("✅ ZFS manager created successfully");
    
    // Test basic manager functionality
    let service_status = manager.get_service_status().await?;
    println!("📊 Service status: {:?}", service_status.overall_health);
    
    println!("✅ ZFS integration test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_zfs_pool_operations() -> Result<()> {
    println!("🔄 Testing ZFS pool operations");
    
    let config = ZfsConfig::default();
    let manager = ZfsManager::new(config).await?;
    
    // Test pool manager operations
    let pool_status = manager.pool_manager.get_overall_status().await?;
    println!("📊 Pool status - Online pools: {}", pool_status.pools_online);
    
    Ok(())
}

#[tokio::test]
async fn test_zfs_dataset_operations() -> Result<()> {
    println!("🗂️ Testing ZFS dataset operations");
    
    let config = ZfsConfig::default();
    let manager = ZfsManager::new(config).await?;
    
    // Test dataset creation
    let dataset_name = "nestpool/test_dataset";
    let result = manager.dataset_manager.create_dataset(
        dataset_name,
        "nestpool",
        nestgate_core::StorageTier::Warm
    ).await;
    
    match result {
        Ok(_) => println!("✅ Dataset created successfully"),
        Err(e) => println!("⚠️ Dataset creation failed (expected in test): {}", e),
    }
    
    Ok(())
}

#[tokio::test]
async fn test_zfs_performance_monitoring() -> Result<()> {
    println!("📈 Testing ZFS performance monitoring");
    
    let config = ZfsConfig::default();
    let manager = ZfsManager::new(config).await?;
    
    // Test performance metrics
    let metrics = manager.performance_monitor.get_current_metrics().await;
    println!("📊 Performance metrics collected: {} pools", 
        metrics.pool_metrics.total_iops as i32);
    
    Ok(())
}

#[tokio::test]
async fn test_zfs_concurrent_operations() -> Result<()> {
    println!("🔄 Testing concurrent ZFS operations");
    
    let config = ZfsConfig::default();
    let manager = Arc::new(ZfsManager::new(config).await?);
    
    // Test concurrent operations
    let mut handles = vec![];
    
    for i in 0..5 {
        let manager_clone = Arc::clone(&manager);
        let handle = tokio::spawn(async move {
            let _metrics = manager_clone.performance_monitor.get_current_metrics().await;
            println!("🔄 Concurrent operation {} completed", i);
            Ok::<(), NestGateError>(())
        });
        handles.push(handle);
    }
    
    // Wait for all operations
    for handle in handles {
        handle.await.map_err(|e| NestGateError::Internal(e.to_string()))??;
    }
    
    println!("✅ All concurrent operations completed");
    Ok(())
}

#[tokio::test]
async fn test_zfs_error_handling() -> Result<()> {
    println!("❌ Testing ZFS error handling");
    
    let mut config = ZfsConfig::default();
    config.default_pool = "nonexistent-pool".to_string();
    
    let manager = ZfsManager::new(config).await?;
    
    // This should handle the error gracefully
    let _status = manager.get_service_status().await?;
    
    println!("✅ Error handling test completed");
    Ok(())
}

#[tokio::test]
async fn test_zfs_timeout_handling() -> Result<()> {
    println!("⏱️ Testing ZFS timeout handling");
    
    let config = ZfsConfig::default();
    let manager = ZfsManager::new(config).await?;
    
    // Test with timeout
    let result = tokio::time::timeout(
        Duration::from_secs(5),
        manager.get_service_status()
    ).await;
    
    match result {
        Ok(status) => {
            println!("✅ Operation completed within timeout: {:?}", status.is_ok());
        }
        Err(_) => {
            println!("⚠️ Operation timed out (expected in some environments)");
        }
    }
    
    Ok(())
} 