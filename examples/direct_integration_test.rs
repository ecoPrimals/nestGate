//! Direct Integration Test
//!
//! A simple test to demonstrate integration with direct println output

use nestgate_core::{get_4kb_buffer, get_or_create_uuid, global_cache_statistics};
use nestgate_zfs::advanced_zfs_optimization::{
    AdvancedZfsOptimizer, OptimizerConfig, Pool, PoolStats, ZfsOperations,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Direct Integration Test");
    println!("{}", "=".repeat(50));

    // Test performance caching
    println!("📊 Testing Performance Caching...");
    let start = std::time::Instant::now();
    for i in 0..1000 {
        let _uuid = get_or_create_uuid(&format!("test-{}", i % 10));
    }
    let duration = start.elapsed();
    println!("  UUID Cache: 1000 operations in {duration:?}");

    let start = std::time::Instant::now();
    for _i in 0..100 {
        let _buffer = get_4kb_buffer();
    }
    let memory_duration = start.elapsed();
    println!("  Memory Pool: 100 allocations in {memory_duration:?}");

    let stats = global_cache_statistics();
    println!(
        "  Cache Stats: {} hits, {:.1}% ratio",
        stats.cache_hits,
        stats.hit_ratio * 100.0
    );

    // Test ZFS optimizer
    println!("⚡ Testing Advanced ZFS Optimizer...");
    let zfs_ops = Arc::new(MockZfs::new());
    let config = OptimizerConfig {
        monitoring_interval: 5,
        forecasting_interval: 30,
        cache_adjustment_interval: 10,
        max_auto_optimizations_per_hour: 6,
        enable_predictive_analytics: true,
        enable_adaptive_caching: true,
    };

    let _optimizer = AdvancedZfsOptimizer::new(zfs_ops.clone(), config);
    println!("  Optimizer created successfully!");

    let pools = zfs_ops.list_pools().await?;
    println!("  Found {} pools", pools.len());

    if let Some(pool) = pools.first() {
        let stats = zfs_ops.get_pool_stats(&pool.name).await?;
        println!(
            "  Pool '{}': {:.1}% ARC hit ratio",
            pool.name,
            stats.arc_hit_ratio * 100.0
        );
    }

    println!("✅ Direct Integration Test completed successfully!");
    println!("🎉 All systems are operational!");

    Ok(())
}

#[derive(Debug)]
pub struct MockZfs {
    pools: Vec<Pool>,
}

impl Default for MockZfs {
    fn default() -> Self {
        Self::new()
    }
}

impl MockZfs {
    pub fn new() -> Self {
        Self {
            pools: vec![Pool {
                name: "tank".to_string(),
                state: "ONLINE".to_string(),
                size: 10_000_000_000_000,
                allocated: 6_000_000_000_000,
                free: 4_000_000_000_000,
                fragmentation: Some(15),
                capacity: Some(60),
                health: "HEALTHY".to_string(),
                altroot: None,
            }],
        }
    }
}

#[async_trait::async_trait]
impl ZfsOperations for MockZfs {
    async fn list_pools(&self) -> anyhow::Result<Vec<Pool>> {
        Ok(self.pools.clone())
    }

    async fn get_pool_stats(&self, _pool_name: &str) -> anyhow::Result<PoolStats> {
        Ok(PoolStats {
            read_ops: 1000,
            write_ops: 500,
            read_bandwidth: 1024000,
            write_bandwidth: 512000,
            arc_hit_ratio: 0.95,
            l2arc_hit_ratio: 0.80,
            l2arc_enabled: true,
            fragmentation: 15.0,
            free_space: 4_000_000_000_000,
            used_space: 6_000_000_000_000,
        })
    }

    async fn list_datasets(&self, _pool_name: &str) -> anyhow::Result<Vec<String>> {
        Ok(vec!["data".to_string()])
    }

    async fn create_pool(&self, name: &str, _devices: &[String]) -> anyhow::Result<Pool> {
        Ok(Pool {
            name: name.to_string(),
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
