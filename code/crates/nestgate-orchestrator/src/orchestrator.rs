//! Main orchestrator implementation
//! 
//! The central connectivity hub that manages all service communication and discovery

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use nestgate_core::Result;
use crate::{OrchestratorConfig, FederationMode, ServiceRegistry, ConnectionProxy, HealthMonitor, McpFederation};

/// Main orchestrator struct - the central connectivity hub
#[derive(Debug, Clone)]
pub struct Orchestrator {
    /// Configuration
    config: Arc<OrchestratorConfig>,
    /// Service registry for managing local services
    service_registry: Arc<ServiceRegistry>,
    /// Connection proxy for routing requests
    connection_proxy: Arc<ConnectionProxy>,
    /// Health monitor for service health checks
    health_monitor: Arc<HealthMonitor>,
    /// MCP federation handler
    mcp_federation: Arc<McpFederation>,
    /// Running state
    running: Arc<RwLock<bool>>,
}

impl Orchestrator {
    /// Create a new orchestrator instance
    pub async fn new(config: OrchestratorConfig) -> Result<Self> {
        let config = Arc::new(config);
        
        // Initialize components
        let service_registry = Arc::new(ServiceRegistry::new());
        let connection_proxy = Arc::new(ConnectionProxy::new(service_registry.clone()));
        let health_monitor = Arc::new(HealthMonitor::new(config.health_check_interval));
        let mcp_federation = Arc::new(McpFederation::new(config.federation_mode.clone()));
        
        Ok(Self {
            config,
            service_registry,
            connection_proxy,
            health_monitor,
            mcp_federation,
            running: Arc::new(RwLock::new(false)),
        })
    }
    
    /// Start the orchestrator
    pub async fn start(&self) -> Result<()> {
        tracing::info!("Starting NestGate Orchestrator on {}", self.config.bind_address);
        
        {
            let mut running = self.running.write().await;
            if *running {
                return Ok(());
            }
            *running = true;
        }
        
        // Start all components
        self.service_registry.start().await?;
        self.connection_proxy.start(&self.config.bind_address).await?;
        self.health_monitor.start().await?;
        
        // Register local services
        for service_name in &self.config.local_services {
            self.register_local_service(service_name).await?;
        }
        
        // Start MCP federation if enabled
        match self.config.federation_mode {
            FederationMode::Standalone => {
                tracing::info!("Operating in standalone mode - no MCP federation");
            }
            FederationMode::AutoDetect => {
                tracing::info!("Auto-detecting MCP federation");
                self.mcp_federation.auto_detect().await?;
            }
            FederationMode::Federated => {
                tracing::info!("Starting MCP federation");
                self.mcp_federation.start().await?;
            }
        }
        
        tracing::info!("NestGate Orchestrator started successfully");
        Ok(())
    }
    
    /// Stop the orchestrator
    pub async fn stop(&self) -> Result<()> {
        tracing::info!("Stopping NestGate Orchestrator");
        
        {
            let mut running = self.running.write().await;
            if !*running {
                return Ok(());
            }
            *running = false;
        }
        
        // Stop all components
        self.mcp_federation.stop().await?;
        self.health_monitor.stop().await?;
        self.connection_proxy.stop().await?;
        self.service_registry.stop().await?;
        
        tracing::info!("NestGate Orchestrator stopped");
        Ok(())
    }
    
    /// Check if the orchestrator is running
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
    
    /// Register a local service
    async fn register_local_service(&self, service_name: &str) -> Result<()> {
        tracing::debug!("Registering local service: {}", service_name);
        
        // TODO: Implement service discovery and registration
        // This would typically involve:
        // 1. Discovering the service endpoint
        // 2. Registering it with the service registry
        // 3. Setting up health checks
        // 4. Configuring routing in the connection proxy
        
        Ok(())
    }
    
    /// Get a service endpoint
    pub async fn get_service(&self, service_name: &str) -> Result<String> {
        self.service_registry.get_service_endpoint(service_name).await
    }
    
    /// Get the service registry
    pub fn service_registry(&self) -> Arc<ServiceRegistry> {
        self.service_registry.clone()
    }
    
    /// Get the connection proxy
    pub fn connection_proxy(&self) -> Arc<ConnectionProxy> {
        self.connection_proxy.clone()
    }
    
    /// Get the health monitor
    pub fn health_monitor(&self) -> Arc<HealthMonitor> {
        self.health_monitor.clone()
    }
    
    /// Get MCP federation status
    pub async fn get_federation_status(&self) -> Result<FederationStatus> {
        self.mcp_federation.get_status().await
    }
}

/// Federation status information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FederationStatus {
    /// Whether federation is enabled
    pub enabled: bool,
    /// Whether currently connected to MCP cluster
    pub connected: bool,
    /// Number of federated nodes
    pub node_count: u32,
    /// Last heartbeat timestamp
    pub last_heartbeat: Option<chrono::DateTime<chrono::Utc>>,
} 