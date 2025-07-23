use tracing::{error, info};
// Quick Integration Test
//
// A simple test to demonstrate integration between Advanced ZFS Optimization,
// Real-time monitoring, and performance caching systems.

use nestgate_core::{get_4kb_buffer, get_or_create_uuid, global_cache_statistics};
use nestgate_zfs::advanced_zfs_optimization::{
    AdvancedZfsOptimizer, OptimizerConfig, Pool, PoolStats, ZfsOperations,
};
use std::sync::Arc;
// Removed unused tracing import

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("🚀 Starting Quick Integration Test");
    info!("{}", "=".repeat(50));

    // Test 1: Performance Caching System
    info!("📊 Testing Performance Caching System...");
    test_performance_caching().await?;

    // Test 2: Advanced ZFS Optimizer Integration
    info!("⚡ Testing Advanced ZFS Optimizer Integration...");
    test_zfs_optimizer().await?;

    // Test 3: Combined System Performance
    info!("🎯 Testing Combined System Performance...");
    test_combined_performance().await?;

    info!("✅ Quick Integration Test completed successfully!");
    info!("🎉 All systems are operational and integrated!");

    Ok(())
}

async fn test_performance_caching() -> std::result::Result<(), Box<dyn std::error::Error>> {
    info!("  Testing UUID caching performance...");

    // Test UUID caching
    let start = std::time::Instant::now();
    for i in 0..1000 {
        let service = format!("service-{}", i % 10);
        let _uuid = get_or_create_uuid(&service);
    }
    let duration = start.elapsed();

    info!(
        "  🔄 UUID Cache: 1000 operations in {:.2}ms",
        duration.as_millis()
    );

    // Test memory pooling
    let start = std::time::Instant::now();
    for _i in 0..100 {
        let _buffer = get_4kb_buffer();
    }
    let memory_duration = start.elapsed();

    info!(
        "  💾 Memory Pool: 100 allocations in {:.2}ms",
        memory_duration.as_millis()
    );

    // Show cache statistics
    let stats = global_cache_statistics();
    info!(
        "  📈 Cache Stats: {} hits, {:.1}% hit ratio",
        stats.cache_hits,
        stats.hit_ratio * 100.0
    );

    Ok(())
}

async fn test_zfs_optimizer() -> std::result::Result<(), Box<dyn std::error::Error>> {
    info!("  Creating Advanced ZFS Optimizer...");

    // Create mock ZFS operations
    let zfs_ops = Arc::new(MockZfsOps::new());

    // Create optimizer configuration
    let config = OptimizerConfig {
        monitoring_interval: 5,
        forecasting_interval: 30,
        cache_adjustment_interval: 10,
        max_auto_optimizations_per_hour: 6,
        enable_predictive_analytics: true,
        enable_adaptive_caching: true,
    };

    // Create optimizer
    let optimizer = Arc::new(AdvancedZfsOptimizer::new(zfs_ops.clone(), config));

    info!("  🔧 Optimizer created successfully!");

    // Test ZFS operations
    let pools = zfs_ops.list_pools().await?;
    info!("  📀 Found {} ZFS pools", pools.len());

    if let Some(pool) = pools.first() {
        let stats = zfs_ops.get_pool_stats(&pool.name).await?;
        info!(
            "  📊 Pool '{}': {:.1}% ARC hit ratio, {} read ops",
            pool.name,
            stats.arc_hit_ratio * 100.0,
            stats.read_ops
        );
    }

    Ok(())
}

async fn test_combined_performance() -> std::result::Result<(), Box<dyn std::error::Error>> {
    info!("  Running combined system performance test...");

    // Combined workload simulation
    let start = std::time::Instant::now();

    for i in 0..100 {
        // UUID operations
        let _uuid = get_or_create_uuid(&format!("combined-test-{}", i % 5));

        // Memory operations
        let _buffer = get_4kb_buffer();

        // Small delay to simulate I/O
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    }

    let total_duration = start.elapsed();

    info!(
        "  ⚡ Combined workload: 100 iterations in {:.2}ms",
        total_duration.as_millis()
    );

    // Final system state
    let final_stats = global_cache_statistics();
    info!(
        "  🏁 Final Cache State: {} entries, {:.1}% hit ratio",
        final_stats.cache_size,
        final_stats.hit_ratio * 100.0
    );

    Ok(())
}

/// Simple mock ZFS operations for testing
#[derive(Debug)]
pub struct MockZfsOps {
    pools: Vec<Pool>,
}

impl MockZfsOps {
    pub fn new() -> Self {
        let pools = vec![
            Pool {
                name: "tank".to_string(),
                state: "ONLINE".to_string(),
                size: 10_000_000_000_000,
                allocated: 6_000_000_000_000,
                free: 4_000_000_000_000,
                fragmentation: Some(15),
                capacity: Some(60),
                health: "HEALTHY".to_string(),
                altroot: None,
            },
            Pool {
                name: "backup".to_string(),
                state: "ONLINE".to_string(),
                size: 2_000_000_000_000,
                allocated: 800_000_000_000,
                free: 1_200_000_000_000,
                fragmentation: Some(8),
                capacity: Some(40),
                health: "HEALTHY".to_string(),
                altroot: Some("/backup".to_string()),
            },
        ];

        Self { pools }
    }
}

#[async_trait::async_trait]
impl ZfsOperations for MockZfsOps {
    async fn list_pools(&self) -> anyhow::Result<Vec<Pool>> {
        Ok(self.pools.clone())
    }

    async fn get_pool_stats(&self, pool_name: &str) -> anyhow::Result<PoolStats> {
        let base_ops = if pool_name == "tank" { 1000 } else { 200 };
        Ok(PoolStats {
            read_ops: base_ops + rand::random::<u64>() % 100,
            write_ops: base_ops / 2 + rand::random::<u64>() % 50,
            read_bandwidth: (base_ops * 1024),
            write_bandwidth: (base_ops * 512),
            arc_hit_ratio: 0.92 + (rand::random::<u64>() % 5) as f64 / 100.0,
            l2arc_hit_ratio: 0.78 + (rand::random::<u64>() % 10) as f64 / 100.0,
            l2arc_enabled: true,
            fragmentation: 12.0 + (rand::random::<u64>() % 8) as f64,
            free_space: self
                .pools
                .iter()
                .find(|p| p.name == pool_name)
                .map(|p| p.free)
                .unwrap_or(0),
            used_space: self
                .pools
                .iter()
                .find(|p| p.name == pool_name)
                .map(|p| p.allocated)
                .unwrap_or(0),
        })
    }

    async fn list_datasets(&self, _pool_name: &str) -> anyhow::Result<Vec<String>> {
        Ok(vec!["data".to_string(), "logs".to_string()])
    }

    async fn create_pool(&self, _name: &str, _devices: &[String]) -> anyhow::Result<Pool> {
        Ok(Pool {
            name: "test".to_string(),
            state: "ONLINE".to_string(),
            size: 1_000_000_000_000,
            allocated: 0,
            free: 1_000_000_000_000,
            fragmentation: Some(0),
            capacity: Some(0),
            health: "HEALTHY".to_string(),
            altroot: None,
        })
    }

    async fn destroy_pool(&self, _name: &str) -> anyhow::Result<()> {
        Ok(())
    }

    async fn create_dataset(&self, _pool_name: &str, _dataset_name: &str) -> anyhow::Result<()> {
        Ok(())
    }

    async fn destroy_dataset(&self, _pool_name: &str, _dataset_name: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
