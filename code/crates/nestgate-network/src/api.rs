//
// This module provides network services and port management through Orchestration orchestration.

//! Api module

use axum::{extract::State, http::StatusCode, response::Json, routing::get, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
// Removed unused tracing import

// Type alias to reduce complexity
type PortAllocationMap = Arc<RwLock<HashMap<String, u16>>>;

use nestgate_core::error::Result as NestGateResult;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::warn;

/// Service status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Status values for Service
pub enum ServiceStatus {
    /// Starting
    Starting,
    /// Running
    Running,
    /// Stopping
    Stopping,
    /// Stopped
    Stopped,
    /// Failed
    Failed,
}
/// Service instance information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serviceinstance
pub struct ServiceInstance {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Host
    pub host: String,
    /// Port
    pub port: u16,
    /// Status
    pub status: ServiceStatus,
    /// Timestamp when this was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Timestamp of last update
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
/// Network API state
type NetworkApiState = Arc<RwLock<HashMap<String, ServiceInstance>>>;
// Type alias to reduce complexity
type ServiceRegistry = Arc<RwLock<HashMap<String, ServiceInstance>>>;

/// Orchestration orchestrator client for network operations
///
/// NOTE: HTTP removed per Concentrated Gap Architecture
/// All orchestration now via Songbird gateway using Unix sockets
#[allow(dead_code)]
#[derive(Debug, Clone)]
/// Orchestrationcapability
pub struct OrchestrationCapability {
    /// Base URL for Orchestration orchestrator
    pub base_url: String,
    // HTTP client removed - use Unix sockets via Songbird
}
#[allow(dead_code)]
impl OrchestrationCapability {
    /// Create a new Orchestration client
    ///
    /// NOTE: HTTP removed - use Unix sockets via Songbird
    #[must_use]
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    /// Register a service with Orchestration
    ///
    /// NOTE: HTTP removed - use Unix sockets via Songbird
    #[allow(dead_code)]
    pub async fn register_service(&self, service: &ServiceInstance) -> NestGateResult<()> {
        // HTTP removed per Concentrated Gap Architecture
        let _ = service;
        unimplemented!("HTTP removed - use Unix sockets via Songbird gateway")
    }

    /// Request port allocation from Orchestration
    ///
    /// NOTE: HTTP removed - use Unix sockets via Songbird
    #[allow(dead_code)]
    pub async fn allocate_port(&self, service_name: &str, port_type: &str) -> NestGateResult<u16> {
        // HTTP removed per Concentrated Gap Architecture
        let _ = (service_name, port_type);
        unimplemented!("HTTP removed - use Unix sockets via Songbird gateway")
    }

    /// Release port allocation
    ///
    /// NOTE: HTTP removed - use Unix sockets via Songbird
    #[allow(dead_code)]
    pub async fn release_port(&self, service_name: &str, port: u16) -> NestGateResult<()> {
        // HTTP removed per Concentrated Gap Architecture
        let _ = (service_name, port);
        unimplemented!("HTTP removed - use Unix sockets via Songbird gateway")
    }

    /// Send health status to Orchestration
    ///
    /// NOTE: HTTP removed - use Unix sockets via Songbird
    #[allow(dead_code)]
    pub async fn send_health_status(
        &self,
        service_name: &str,
        status: ServiceStatus,
    ) -> NestGateResult<()> {
        // HTTP removed per Concentrated Gap Architecture
        let _ = (service_name, status);
        unimplemented!("HTTP removed - use Unix sockets via Songbird gateway")
    }
}

/// Port allocation request for Orchestration
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PortAllocationRequest {
    service_name: String,
    port_type: String,
    preferred_port: Option<u16>,
}
/// Port allocation response from Orchestration
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PortAllocationResponse {
    port: u16,
    expires_at: Option<String>,
}
/// Port release request for Orchestration
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PortReleaseRequest {
    service_name: String,
    port: u16,
}
/// Health status request for Orchestration
#[derive(Debug, Clone, Serialize, Deserialize)]
struct HealthStatusRequest {
    service_name: String,
    status: ServiceStatus,
    timestamp: chrono::DateTime<chrono::Utc>,
    metadata: std::collections::HashMap<String, String>,
}
/// Network API with Orchestration integration
#[derive(Debug)]
/// Networkapi
pub struct NetworkApi {
    /// Orchestration client for orchestration
    orchestration_client: Option<OrchestrationCapability>,
    /// Local service registry
    services: ServiceRegistry,
    /// Allocated ports
    allocated_ports: PortAllocationMap,
}
impl NetworkApi {
    /// Create a new `NetworkApi` instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            orchestration_client: None,
            services: Arc::new(RwLock::new(HashMap::new())),
            allocated_ports: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initialize with orchestration capability
    pub fn initialize_with_orchestration(
        &mut self,
        _orchestration_endpoint: String,
    ) -> NestGateResult<()> {
        self.orchestration_client = Some(OrchestrationCapability::new(_orchestration_endpoint));
        info!("🌐 NetworkApi initialized with orchestration capability");
        Ok(())
    }

    /// Register a service
    pub async fn register_service(&self, service: ServiceInstance) -> NestGateResult<()> {
        // Register with Orchestration if available
        if let Some(orchestration) = &self.orchestration_client {
            orchestration.register_service(&service).await?;
        }

        // Store locally - log before moving service
        info!("✅ Service registered: {}", service.name);
        let mut services = self.services.write().await;
        services.insert(service.name.clone(), service);
        Ok(())
    }

    /// Allocate a port for a service
    pub async fn allocate_port(&self, service_name: &str, port_type: &str) -> NestGateResult<u16> {
        // ✅ ORCHESTRATION CAPABILITY IS MANDATORY - NO LOCAL FALLBACK
        let orchestration_client = self.orchestration_client.as_ref()
            .ok_or_else(|| nestgate_core::NestGateError::network_error(
                "Orchestration capability is required for port allocation. Initialize with initialize_with_orchestration() first."
            ))?;

        let port = orchestration_client
            .allocate_port(service_name, port_type)
            .await?;

        // Store allocation
        let mut allocated = self.allocated_ports.write().await;
        allocated.insert(service_name.to_string(), port);

        Ok(port)
    }

    /// Release a port
    pub async fn release_port(&self, service_name: &str) -> NestGateResult<()> {
        let port = {
            let mut allocated = self.allocated_ports.write().await;
            allocated.remove(service_name)
        };

        if let Some(port) = port {
            // ✅ ORCHESTRATION CAPABILITY IS MANDATORY - NO LOCAL FALLBACK
            let orchestrator = self.orchestration_client.as_ref().ok_or_else(|| {
                nestgate_core::NestGateError::network_error(
                    "Orchestration capability is required for port release.",
                )
            })?;

            orchestrator.release_port(service_name, port).await?;
        }
        Ok(())
    }

    /// Get service status
    pub async fn get_service_status(&self, service_name: &str) -> NestGateResult<ServiceStatus> {
        let services = self.services.read().await;

        if let Some(service) = services.get(service_name) {
            Ok(service.status.clone())
        } else {
            Err(nestgate_core::NestGateError::network_error(&format!(
                "Service not found: {service_name}"
            )))
        }
    }

    /// List all registered services
    pub async fn list_services(&self) -> NestGateResult<Vec<ServiceInstance>> {
        let services = self.services.read().await;
        Ok(services.values().cloned().collect())
    }

    /// Create a router for the Network API
    pub fn create_router(&self) -> Router {
        Router::new()
            .route("/api/health", get(health_check))
            .route("/api/services", get(list_services_handler))
            .with_state(self.services.clone())
    }
}

impl Default for NetworkApi {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

/// Re-export universal API response from nestgate-core to eliminate duplication
pub use nestgate_core::response::api_response::ApiResponse;
// API Handlers

/// Health Check
async fn health_check() -> (StatusCode, Json<ApiResponse<String>>) {
    (StatusCode::OK, Json(ApiResponse::success("OK".to_string())))
}

/// List Services Handler
async fn list_services_handler(
    State(state): State<NetworkApiState>,
) -> (StatusCode, Json<ApiResponse<Vec<ServiceInstance>>>) {
    let services = state.read().await;
    let service_list: Vec<ServiceInstance> = services.values().cloned().collect();

    (StatusCode::OK, Json(ApiResponse::success(service_list)))
}
