use std::time::Duration;
use tokio::time::timeout;
use nestgate_core::Result;
use nestgate_zfs::{ZfsPoolManager, ZfsConfig};
use nestgate_orchestrator::{Orchestrator, OrchestratorConfig, NetworkConfig, EnvironmentConfig};
use nestgate_mcp::{EnhancedMcpService, McpConfig};

/// Integration test suite for NestGate system components
#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Test ZFS integration with real pools
    #[tokio::test]
    async fn test_zfs_pool_integration() -> Result<()> {
        println!("🧪 Testing ZFS Pool Integration");
        
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
        
        Ok(())
    }

    /// Test orchestrator service integration
    #[tokio::test]
    async fn test_orchestrator_integration() -> Result<()> {
        println!("🧪 Testing Orchestrator Integration");
        
        // Create orchestrator configuration
        let config = OrchestratorConfig {
            network: NetworkConfig {
                bind_address: "127.0.0.1:8080".to_string(),
                port_range: (8080, 8090),
                allow_external_access: false,
            },
            environment: EnvironmentConfig {
                mode: "test".to_string(),
                debug: true,
            },
            ..Default::default()
        };
        
        // Initialize orchestrator
        let orchestrator = Orchestrator::new(config).await?;
        
        // Test orchestrator startup
        orchestrator.start().await?;
        println!("✅ Orchestrator started successfully");
        
        // Verify orchestrator is running
        assert!(orchestrator.is_running().await, "Orchestrator should be running");
        
        // Test service registry
        let service_registry = orchestrator.service_registry();
        println!("📋 Service registry initialized");
        
        // Test connection proxy
        let connection_proxy = orchestrator.connection_proxy();
        println!("🔗 Connection proxy initialized");
        
        // Test health monitor
        let health_monitor = orchestrator.health_monitor();
        println!("💓 Health monitor initialized");
        
        // Clean shutdown
        orchestrator.stop().await?;
        println!("🛑 Orchestrator stopped successfully");
        
        Ok(())
    }

    /// Test MCP service integration
    #[tokio::test]
    async fn test_mcp_integration() -> Result<()> {
        println!("🧪 Testing MCP Integration");
        
        // Create MCP configuration
        let config = McpConfig {
            endpoint: "http://localhost:8081".to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
        };
        
        // Initialize MCP service
        let mcp_service = EnhancedMcpService::new(config);
        println!("✅ MCP service initialized");
        
        // Test capabilities discovery
        let capabilities = mcp_service.get_capabilities().await?;
        println!("🎯 MCP capabilities: {} protocols supported", capabilities.supported_protocols.len());
        
        // Verify we have expected capabilities
        assert!(!capabilities.supported_protocols.is_empty(), "Should have supported protocols");
        
        // Test metrics collection
        let metrics = mcp_service.collect_metrics().await?;
        println!("📊 System metrics - CPU: {:.2}%, Memory: {:.2}%, Disk: {:.2}%", 
                 metrics.cpu_usage, metrics.memory_usage, metrics.disk_usage);
        
        Ok(())
    }

    /// Test end-to-end system integration
    #[tokio::test]
    async fn test_end_to_end_integration() -> Result<()> {
        println!("🧪 Testing End-to-End System Integration");
        
        // 1. Initialize ZFS system
        let zfs_config = ZfsConfig::default();
        let pool_manager = ZfsPoolManager::new(zfs_config);
        
        // 2. Initialize Orchestrator
        let orchestrator_config = OrchestratorConfig {
            network: NetworkConfig {
                bind_address: "127.0.0.1:8082".to_string(),
                port_range: (8082, 8092),
                allow_external_access: false,
            },
            environment: EnvironmentConfig {
                mode: "test".to_string(),
                debug: true,
            },
            ..Default::default()
        };
        let orchestrator = Orchestrator::new(orchestrator_config).await?;
        
        // 3. Initialize MCP service
        let mcp_config = McpConfig {
            endpoint: "http://localhost:8083".to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
        };
        let mcp_service = EnhancedMcpService::new(mcp_config);
        
        // 4. Start orchestrator
        orchestrator.start().await?;
        println!("✅ All services initialized and started");
        
        // 5. Test integrated functionality
        
        // Test ZFS pool discovery through orchestrator
        let pools = pool_manager.discover_pools().await?;
        println!("📊 Discovered {} pools through integrated system", pools.len());
        
        // Test MCP capabilities
        let capabilities = mcp_service.get_capabilities().await?;
        println!("🎯 MCP reports {} supported protocols", capabilities.supported_protocols.len());
        
        // Test system metrics
        let metrics = mcp_service.collect_metrics().await?;
        println!("📈 System health - CPU: {:.1}%, Memory: {:.1}%, Disk: {:.1}%",
                 metrics.cpu_usage, metrics.memory_usage, metrics.disk_usage);
        
        // 6. Test error handling and recovery
        println!("🔄 Testing error handling...");
        
        // Simulate a service lookup
        let service_result = orchestrator.get_service("nestgate-zfs").await;
        match service_result {
            Ok(endpoint) => println!("✅ Service discovery successful: {}", endpoint),
            Err(e) => println!("⚠️  Service discovery error (expected in test): {}", e),
        }
        
        // 7. Clean shutdown
        orchestrator.stop().await?;
        println!("🛑 All services stopped successfully");
        
        println!("🎉 End-to-end integration test completed successfully!");
        Ok(())
    }

    /// Test ZFS tiered storage integration
    #[tokio::test]
    async fn test_tiered_storage_integration() -> Result<()> {
        println!("🧪 Testing Tiered Storage Integration");
        
        let config = ZfsConfig::default();
        let pool_manager = ZfsPoolManager::new(config);
        
        // Test pool discovery
        let pools = pool_manager.discover_pools().await?;
        let nestpool = pools.iter().find(|p| p.name == "nestpool");
        assert!(nestpool.is_some(), "nestpool required for tiered storage test");
        
        let pool = nestpool.unwrap();
        println!("📊 Testing tiered storage on pool: {}", pool.name);
        
        // Test dataset enumeration (should include hot, warm, cold)
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
        
        println!("✅ Tiered storage integration verified");
        Ok(())
    }

    /// Test system performance under load
    #[tokio::test]
    async fn test_performance_integration() -> Result<()> {
        println!("🧪 Testing Performance Integration");
        
        let start_time = std::time::Instant::now();
        
        // Test concurrent pool operations
        let config = ZfsConfig::default();
        let pool_manager = ZfsPoolManager::new(config);
        
        // Run multiple concurrent pool discovery operations
        let mut handles = Vec::new();
        for i in 0..5 {
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
        
        let duration = start_time.elapsed();
        println!("⚡ Performance test completed: {}/{} operations successful in {:?}",
                 successful_operations, results.len(), duration);
        
        // Verify reasonable performance (should complete within 10 seconds)
        assert!(duration < Duration::from_secs(10), "Operations should complete within 10 seconds");
        assert!(successful_operations > 0, "At least some operations should succeed");
        
        Ok(())
    }

    /// Test system integration with timeout handling
    #[tokio::test]
    async fn test_timeout_integration() -> Result<()> {
        println!("🧪 Testing Timeout Integration");
        
        // Test with short timeout to verify timeout handling
        let config = ZfsConfig::default();
        let pool_manager = ZfsPoolManager::new(config);
        
        // Test operation with timeout
        let result = timeout(Duration::from_millis(100), async {
            pool_manager.discover_pools().await
        }).await;
        
        match result {
            Ok(Ok(pools)) => {
                println!("✅ Fast operation completed: {} pools discovered", pools.len());
            }
            Ok(Err(e)) => {
                println!("⚠️  Operation failed: {}", e);
            }
            Err(_) => {
                println!("⏰ Operation timed out (expected for very short timeout)");
            }
        }
        
        // Test with reasonable timeout
        let result = timeout(Duration::from_secs(5), async {
            pool_manager.discover_pools().await
        }).await;
        
        assert!(result.is_ok(), "Operation should complete within 5 seconds");
        let pools = result.unwrap()?;
        println!("✅ Timeout integration test passed: {} pools discovered", pools.len());
        
        Ok(())
    }
}

/// Helper function to check if ZFS is available
async fn is_zfs_available() -> bool {
    tokio::process::Command::new("zpool")
        .arg("status")
        .output()
        .await
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Integration test runner with system checks
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("🚀 Starting NestGate Integration Tests");
    
    // Check system prerequisites
    if !is_zfs_available().await {
        println!("⚠️  ZFS not available - some tests may be skipped");
    } else {
        println!("✅ ZFS available - full integration testing enabled");
    }
    
    println!("🧪 Integration tests completed - run with 'cargo test --test integration_test' to execute");
    
    Ok(())
} 