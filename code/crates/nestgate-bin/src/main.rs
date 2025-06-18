/*!
 * NestGate Main Binary
 * 
 * This is a thin wrapper around the port manager crate
 */

use std::sync::Arc;
use tokio::signal;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use anyhow::Result;

// Use the orchestrator crate with correct imports
use nestgate_orchestrator::{Orchestrator, OrchestratorConfig};
use nestgate_network::NetworkApi;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nestgate=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting NestGate system...");
    
    // Create orchestrator configuration
    let orchestrator_config = OrchestratorConfig {
        bind_address: "0.0.0.0:8080".to_string(),
        mcp_config: None,
        federation_config: None,
        load_balancer_config: Default::default(),
        health_config: Default::default(),
        metrics_config: Default::default(),
    };

    // Create and start the orchestrator
    let orchestrator = Arc::new(Orchestrator::new(orchestrator_config).await?);
    
    // Start the orchestrator
    orchestrator.start().await?;
    info!("Orchestrator started successfully");

    // Create and start the network API
    let network_api = NetworkApi::new();
    let api_router = network_api.create_router();

    // Start the API server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await?;
    info!("Network API server listening on 0.0.0.0:8081");
    
    let api_server = axum::serve(listener, api_router);
    
    // Handle shutdown gracefully
    tokio::select! {
        result = api_server => {
            if let Err(e) = result {
                error!("API server error: {}", e);
            }
        }
        _ = signal::ctrl_c() => {
            info!("Received shutdown signal");
        }
    }

    // Stop the orchestrator
    orchestrator.shutdown().await?;
    info!("NestGate system stopped");

    Ok(())
} 