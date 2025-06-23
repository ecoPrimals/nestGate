//! Main Orchestrator Module
//! 
//! The core orchestrator that manages all services

use std::sync::Arc;
use std::collections::HashMap;
use std::sync::RwLock;

use crate::errors::Result;
use crate::config::OrchestratorConfig;
use crate::traits::service::UniversalService;
use crate::registry::{ServiceRegistry, ServiceHandle};

/// Main orchestrator implementation
pub struct Orchestrator {
    config: Arc<OrchestratorConfig>,
    registry: Arc<ServiceRegistry>,
}

impl Orchestrator {
    /// Create a new orchestrator with the given configuration
    /// 
    /// # Errors
    /// 
    /// Returns an error if the orchestrator fails to initialize
    pub async fn new(config: OrchestratorConfig) -> Result<Self> {
        let registry = ServiceRegistry::new().await?;
        
        Ok(Self {
            config: Arc::new(config),
            registry: Arc::new(registry),
        })
    }

    /// Get the orchestrator configuration
    pub fn config(&self) -> &OrchestratorConfig {
        &self.config
    }

    /// Get the service registry
    pub fn registry(&self) -> &ServiceRegistry {
        &self.registry
    }
    
    /// Register a service with the orchestrator
    /// 
    /// # Errors
    /// 
    /// Returns an error if the service registration fails
    pub async fn register_service<S: UniversalService>(&self, service: Box<S>) -> Result<()> {
        let info = service.service_info();
        self.registry.register(info).await?;
        Ok(())
    }
    
    /// Start the orchestrator
    /// 
    /// # Errors
    /// 
    /// Returns an error if the orchestrator fails to start
    pub fn start(&self) -> Result<()> {
        tracing::info!("Starting Songbird Orchestrator with config: {:?}", self.config.name);
        // Implementation will be expanded later
        Ok(())
    }
    
    /// Stop the orchestrator
    /// 
    /// # Errors
    /// 
    /// Returns an error if the orchestrator fails to stop
    pub fn stop(&self) -> Result<()> {
        tracing::info!("Stopping Songbird Orchestrator: {}", self.config.name);
        // Implementation will be expanded later
        Ok(())
    }
    
    /// Get orchestrator statistics
    /// 
    /// # Errors
    /// 
    /// Returns an error if the statistics cannot be retrieved
    pub async fn get_stats(&self) -> Result<OrchestratorStats> {
        let service_count = self.registry.service_count().await;
        
        Ok(OrchestratorStats {
            service_count,
            uptime_seconds: 0, // Will be implemented later
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
        })
    }
}

/// Orchestrator statistics
#[derive(Debug, Clone)]
pub struct OrchestratorStats {
    pub service_count: usize,
    pub uptime_seconds: u64,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
} 