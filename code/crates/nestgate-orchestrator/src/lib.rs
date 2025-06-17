//! NestGate Orchestrator
//! 
//! Central connectivity hub and service orchestrator for NestGate NAS system.
//! ALL connectivity flows through the orchestrator.

pub mod orchestrator;
pub mod service_registry;
pub mod connection_proxy;
pub mod health_monitor;
pub mod mcp_federation;

// Re-export main types
pub use orchestrator::*;
pub use service_registry::*;
pub use connection_proxy::*;
pub use health_monitor::*;
pub use mcp_federation::*;

use nestgate_core::Result;

/// Configuration for the orchestrator
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OrchestratorConfig {
    /// Address to bind the orchestrator to
    pub bind_address: String,
    /// Federation mode (standalone, auto_detect, federated)
    pub federation_mode: FederationMode,
    /// Local services to manage
    pub local_services: Vec<String>,
    /// Health check interval in seconds
    pub health_check_interval: u64,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0:8080".to_string(),
            federation_mode: FederationMode::AutoDetect,
            local_services: vec![
                "nestgate-core".to_string(),
                "nestgate-zfs".to_string(),
                "nestgate-meta".to_string(),
            ],
            health_check_interval: 30,
        }
        }
    }
    
/// Federation mode for the orchestrator
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum FederationMode {
    /// Standalone mode - no MCP connectivity
    Standalone,
    /// Auto-detect MCP connectivity
    AutoDetect,
    /// Forced federated mode
    Federated,
}

/// Initialize the orchestrator with the given configuration
pub async fn initialize_orchestrator(config: OrchestratorConfig) -> Result<Orchestrator> {
    tracing::info!("Initializing NestGate Orchestrator");
    
    let orchestrator = Orchestrator::new(config).await?;
        
    tracing::info!("NestGate Orchestrator initialized successfully");
    Ok(orchestrator)
} 