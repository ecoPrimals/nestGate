//! Development server for NestGate API
//!
//! This example demonstrates how to set up and run the NestGate API server
//! with full ZFS integration for development and testing purposes.

use std::sync::Arc;
use tracing::{info, warn, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use nestgate_api::{Config, serve_with_zfs};
use nestgate_zfs::{ZfsManager, config::ZfsConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nestgate_api=debug,nestgate_zfs=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting NestGate API Development Server");

    // Create ZFS configuration
    let zfs_config = ZfsConfig::default();

    // Initialize ZFS manager
    info!("Initializing ZFS manager");
    let zfs_manager = match ZfsManager::new(zfs_config).await {
        Ok(manager) => {
            info!("ZFS manager initialized successfully");
            Arc::new(manager)
        }
        Err(e) => {
            error!("Failed to initialize ZFS manager: {}", e);
            warn!("Continuing without ZFS integration");
            return Err(e.into());
        }
    };

    // Create API configuration
    let api_config = Config {
        bind_addr: "0.0.0.0:3000".to_string(),
        cors: None, // Will use permissive CORS
        enable_zfs_api: true,
        request_timeout: 30,
        max_body_size: 16 * 1024 * 1024, // 16MB
    };

    info!("API server configuration:");
    info!("  Bind address: {}", api_config.bind_addr);
    info!("  ZFS API enabled: {}", api_config.enable_zfs_api);
    info!("  Request timeout: {}s", api_config.request_timeout);
    info!("  Max body size: {}MB", api_config.max_body_size / (1024 * 1024));

    // Print available endpoints
    print_available_endpoints();

    // Start the server
    info!("Starting API server...");
    if let Err(e) = serve_with_zfs(api_config, zfs_manager).await {
        error!("API server failed: {}", e);
        return Err(e);
    }

    Ok(())
}

fn print_available_endpoints() {
    info!("Available API endpoints:");
    info!("  Health & Status:");
    info!("    GET  /health                           - Basic health check");
    info!("    GET  /api/v1/status                    - System status");
    info!("    GET  /api/v1/zfs/health                - ZFS health status");
    info!("    GET  /api/v1/zfs/status                - ZFS system status");
    
    info!("  Pool Management:");
    info!("    GET  /api/v1/zfs/pools                 - List all pools");
    info!("    POST /api/v1/zfs/pools                 - Create new pool");
    info!("    GET  /api/v1/zfs/pools/{{name}}          - Get pool info");
    info!("    DELETE /api/v1/zfs/pools/{{name}}       - Destroy pool");
    info!("    GET  /api/v1/zfs/pools/{{name}}/status   - Get pool status");
    info!("    POST /api/v1/zfs/pools/{{name}}/scrub    - Start pool scrub");
    
    info!("  Dataset Management:");
    info!("    GET  /api/v1/zfs/datasets              - List all datasets");
    info!("    POST /api/v1/zfs/datasets              - Create new dataset");
    info!("    GET  /api/v1/zfs/datasets/{{name}}       - Get dataset info");
    info!("    DELETE /api/v1/zfs/datasets/{{name}}    - Destroy dataset");
    info!("    GET  /api/v1/zfs/datasets/{{name}}/properties - Get dataset properties");
    info!("    PUT  /api/v1/zfs/datasets/{{name}}/properties - Set dataset properties");
    
    info!("  Snapshot Management:");
    info!("    GET  /api/v1/zfs/datasets/{{name}}/snapshots        - List snapshots");
    info!("    POST /api/v1/zfs/datasets/{{name}}/snapshots        - Create snapshot");
    info!("    DELETE /api/v1/zfs/datasets/{{name}}/snapshots/{{snap}} - Delete snapshot");
    
    info!("  AI & Optimization:");
    info!("    POST /api/v1/zfs/ai/tier-prediction    - Get tier prediction");
    info!("    GET  /api/v1/zfs/optimization/analytics - Get performance analytics");
    info!("    POST /api/v1/zfs/optimization/trigger  - Trigger optimization");

    info!("");
    info!("Example requests:");
    info!("  curl http://localhost:3000/health");
    info!("  curl http://localhost:3000/api/v1/zfs/pools");
    info!("  curl -X POST http://localhost:3000/api/v1/zfs/pools \\");
    info!("    -H 'Content-Type: application/json' \\");
    info!("    -d '{{\"name\":\"test_pool\",\"devices\":[\"/dev/loop0\"]}}'");
    info!("");
    info!("Web interface (if available): http://localhost:3000/");
} 