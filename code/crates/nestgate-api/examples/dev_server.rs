//! Development server for NestGate API
//!
//! This example demonstrates how to set up and run the NestGate API server
//! with full ZFS integration for development and testing purposes.

use nestgate_api::{serve_with_zfs, Config};
use nestgate_zfs::{config::ZfsConfig, ZfsManager};
use std::sync::Arc;
use tracing::{error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "nestgate_api=debug,nestgate_zfs=debug,tower_http=debug".into()
            }),
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
        bind_addr: std::env::var("NESTGATE_DEV_SERVER_BIND")
            .unwrap_or_else(|_| "0.0.0.0:3000".to_string()),
        enable_zfs_api: true,
        enable_sse: true,
        enable_websockets: true,
        max_request_size: 16 * 1024 * 1024, // 16MB
    };

    info!("API server configuration:");
    info!("  Bind address: {}", api_config.bind_addr);
    info!("  ZFS API enabled: {}", api_config.enable_zfs_api);
    info!("  SSE enabled: {}", api_config.enable_sse);
    info!("  WebSockets enabled: {}", api_config.enable_websockets);
    info!(
        "  Max request size: {}MB",
        api_config.max_request_size / (1024 * 1024)
    );

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
    let port = std::env::var("NESTGATE_DEV_SERVER_PORT").unwrap_or_else(|_| "3000".to_string());

    info!("Development server running successfully!");
    info!("Available endpoints:");
    info!("  curl http://localhost:{}/health", port);
    info!("  curl http://localhost:{}/api/v1/zfs/pools", port);
    info!(
        "  curl -X POST http://localhost:{}/api/v1/zfs/pools \\",
        port
    );
    info!("       -H 'Content-Type: application/json' \\");
    info!("       -d '{{\"name\":\"test-pool\",\"devices\":[\"/dev/loop0\"]}}'");
    info!("Web interface (if available): http://localhost:{}/", port);
}
