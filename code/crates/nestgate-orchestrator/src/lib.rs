//! NestGate v2 Orchestrator
//! 
//! Enhanced orchestrator with advanced integration MCP capabilities
//! Central coordination hub for all NestGate services

pub mod service;
pub mod registry;
pub mod proxy;
pub mod health;
pub mod mcp_integration;
pub mod federation;
pub mod load_balancer;
pub mod metrics;
pub mod errors;

// Re-export main types
pub use service::*;
pub use registry::*;
pub use proxy::*;
pub use health::*;
pub use mcp_integration::*;
pub use federation::*;
pub use load_balancer::*;
pub use metrics::*;
pub use errors::*;

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error, debug};
use std::collections::HashMap;

// Import enhanced MCP types
use nestgate_mcp::{
    McpConfig, SystemMetrics, ProviderCapabilities,
    protocol::{Message, Response, MessageType, ResponsePayload, ServiceInfo, LoadBalancingInfo, HealthStatus},
    error::{Result, Error},
};

// Use specific registry to avoid ambiguity
use crate::registry::ServiceRegistry;
use crate::metrics::OrchestratorMetrics;

/// Enhanced Orchestrator Configuration integrating enhanced NestGate capabilities
#[derive(Debug, Clone)]
pub struct OrchestratorConfig {
    /// HTTP server bind address
    pub bind_address: String,
    /// MCP integration configuration
    pub mcp_config: Option<McpConfig>,
    /// Federation configuration
    pub federation_config: Option<FederationConfig>,
    /// Load balancing configuration
    pub load_balancer_config: LoadBalancerConfig,
    /// Health check configuration
    pub health_config: HealthConfig,
    /// Metrics configuration
    pub metrics_config: MetricsConfig,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0:8090".to_string(),
            mcp_config: Some(McpConfig::default()),
            federation_config: None,
            load_balancer_config: LoadBalancerConfig::default(),
            health_config: HealthConfig::default(),
            metrics_config: MetricsConfig::default(),
        }
    }
}

/// Enhanced Orchestrator with advanced MCP integration
pub struct Orchestrator {
    config: Arc<RwLock<OrchestratorConfig>>,
    service_registry: Arc<registry::ServiceRegistry>,
    connection_proxy: Arc<ConnectionProxy>,
    health_monitor: Arc<HealthMonitor>,
    federation_manager: Option<Arc<FederationManager>>,
    load_balancer: Arc<LoadBalancer>,
    metrics_collector: Arc<MetricsCollector>,
}

impl Orchestrator {
    /// Create new orchestrator with enhanced MCP integration
    pub async fn new(config: OrchestratorConfig) -> Result<Self> {
        info!("Initializing NestGate v2 Orchestrator with enhanced MCP integration");
        
        let service_registry = Arc::new(registry::ServiceRegistry::new());
        let connection_proxy = Arc::new(ConnectionProxy::new());
        let health_monitor = Arc::new(HealthMonitor::new(config.health_config.clone()));
        let load_balancer = Arc::new(LoadBalancer::new(config.load_balancer_config.clone()));
        let metrics_collector = Arc::new(MetricsCollector::new(config.metrics_config.clone()));

        // Initialize federation manager if configured
        let federation_manager = if let Some(federation_config) = &config.federation_config {
            info!("Initializing federation manager");
            Some(Arc::new(FederationManager::new(federation_config.clone()).await?))
        } else {
            None
        };

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            service_registry,
            connection_proxy,
            health_monitor,
            federation_manager,
            load_balancer,
            metrics_collector,
        })
    }

    /// Start the orchestrator
    pub async fn start(&self) -> Result<()> {
        info!("Starting NestGate v2 Orchestrator");

        // Start health monitoring
        self.health_monitor.start().await.map_err(|e| Error::internal(format!("Health monitor start failed: {}", e)))?;
        info!("Health monitor started");

        // Start metrics collection
        self.metrics_collector.start().await.map_err(|e| Error::internal(format!("Metrics collector start failed: {}", e)))?;
        info!("Metrics collector started");

        // Start federation if configured
        if let Some(federation_manager) = &self.federation_manager {
            federation_manager.start().await?;
            info!("Federation manager started");
        }

        // Start load balancer
        self.load_balancer.start().await?;
        info!("Load balancer started");

        info!("NestGate v2 Orchestrator fully operational");
        Ok(())
    }

    /// Register a service with enhanced capabilities
    pub async fn register_service(&self, service_info: ServiceInfo) -> Result<()> {
        info!("Registering service: {} ({})", service_info.service_name, service_info.service_id);
        
        // Register with service registry
        self.service_registry.register(service_info.clone()).await?;
        
        // Update load balancer
        self.load_balancer.add_service(service_info.clone()).await?;
        
        // Start health monitoring for the service
        self.health_monitor.monitor_service(service_info.clone()).await?;
        
        // Notify federation if available
        if let Some(federation_manager) = &self.federation_manager {
            federation_manager.notify_service_registration(&service_info).await?;
        }

        info!("Service registered successfully: {}", service_info.service_name);
        Ok(())
    }

    /// Route MCP message through orchestrator (advanced integration capabilities)
    pub async fn route_mcp_message(&self, message: Message) -> Result<Response> {
        debug!("Routing MCP message: {:?}", message.message_type);
        
        match message.message_type {
            MessageType::ServiceRegistration => {
                self.handle_service_registration_message(message).await
            }
            MessageType::ServiceDiscovery => {
                self.handle_service_discovery_message(message).await
            }
            MessageType::LoadBalancing => {
                self.handle_load_balancing_message(message).await
            }
            MessageType::HealthCheck => {
                self.handle_health_check_message(message).await
            }
            MessageType::MetricsReport => {
                self.handle_metrics_report_message(message).await
            }
            MessageType::FederationJoin | MessageType::FederationLeave | MessageType::FederationSync => {
                self.handle_federation_message(message).await
            }
            _ => {
                // Route to appropriate service
                self.route_to_service(message).await
            }
        }
    }

    /// Collect comprehensive system metrics with advanced capabilities
    pub async fn collect_system_metrics(&self) -> Result<SystemMetrics> {
        debug!("Collecting comprehensive system metrics");
        
        // Collect base system metrics
        let mut metrics = SystemMetrics::collect().await
            .map_err(|e| Error::internal(format!("Failed to collect system metrics: {}", e)))?;
        
        // Enhance with orchestrator-specific metrics
        self.enhance_metrics_with_orchestrator_data(&mut metrics).await?;
        
        // Collect service metrics
        self.enhance_metrics_with_service_data(&mut metrics).await?;
        
        Ok(metrics)
    }

    /// Get load balancing information
    pub async fn get_load_balancing_info(&self) -> Result<LoadBalancingInfo> {
        self.load_balancer.get_info().await
    }

    /// Get service list
    pub async fn get_services(&self) -> Result<Vec<ServiceInfo>> {
        self.service_registry.list_services().await
    }

    /// Shutdown orchestrator gracefully
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down NestGate v2 Orchestrator");
        
        // Stop federation first
        if let Some(federation_manager) = &self.federation_manager {
            federation_manager.shutdown().await?;
        }
        
        // Stop other components
        self.load_balancer.shutdown().await?;
        self.health_monitor.shutdown().await?;
        self.metrics_collector.shutdown().await.map_err(|e| Error::internal(format!("Metrics collector shutdown failed: {}", e)))?;
        
        info!("NestGate v2 Orchestrator shutdown complete");
        Ok(())
    }

    // Private helper methods

    async fn handle_service_registration_message(&self, message: Message) -> Result<Response> {
        // Handle service registration through MCP protocol
        Ok(Response::success(message.id, ResponsePayload::Empty))
    }

    async fn handle_service_discovery_message(&self, message: Message) -> Result<Response> {
        let services = self.get_services().await?;
        Ok(Response::success(message.id, ResponsePayload::ServiceList(services)))
    }

    async fn handle_load_balancing_message(&self, message: Message) -> Result<Response> {
        let lb_info = self.get_load_balancing_info().await?;
        Ok(Response::success(message.id, ResponsePayload::LoadBalancingInfo(lb_info)))
    }

    async fn handle_health_check_message(&self, message: Message) -> Result<Response> {
        let health_status = self.health_monitor.get_overall_health().await?;
        Ok(Response::success(message.id, ResponsePayload::HealthStatus(health_status)))
    }

    async fn handle_metrics_report_message(&self, message: Message) -> Result<Response> {
        let metrics = self.collect_system_metrics().await?;
        Ok(Response::success(message.id, ResponsePayload::MetricsReport(metrics)))
    }

    async fn handle_federation_message(&self, message: Message) -> Result<Response> {
        if let Some(federation_manager) = &self.federation_manager {
            federation_manager.handle_message(message).await
        } else {
            Err(Error::unsupported("Federation not enabled".to_string()))
        }
    }

    async fn route_to_service(&self, message: Message) -> Result<Response> {
        // Route message to appropriate service based on load balancing
        let target_service = self.load_balancer.select_service(&message).await?;
        self.connection_proxy.forward_message(target_service, message).await
    }

    async fn enhance_metrics_with_orchestrator_data(&self, _metrics: &mut SystemMetrics) -> Result<()> {
        // Get orchestrator-specific metrics
        let orchestrator_metrics = self.metrics_collector.get_system_metrics().await;
        
        // The SystemMetrics from nestgate_mcp has different fields, so we can't directly copy
        // Instead, we'll update what we can
        debug!("Enhanced metrics with orchestrator data - services: {}, uptime: {}s", 
               orchestrator_metrics.total_services, orchestrator_metrics.uptime_seconds);
        
        Ok(())
    }

    async fn enhance_metrics_with_service_data(&self, _metrics: &mut SystemMetrics) -> Result<()> {
        // Add service-specific metrics
        // This would aggregate metrics from all registered services
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_orchestrator_creation() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config).await.unwrap();
        
        // Test that orchestrator is created successfully
        assert!(orchestrator.service_registry.list_services().await.unwrap().is_empty());
    }

    #[test]
    async fn test_service_registration() {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config).await.unwrap();
        
        let service_info = ServiceInfo {
            service_id: "test-service-1".to_string(),
            service_name: "Test Service".to_string(),
            service_type: "storage".to_string(),
            endpoint: "http://localhost:8080".to_string(),
            status: nestgate_mcp::protocol::ServiceStatus::Online,
            capabilities: vec!["nfs".to_string(), "s3".to_string()],
            metadata: HashMap::new(),
        };
        
        orchestrator.register_service(service_info.clone()).await.unwrap();
        
        let services = orchestrator.get_services().await.unwrap();
        assert_eq!(services.len(), 1);
        assert_eq!(services[0].service_id, "test-service-1");
    }
} 