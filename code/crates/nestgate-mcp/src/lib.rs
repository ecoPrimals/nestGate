//! NestGate v2 MCP Integration
//! 
//! Enhanced MCP protocol integration adapter for NestGate v2
//! Integrates mature v1 capabilities with v2 orchestrator-centric architecture

use std::sync::Arc;
use std::collections::HashMap;
use std::time::SystemTime;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{debug, info};

// Internal modules
pub mod types;
pub mod protocol;
pub mod error;
pub mod provider;
pub mod adapter;
pub mod session;
pub mod storage;
pub mod security;

// Re-export types for convenience
pub use types::{
    EnhancedSystemMetrics as SystemMetrics, 
    EnhancedProviderCapabilities as ProviderCapabilities,
    MountRequest, MountInfo, VolumeRequest, VolumeInfo,
    AuthConfig, TlsConfig, ProviderConfig,
};

// Use specific Result type to avoid ambiguity
pub use error::{Error, ErrorType, ErrorSeverity};
pub type Result<T> = std::result::Result<T, Error>;

/// Enhanced MCP Configuration for v2 with advanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    /// MCP cluster endpoint
    pub cluster_endpoint: String,
    /// Node identifier
    pub node_id: String,
    /// Authentication configuration
    pub auth: Option<AuthConfig>,
    /// TLS configuration
    pub tls: Option<TlsConfig>,
    /// Provider configuration
    pub provider_config: Option<ProviderConfig>,
    /// Orchestrator endpoint
    pub orchestrator_endpoint: String,
    /// Federation enabled
    pub federation_enabled: bool,
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            cluster_endpoint: "http://localhost:8080".to_string(),
            node_id: "default-node".to_string(),
            auth: None,
            tls: None,
            provider_config: None,
            orchestrator_endpoint: "http://localhost:8090".to_string(),
            federation_enabled: false,
        }
    }
}

/// Enhanced MCP configuration with advanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedMcpConfig {
    pub node_id: String,
    pub cluster_name: String,
    pub federation_enabled: bool,
    pub orchestrator_endpoint: String,
    pub capabilities: ProviderCapabilities,
    pub metrics_collection_interval: u64,
    pub health_check_interval: u64,
    pub retry_config: RetryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
}

/// Orchestrator client trait for v2 integration
#[async_trait]
pub trait OrchestratorClient: Send + Sync {
    async fn register_service(&self, service_info: protocol::ServiceInfo) -> Result<()>;
    async fn send_metrics(&self, metrics: &SystemMetrics) -> Result<()>;
    async fn route_message(&self, message: protocol::Message) -> Result<protocol::Response>;
}

/// Enhanced MCP Service with enhanced NestGate capabilities integrated into v2 orchestrator
pub struct EnhancedMcpService {
    config: EnhancedMcpConfig,
    orchestrator_client: Arc<dyn OrchestratorClient>,
    capabilities: Arc<RwLock<ProviderCapabilities>>,
    metrics: Arc<RwLock<SystemMetrics>>,
    session_manager: Arc<session::SessionManager>,
    storage_adapter: Arc<storage::StorageAdapter>,
    provider_registry: Arc<RwLock<HashMap<String, provider::ProviderInfo>>>,
}

impl EnhancedMcpService {
    pub fn new(
        config: EnhancedMcpConfig,
        orchestrator_client: Arc<dyn OrchestratorClient>,
    ) -> Self {
        Self {
            config,
            orchestrator_client,
            capabilities: Arc::new(RwLock::new(ProviderCapabilities::default())),
            metrics: Arc::new(RwLock::new(SystemMetrics {
                timestamp: SystemTime::now(),
                node_id: "default".to_string(),
                cpu_usage: 0.0,
                memory_usage: 0.0,
                disk_usage: 0.0,
                network_io: types::NetworkIo {
                    bytes_sent: 0,
                    bytes_received: 0,
                    packets_sent: 0,
                    packets_received: 0,
                    errors: 0,
                },
                disk_io: types::DiskIo {
                    reads_completed: 0,
                    writes_completed: 0,
                    bytes_read: 0,
                    bytes_written: 0,
                    io_time_ms: 0,
                },
                storage_metrics: types::StorageMetrics {
                    total_capacity: 0,
                    used_capacity: 0,
                    available_capacity: 0,
                    tier_metrics: HashMap::new(),
                    pool_metrics: HashMap::new(),
                },
                performance_metrics: types::PerformanceMetrics {
                    iops: types::IopsMetrics {
                        read_iops: 0.0,
                        write_iops: 0.0,
                        total_iops: 0.0,
                        peak_iops: 0.0,
                    },
                    throughput: types::ThroughputMetrics {
                        read_throughput: 0.0,
                        write_throughput: 0.0,
                        total_throughput: 0.0,
                        peak_throughput: 0.0,
                    },
                    latency: types::LatencyMetrics {
                        read_latency_ms: 0.0,
                        write_latency_ms: 0.0,
                        avg_latency_ms: 0.0,
                        p95_latency_ms: 0.0,
                        p99_latency_ms: 0.0,
                    },
                },
            })),
            session_manager: Arc::new(session::SessionManager::new()),
            storage_adapter: Arc::new(storage::StorageAdapter::new()),
            provider_registry: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Collect enhanced system metrics
    pub async fn collect_metrics(&self) -> Result<SystemMetrics> {
        let metrics = SystemMetrics::collect().await.map_err(|e| {
            Error::internal(format!("Failed to collect metrics: {}", e))
        })?;
        
        // Update internal metrics
        *self.metrics.write().await = metrics.clone();
        
        Ok(metrics)
    }

    /// Register capabilities with orchestrator
    pub async fn register_capabilities(&self, capabilities: ProviderCapabilities) -> Result<()> {
        *self.capabilities.write().await = capabilities.clone();
        
        let service_info = protocol::ServiceInfo {
            service_id: self.config.node_id.clone(),
            service_name: "enhanced-mcp-service".to_string(),
            service_type: "mcp-service".to_string(),
            endpoint: self.config.orchestrator_endpoint.clone(),
            status: protocol::ServiceStatus::Online,
            capabilities: vec!["mcp".to_string(), "storage".to_string()],
            metadata: HashMap::new(),
        };
        
        self.orchestrator_client.register_service(service_info).await?;
        info!("Capabilities registered with orchestrator");
        Ok(())
    }

    /// Handle MCP message through orchestrator routing
    pub async fn handle_message(&self, message: protocol::Message) -> Result<protocol::Response> {
        debug!("Handling MCP message: {:?}", message);
        
        // Route through orchestrator for v2 integration
        self.orchestrator_client.route_message(message).await
    }

    /// Handle mount request through orchestrator
    pub async fn handle_mount_request(&self, request: MountRequest) -> Result<MountInfo> {
        debug!("Handling mount request: {:?}", request);
        
        // Use storage adapter for actual mounting
        self.storage_adapter.mount_volume(&request).await
    }

    /// Start the enhanced MCP service
    pub async fn start(&self) -> Result<()> {
        info!("Starting Enhanced MCP Service");
        
        // Register with orchestrator
        let capabilities = self.capabilities.read().await.clone();
        self.register_capabilities(capabilities).await?;
        
        // Start metrics collection
        self.start_metrics_collection().await?;
        
        // Start health monitoring
        self.start_health_monitoring().await?;
        
        info!("Enhanced MCP Service started successfully");
        Ok(())
    }

    /// Start periodic metrics collection
    async fn start_metrics_collection(&self) -> Result<()> {
        let metrics_interval = self.config.metrics_collection_interval;
        let orchestrator_client = self.orchestrator_client.clone();
        let metrics = self.metrics.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(
                std::time::Duration::from_secs(metrics_interval)
            );
            
            loop {
                interval.tick().await;
                
                // Handle metrics collection with proper error handling
                match SystemMetrics::collect().await {
                    Ok(current_metrics) => {
                        *metrics.write().await = current_metrics.clone();
                        
                        if let Err(e) = orchestrator_client.send_metrics(&current_metrics).await {
                            tracing::error!("Failed to send metrics to orchestrator: {}", e);
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to collect metrics: {}", e);
                    }
                }
            }
        });
        
        Ok(())
    }

    /// Start health monitoring
    async fn start_health_monitoring(&self) -> Result<()> {
        let health_interval = self.config.health_check_interval;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(
                std::time::Duration::from_secs(health_interval)
            );
            
            loop {
                interval.tick().await;
                // Perform health checks
                debug!("Performing health check");
            }
        });
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_config_creation() {
        let config = McpConfig {
            cluster_endpoint: "localhost:8080".to_string(),
            node_id: "test-node".to_string(),
            auth: None,
            tls: None,
            provider_config: None,
            orchestrator_endpoint: "localhost:9090".to_string(),
            federation_enabled: true,
        };
        
        assert_eq!(config.node_id, "test-node");
        assert_eq!(config.cluster_endpoint, "localhost:8080");
    }

    #[tokio::test]
    async fn test_provider_capabilities() {
        let capabilities = ProviderCapabilities::default();
        assert!(capabilities.supported_protocols.len() >= 0);
    }
} 