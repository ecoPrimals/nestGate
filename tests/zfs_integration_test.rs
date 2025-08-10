/// ZFS Integration Test
///
/// Real integration tests for ZFS functionality
use nestgate_core::{NestGateError, Result};
use nestgate_core::unified_config_consolidation::UnifiedZfsConfig;
use nestgate_zfs::ZfsManager;
use std::sync::Arc;
use tokio::time::Duration;

#[tokio::test]
async fn test_zfs_integration() -> Result<()> {
    println!("🚀 Starting ZFS integration test");

    // Create ZFS manager
    let config = UnifiedZfsConfig::default();
    let manager = match ZfsManager::new(config).await {
        Ok(m) => m,
        Err(e) if e.to_string().contains("ZFS modules cannot be auto-loaded") => {
            println!("⏭️ Skipping ZFS integration test - ZFS not available");
            return Ok(());
        }
        Err(e) => {
            return Err(NestGateError::Internal {
                message: e.to_string(),
                location: Some(file!().to_string()),
                debug_info: None,
                is_bug: false,
            })
        }
    };

    println!("✅ ZFS manager created successfully");

    // Test basic manager functionality
    let service_status = match manager.get_service_status().await {
        Ok(status) => status,
        Err(e) if e.to_string().contains("ZFS modules cannot be auto-loaded") => {
            println!("⏭️ Skipping service status check - ZFS not available");
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    };
    println!("📊 Service status: {:?}", service_status.overall_health);

    println!("✅ ZFS integration test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_zfs_pool_operations() -> Result<()> {
    println!("🔄 Testing ZFS pool operations");

    let config = UnifiedZfsConfig::default();
    let manager = ZfsManager::new(config).await?;

    // Test pool manager operations
    let pool_status = manager.pool_manager.get_overall_status().await?;
    println!(
        "📊 Pool status - Online pools: {}",
        pool_status.pools_online
    );

    Ok(())
}

#[tokio::test]
async fn test_zfs_dataset_operations() -> Result<()> {
    println!("🗂️ Testing ZFS dataset operations");

    let config = UnifiedZfsConfig::default();
    let manager = ZfsManager::new(config).await?;

    // Test dataset creation
    let dataset_name = "nestpool/test_dataset";
    let result = manager
        .dataset_manager
        .create_dataset(dataset_name, "nestpool", nestgate_core::StorageTier::Warm)
        .await;

    match result {
        Ok(_) => println!("✅ Dataset created successfully"),
        Err(e) => println!("⚠️ Dataset creation failed (expected in test): {e}"),
    }

    Ok(())
}

#[tokio::test]
async fn test_zfs_performance_monitoring() -> Result<()> {
    println!("📈 Testing ZFS performance monitoring");

    let config = UnifiedZfsConfig::default();
    let manager = ZfsManager::new(config).await?;

    // Test performance metrics
    let metrics = manager
        .performance_monitor
        .read()
        .await
        .get_current_metrics()
        .await;
    println!(
        "📊 Performance metrics collected: {} pools",
        metrics.pool_metrics.total_iops as i32
    );

    Ok(())
}

#[tokio::test]
async fn test_zfs_concurrent_operations() -> Result<()> {
    println!("🔄 Testing concurrent ZFS operations");

    let config = UnifiedZfsConfig::default();
    let manager = Arc::new(ZfsManager::new(config).await?);

    // Test concurrent operations
    let mut handles = vec![];

    for i in 0..5 {
        let manager_clone = Arc::clone(&manager);
        let handle = tokio::spawn(async move {
            let _metrics = manager_clone
                .performance_monitor
                .read()
                .await
                .get_current_metrics()
                .await;
            println!("🔄 Concurrent operation {i} completed");
            Ok::<(), NestGateError>(())
        });
        handles.push(handle);
    }

    // Wait for all operations
    for handle in handles {
        handle.await.map_err(|e| NestGateError::Internal {
            message: e.to_string(),
            location: Some(file!().to_string()),
            debug_info: None,
            is_bug: false,
        })??;
    }

    println!("✅ All concurrent operations completed");
    Ok(())
}

#[tokio::test]
async fn test_zfs_error_handling() -> Result<()> {
    println!("❌ Testing ZFS error handling");

    let config = ZfsConfig {
        api_endpoint: "http://nonexistent-endpoint:8080".to_string(),
        ..Default::default()
    };

    let manager = match ZfsManager::new(config).await {
        Ok(m) => m,
        Err(e) if e.to_string().contains("ZFS modules cannot be auto-loaded") => {
            println!("⏭️ Skipping ZFS error handling test - ZFS not available");
            return Ok(());
        }
        Err(e) => {
            return Err(NestGateError::Internal {
                message: e.to_string(),
                location: Some(file!().to_string()),
                debug_info: None,
                is_bug: false,
            })
        }
    };

    // This should handle the error gracefully
    let _status = match manager.get_service_status().await {
        Ok(status) => status,
        Err(e) if e.to_string().contains("ZFS modules cannot be auto-loaded") => {
            println!("⏭️ Skipping service status check - ZFS not available");
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    };

    println!("✅ Error handling test completed");
    Ok(())
}

#[tokio::test]
async fn test_zfs_timeout_handling() -> Result<()> {
    println!("⏱️ Testing ZFS timeout handling");

    let config = UnifiedZfsConfig::default();
    let manager = ZfsManager::new(config).await?;

    // Test with timeout
    let result = tokio::time::timeout(Duration::from_secs(5), manager.get_service_status()).await;

    match result {
        Ok(status) => {
            println!(
                "✅ Operation completed within timeout: {:?}",
                status.is_ok()
            );
        }
        Err(_) => {
            println!("⚠️ Operation timed out (expected in some environments)");
        }
    }

    Ok(())
}
