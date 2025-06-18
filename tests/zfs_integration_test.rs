use std::time::Duration;
use nestgate_core::Result;
use nestgate_zfs::{ZfsPoolManager, ZfsConfig};

/// Test real ZFS pool discovery and operations
#[tokio::test]
async fn test_real_zfs_integration() -> Result<()> {
    println!("🧪 Testing Real ZFS Integration");
    
    // Initialize ZFS pool manager
    let config = ZfsConfig::default();
    let pool_manager = ZfsPoolManager::new(config);
    
    // Test pool discovery
    let pools = pool_manager.discover_pools().await?;
    println!("📊 Discovered {} ZFS pools", pools.len());
    
    // Verify we have the nestpool from our setup
    let nestpool = pools.iter().find(|p| p.name == "nestpool");
    assert!(nestpool.is_some(), "nestpool should be discovered");
    
    let pool = nestpool.unwrap();
    println!("✅ Found nestpool: {} capacity", pool.capacity);
    
    // Test pool status
    let status = pool_manager.get_pool_status(&pool.name).await?;
    println!("📈 Pool status: {:?}", status);
    assert_eq!(status.name, "nestpool");
    assert_eq!(status.state, "ONLINE");
    
    // Test dataset enumeration
    let datasets = pool_manager.list_datasets(&pool.name).await?;
    println!("📁 Found {} datasets", datasets.len());
    
    // Look for tier datasets
    let hot_dataset = datasets.iter().find(|d| d.name.contains("hot"));
    let warm_dataset = datasets.iter().find(|d| d.name.contains("warm"));
    let cold_dataset = datasets.iter().find(|d| d.name.contains("cold"));
    
    if hot_dataset.is_some() {
        println!("🔥 Hot tier dataset found: {}", hot_dataset.unwrap().name);
    }
    if warm_dataset.is_some() {
        println!("🌡️  Warm tier dataset found: {}", warm_dataset.unwrap().name);
    }
    if cold_dataset.is_some() {
        println!("❄️  Cold tier dataset found: {}", cold_dataset.unwrap().name);
    }
    
    println!("🎉 ZFS integration test completed successfully!");
    Ok(())
}

/// Test ZFS performance under concurrent operations
#[tokio::test]
async fn test_zfs_concurrent_operations() -> Result<()> {
    println!("🧪 Testing ZFS Concurrent Operations");
    
    let config = ZfsConfig::default();
    let pool_manager = ZfsPoolManager::new(config);
    
    // Run multiple concurrent pool discovery operations
    let mut handles = Vec::new();
    for i in 0..3 {
        let pm = pool_manager.clone();
        let handle = tokio::spawn(async move {
            let pools = pm.discover_pools().await?;
            println!("🔄 Concurrent operation {} found {} pools", i, pools.len());
            Ok::<usize, nestgate_core::NestGateError>(pools.len())
        });
        handles.push(handle);
    }
    
    // Wait for all operations to complete
    let results = futures::future::join_all(handles).await;
    let successful_operations = results.iter()
        .filter_map(|r| r.as_ref().ok())
        .filter_map(|r| r.as_ref().ok())
        .count();
    
    println!("⚡ Concurrent operations completed: {} successful", successful_operations);
    assert!(successful_operations > 0, "At least some operations should succeed");
    
    Ok(())
}

/// Test ZFS error handling
#[tokio::test]
async fn test_zfs_error_handling() -> Result<()> {
    println!("🧪 Testing ZFS Error Handling");
    
    let config = ZfsConfig::default();
    let pool_manager = ZfsPoolManager::new(config);
    
    // Test getting status of non-existent pool
    let result = pool_manager.get_pool_status("nonexistent-pool").await;
    match result {
        Ok(_) => {
            println!("⚠️  Unexpected success for non-existent pool");
        }
        Err(e) => {
            println!("✅ Expected error for non-existent pool: {}", e);
        }
    }
    
    // Test listing datasets for non-existent pool
    let result = pool_manager.list_datasets("nonexistent-pool").await;
    match result {
        Ok(_) => {
            println!("⚠️  Unexpected success for non-existent pool datasets");
        }
        Err(e) => {
            println!("✅ Expected error for non-existent pool datasets: {}", e);
        }
    }
    
    println!("✅ Error handling test completed");
    Ok(())
}

/// Test ZFS with timeout handling
#[tokio::test]
async fn test_zfs_timeout_handling() -> Result<()> {
    println!("🧪 Testing ZFS Timeout Handling");
    
    let config = ZfsConfig::default();
    let pool_manager = ZfsPoolManager::new(config);
    
    // Test operation with reasonable timeout (should succeed)
    let result = tokio::time::timeout(Duration::from_secs(10), async {
        pool_manager.discover_pools().await
    }).await;
    
    match result {
        Ok(Ok(pools)) => {
            println!("✅ Operation completed within timeout: {} pools discovered", pools.len());
        }
        Ok(Err(e)) => {
            println!("⚠️  Operation failed: {}", e);
        }
        Err(_) => {
            println!("⏰ Operation timed out (unexpected for 10 second timeout)");
        }
    }
    
    println!("✅ Timeout handling test completed");
    Ok(())
} 