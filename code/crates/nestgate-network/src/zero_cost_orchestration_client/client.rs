use crate::zero_cost_orchestration_types::ZeroCostOrchestrationConfig;
use nestgate_core::error::{NestGateError, Result};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};
use tokio::sync::Semaphore;
use uuid::Uuid;

use super::ZeroCostOrchestrationStats;

/// **Zero-cost orchestration client implementation**
///
/// This implementation demonstrates the migration from async_trait to native
/// async methods, providing significant performance improvements for network
/// operations while maintaining full compatibility.
pub struct ZeroCostOrchestrationClient {
    /// Service identifier
    pub id: Uuid,
    /// Service name
    pub name: String,
    /// Client configuration
    pub config: ZeroCostOrchestrationConfig,
    /// HTTP client with connection pooling
    pub http_client: reqwest::Client,
    /// Service startup time
    pub startup_time: SystemTime,
    /// Connection semaphore for rate limiting
    pub connection_semaphore: Arc<Semaphore>,
    /// Operation statistics
    pub stats: Arc<RwLock<ZeroCostOrchestrationStats>>,
    /// Registered services cache
    pub service_registry: Arc<
        RwLock<
            HashMap<
                String,
                crate::zero_cost_orchestration_client::registry::ZeroCostServiceInstance,
            >,
        >,
    >,
    /// Allocated ports tracking
    pub allocated_ports: Arc<RwLock<HashMap<String, u16>>>,
}

impl ZeroCostOrchestrationClient {
    /// Create a new zero-cost orchestration client
    pub fn new(name: String) -> Self {
        let config = ZeroCostOrchestrationConfig::default();
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.request_timeout_s))
            .connection_verbose(true)
            .build()
            .unwrap_or_else(|e| {
                tracing::error!("Failed to create HTTP client: {:?}", e);
                // Return a default client configuration
                reqwest::Client::new()
            });

        Self {
            id: Uuid::new_v4(),
            name,
            connection_semaphore: Arc::new(Semaphore::new(config.max_connections)),
            config,
            http_client,
            startup_time: SystemTime::now(),
            stats: Arc::new(RwLock::new(ZeroCostOrchestrationStats::default())),
            service_registry: Arc::new(RwLock::new(HashMap::new())),
            allocated_ports: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new client with custom configuration
    pub fn with_config(name: String, config: ZeroCostOrchestrationConfig) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.request_timeout_s))
            .connection_verbose(true)
            .build()
            .map_err(|e| {
                NestGateError::network_error(
                    &format!("Failed to create HTTP client: {}", e),
                    "client_creation",
                    Some(&config.base_url),
                )
            })?;

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            connection_semaphore: Arc::new(Semaphore::new(config.max_connections)),
            http_client,
            config,
            startup_time: SystemTime::now(),
            stats: Arc::new(RwLock::new(ZeroCostOrchestrationStats::default())),
            service_registry: Arc::new(RwLock::new(HashMap::new())),
            allocated_ports: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Get client configuration
    pub fn config(&self) -> &ZeroCostOrchestrationConfig {
        &self.config
    }

    /// Get client ID
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Get client name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get startup time
    pub fn startup_time(&self) -> SystemTime {
        self.startup_time
    }

    /// Get connection semaphore for rate limiting
    pub fn connection_semaphore(&self) -> &Arc<Semaphore> {
        &self.connection_semaphore
    }

    /// Get HTTP client reference
    pub fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    /// Update client configuration
    pub async fn update_config(&mut self, config: ZeroCostOrchestrationConfig) -> Result<()> {
        // Validate configuration
        if config.base_url.is_empty() {
            return Err(NestGateError::invalid_input(
                "base_url".to_string(),
                "Base URL cannot be empty".to_string(),
            ));
        }

        if config.max_connections == 0 {
            return Err(NestGateError::invalid_input(
                "max_connections".to_string(),
                "Maximum connections must be greater than 0".to_string(),
            ));
        }

        // Recreate HTTP client with new configuration
        self.http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.request_timeout_s))
            .connection_verbose(true)
            .build()
            .map_err(|e| {
                NestGateError::network_error(
                    &format!("Failed to create HTTP client: {}", e),
                    "config_update",
                    Some(&config.base_url),
                )
            })?;

        // Update connection semaphore
        self.connection_semaphore = Arc::new(Semaphore::new(config.max_connections));

        // Update configuration
        self.config = config;

        Ok(())
    }

    /// Get current statistics
    pub fn get_stats(&self) -> ZeroCostOrchestrationStats {
        match self.stats.read() {
            Ok(stats) => stats.clone(),
            Err(e) => {
                tracing::error!("RwLock read poisoned - returning default stats: {:?}", e);
                ZeroCostOrchestrationStats::default()
            }
        }
    }

    /// Reset statistics
    pub fn reset_stats(&self) -> Result<(), NetworkError> {
        match self.stats.write() {
            Ok(mut stats) => {
                *stats = ZeroCostOrchestrationStats::default();
                Ok(())
            }
            Err(e) => {
                tracing::error!("RwLock write poisoned during stats reset: {:?}", e);
                Err(NetworkError::Internal {
                    message: "Concurrency error during stats reset".to_string(),
                    source: None,
                })
            }
        }
    }

    /// Get service registry snapshot
    pub fn get_registry_snapshot(
        &self,
    ) -> HashMap<String, super::registry::ZeroCostServiceInstance> {
        match self.service_registry.read() {
            Ok(registry) => registry.clone(),
            Err(e) => {
                tracing::error!("RwLock read poisoned - returning empty registry: {:?}", e);
                HashMap::new()
            }
        }
    }

    /// Get allocated ports snapshot
    pub fn get_allocated_ports(&self) -> HashMap<String, u16> {
        match self.allocated_ports.read() {
            Ok(ports) => ports.clone(),
            Err(e) => {
                tracing::error!("RwLock read poisoned - returning empty ports map: {:?}", e);
                HashMap::new()
            }
        }
    }
}

impl Default for ZeroCostOrchestrationClient {
    fn default() -> Self {
        Self::new("default-orchestration-client".to_string())
    }
}
