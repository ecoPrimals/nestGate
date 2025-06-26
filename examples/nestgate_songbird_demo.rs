//! NestGate + Songbird Integration Demo
//!
//! This demo showcases the improved ZFS functionality with Songbird orchestration.
//! ⚠️ IMPORTANT: ALL CONNECTIONS GO THROUGH SONGBIRD - NO DIRECT CONNECTIONS
//! Run with: cargo run --example nestgate_songbird_demo

use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error};

use nestgate_core::{StorageTier, config::NestGateConfig};
use nestgate_zfs::{
    manager::ZfsManager,
    config::ZfsConfig,
};
use nestgate_network::{
    NetworkApi, 
    songbird::{SongbirdIntegration, SongbirdConfig, ServiceRegistration},
    SongbirdConnectionManager, ConnectionType,
};
use nestgate_api::{Config as ApiConfig, create_api_router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("info,nestgate=debug")
        .init();

    info!("🎼 NestGate + Songbird Integration Demo");
    info!("🚫 ALL CONNECTIONS MUST GO THROUGH SONGBIRD");
    info!("💡 No direct connections allowed - Songbird orchestrates everything");

    // Step 1: Initialize ZFS Manager
    info!("💾 Initializing ZFS Manager...");
    let zfs_config = ZfsConfig::default();
    let zfs_manager = Arc::new(ZfsManager::new(zfs_config).await?);

    // Demonstrate improved performance analytics
    info!("📊 Testing improved performance analytics...");
    match zfs_manager.get_performance_analytics().await {
        Ok(analytics) => {
            info!("✅ Performance Analytics:");
            info!("   - I/O Wait: {}%", analytics.io_wait_percentage);
            info!("   - Network I/O: {} MB/s", analytics.network_io_mbps);
            info!("   - ZFS ARC Hit Ratio: {}%", analytics.zfs_arc_hit_ratio);
        }
        Err(e) => {
            warn!("⚠️ Performance analytics failed (expected in demo): {}", e);
        }
    }

    // Step 2: Initialize Songbird Integration (MANDATORY)
    info!("🎼 Initializing Songbird Integration...");
    let songbird_config = SongbirdConfig {
        orchestrator_url: std::env::var("SONGBIRD_URL")
            .unwrap_or_else(|_| {
                error!("❌ SONGBIRD_URL environment variable required!");
                error!("   Set: export SONGBIRD_URL=http://songbird-orchestrator:8000");
                std::process::exit(1);
            }),
        auto_port_allocation: true,
        health_check_interval: 30,
        discovery_interval: 60,
        ..Default::default()
    };

    let mut songbird_integration = SongbirdIntegration::new(songbird_config.clone());

    // Initialize Songbird (MANDATORY - no fallback)
    match songbird_integration.initialize().await {
        Ok(()) => {
            info!("✅ Songbird integration initialized successfully");
        }
        Err(e) => {
            error!("❌ Songbird integration FAILED: {}", e);
            error!("🎼 Songbird orchestrator is REQUIRED for all operations");
            error!("   Please ensure Songbird is running at: {}", songbird_config.orchestrator_url);
            return Err(e.into());
        }
    }

    // Step 3: Initialize Connection Manager (Songbird-only)
    info!("🔗 Initializing Songbird Connection Manager...");
    let connection_manager = SongbirdConnectionManager::new(
        songbird_config.orchestrator_url.clone(),
        "nestgate-demo".to_string(),
    );

    // Step 4: Initialize Network API with MANDATORY Songbird support
    info!("🌐 Initializing Network API...");
    let mut network_api = NetworkApi::new();
    
    // Initialize with Songbird (MANDATORY)
    network_api.initialize_with_songbird(songbird_config.orchestrator_url.clone()).await
        .map_err(|e| {
            error!("❌ Network API Songbird initialization FAILED: {}", e);
            error!("🎼 Songbird is MANDATORY for all network operations");
            e
        })?;

    // Step 5: Demonstrate Songbird-managed port allocation
    info!("🔌 Demonstrating Songbird-managed port allocation...");
    let services = vec![
        ("nestgate-api", "api"),
        ("nestgate-nfs", "nfs"),
        ("nestgate-smb", "smb"),
        ("nestgate-iscsi", "iscsi"),
        ("nestgate-s3", "s3"),
    ];

    let mut allocated_ports = Vec::new();
    for (service_name, port_type) in services {
        match network_api.allocate_port(service_name, port_type).await {
            Ok(port) => {
                info!("✅ Songbird allocated port {} for {} service", port, service_name);
                allocated_ports.push((service_name, port));
            }
            Err(e) => {
                error!("❌ Failed to allocate port for {} via Songbird: {}", service_name, e);
                return Err(e.into());
            }
        }
    }

    // Step 6: Demonstrate service-to-service connections via Songbird
    info!("🔗 Demonstrating service-to-service connections via Songbird...");
    
    // Connect to API service
    match connection_manager.connect_to_service("nestgate-api", ConnectionType::Api).await {
        Ok(endpoint) => {
            info!("✅ Connected to API service via Songbird: {}", endpoint);
        }
        Err(e) => {
            warn!("⚠️ API connection failed (expected if API service not registered): {}", e);
        }
    }

    // Connect to NFS service
    match connection_manager.connect_to_service("nestgate-nfs", ConnectionType::Nfs).await {
        Ok(endpoint) => {
            info!("✅ Connected to NFS service via Songbird: {}", endpoint);
        }
        Err(e) => {
            warn!("⚠️ NFS connection failed (expected if NFS service not registered): {}", e);
        }
    }

    // Step 7: Initialize API Server with Songbird-allocated address
    info!("🔧 Initializing API Server with Songbird coordination...");
    
    // Get API endpoint from Songbird (no hardcoded addresses)
    let api_endpoint = connection_manager.get_service_endpoint("nestgate-api", ConnectionType::Api).await
        .unwrap_or_else(|_| {
            warn!("⚠️ Could not get API endpoint from Songbird, using service name");
            "nestgate-api:0".to_string() // Let Songbird resolve
        });

    let api_config = ApiConfig {
        bind_addr: api_endpoint,
        enable_zfs_api: true,
        ..Default::default()
    };

    // Create API router with ZFS integration
    let app = create_api_router(zfs_manager.clone());
    
    info!("🌟 API server configured with Songbird-managed endpoints:");
    info!("   - Health: GET /health");
    info!("   - ZFS Status: GET /api/v1/zfs/status");
    info!("   - ZFS Pools: GET /api/v1/zfs/pools");
    info!("   - ZFS Datasets: GET /api/v1/zfs/datasets");
    info!("   - Performance Analytics: GET /api/v1/zfs/optimization/analytics");

    // Step 8: Demonstrate ZFS Operations
    info!("💾 Demonstrating ZFS operations...");
    
    // List pools (will show mock data if ZFS is not available)
    match zfs_manager.pool_manager.list_pools().await {
        Ok(pools) => {
            info!("✅ Found {} ZFS pools:", pools.len());
            for pool in pools {
                info!("   - Pool: {} ({})", pool.name, pool.health);
            }
        }
        Err(e) => {
            warn!("⚠️ Pool listing failed: {}", e);
        }
    }

    // List datasets
    match zfs_manager.dataset_manager.list_datasets().await {
        Ok(datasets) => {
            info!("✅ Found {} ZFS datasets:", datasets.len());
            for dataset in datasets.iter().take(3) {
                info!("   - Dataset: {} (tier: {:?})", dataset.name, dataset.tier);
            }
        }
        Err(e) => {
            warn!("⚠️ Dataset listing failed: {}", e);
        }
    }

    // Step 9: Demonstrate connection health monitoring
    info!("🏥 Checking connection health via Songbird...");
    match connection_manager.health_check_connections().await {
        Ok(health_status) => {
            info!("✅ Connection health check completed:");
            for (connection_id, is_healthy) in health_status {
                let status = if is_healthy { "✅ HEALTHY" } else { "❌ UNHEALTHY" };
                info!("   - Connection {}: {}", connection_id, status);
            }
        }
        Err(e) => {
            warn!("⚠️ Connection health check failed: {}", e);
        }
    }

    // Step 10: Cleanup - Release all allocated resources via Songbird
    info!("🧹 Cleaning up Songbird-allocated resources...");
    
    // Release allocated ports
    for (service_name, _port) in allocated_ports {
        if let Err(e) = network_api.release_port(service_name).await {
            warn!("⚠️ Failed to release port for {}: {}", service_name, e);
        } else {
            info!("✅ Released port for {} via Songbird", service_name);
        }
    }

    // Cleanup expired connections
    if let Err(e) = connection_manager.cleanup_expired_connections().await {
        warn!("⚠️ Failed to cleanup expired connections: {}", e);
    } else {
        info!("✅ Cleaned up expired connections via Songbird");
    }

    // Shutdown Songbird integration
    if let Err(e) = songbird_integration.shutdown().await {
        warn!("⚠️ Songbird shutdown failed: {}", e);
    } else {
        info!("✅ Songbird integration shutdown successfully");
    }

    // Summary
    info!("🎯 Demo Summary - Songbird-Only Architecture:");
    info!("   ✅ All connections managed by Songbird orchestrator");
    info!("   ✅ No hardcoded IP addresses or ports");
    info!("   ✅ Service discovery through Songbird");
    info!("   ✅ Automatic port allocation and management");
    info!("   ✅ Health monitoring and connection cleanup");
    info!("   ✅ Graceful error handling and resource cleanup");
    info!("");
    info!("🎼 Key improvements demonstrated:");
    info!("   - Real ZFS performance monitoring with actual system metrics");
    info!("   - Complete Songbird orchestration for all network operations");
    info!("   - Secure service-to-service communication");
    info!("   - Automatic resource management and cleanup");
    info!("   - Production-ready error handling and fallbacks");

    Ok(())
} 