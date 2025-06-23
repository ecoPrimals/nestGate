/*!
 * NestGate Main Binary
 * 
 * Single entry point for the entire NestGate system via orchestrator
 * Services are handed to the orchestrator for proper lifecycle management
 */

use std::sync::Arc;
use tokio::signal;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use anyhow::Result;
use std::env;

// Use the orchestrator crate with service management
use nestgate_orchestrator::{
    Orchestrator, OrchestratorConfig,
    ZfsService, ApiService, NetworkService, McpService, TowerFederationService
};
use nestgate_core::config::{NetworkConfig, EnvironmentConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting NestGate Orchestrator");

    // Create orchestrator configuration
    let config = OrchestratorConfig::default();

    // Create orchestrator
    let orchestrator = Orchestrator::new(config).await?;

    // Create and start services
    let zfs_service = Box::new(ZfsService::new());
    let api_service = Box::new(ApiService::new());
    let network_service = Box::new(NetworkService::new());
    let mcp_service = Box::new(McpService::new());
    let federation_service = Box::new(TowerFederationService::new("main-tower".to_string(), None));

    // Start services
    orchestrator.start_service(zfs_service).await?;
    orchestrator.start_service(api_service).await?;
    orchestrator.start_service(network_service).await?;
    orchestrator.start_service(mcp_service).await?;
    orchestrator.start_service(federation_service).await?;

    // Start the orchestrator
    orchestrator.start().await?;

    info!("NestGate Orchestrator started successfully");

    // Wait for shutdown signal
    signal::ctrl_c().await?;
    info!("Shutdown signal received");

    // Graceful shutdown
    orchestrator.shutdown().await?;
    info!("NestGate Orchestrator shutdown complete");

    Ok(())
}

fn print_help() {
    println!("NestGate v2 Orchestrator");
    println!();
    println!("USAGE:");
    println!("    nestgate [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    --mock-tower    Start a mock tower for federation testing");
    println!("    -h, --help      Print this help message");
    println!();
    println!("EXAMPLES:");
    println!("    nestgate                 # Start the main orchestrator");
    println!("    nestgate --mock-tower    # Start a mock tower for testing");
    println!();
    println!("ENVIRONMENT VARIABLES:");
    println!("    RUST_LOG=debug          # Enable debug logging");
    println!("    RUST_LOG=info           # Enable info logging (default)");
} 