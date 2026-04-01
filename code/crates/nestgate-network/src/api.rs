// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// This module provides network services and port management through Orchestration orchestration.

//! Api module

use axum::{Router, extract::State, http::StatusCode, response::Json, routing::get};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
// Removed unused tracing import

// Type alias to reduce complexity
type PortAllocationMap = Arc<RwLock<HashMap<String, u16>>>;

use nestgate_core::error::{NestGateError, Result as NestGateResult};
use tracing::info;

/// Service status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Clone)]
/// Orchestrationcapability
pub struct OrchestrationCapability {
    /// Base URL for Orchestration orchestrator
    pub base_url: String,
    // HTTP client removed - use Unix sockets via Songbird
}
impl OrchestrationCapability {
    /// Create a new Orchestration client
    ///
    /// NOTE: HTTP removed - use Unix sockets via Songbird
    #[must_use]
    pub const fn new(base_url: String) -> Self {
        Self { base_url }
    }

    /// Register a service with Orchestration
    ///
    /// ✅ EVOLVED: Returns error instead of panicking via unimplemented!()
    /// HTTP removed per Concentrated Gap Architecture - use Unix sockets via Songbird
    pub fn register_service(&self, service: &ServiceInstance) -> NestGateResult<()> {
        let _ = service;
        Err(NestGateError::network_error(
            "HTTP removed - use Unix sockets via Songbird gateway for service registration",
        ))
    }

    /// Request port allocation from Orchestration
    ///
    /// ✅ EVOLVED: Returns error instead of panicking
    pub fn allocate_port(&self, service_name: &str, port_type: &str) -> NestGateResult<u16> {
        let _ = (service_name, port_type);
        Err(NestGateError::network_error(
            "HTTP removed - use Unix sockets via Songbird gateway for port allocation",
        ))
    }

    /// Release port allocation
    ///
    /// ✅ EVOLVED: Returns error instead of panicking
    pub fn release_port(&self, service_name: &str, port: u16) -> NestGateResult<()> {
        let _ = (service_name, port);
        Err(NestGateError::network_error(
            "HTTP removed - use Unix sockets via Songbird gateway for port release",
        ))
    }

    /// Send health status to Orchestration
    ///
    /// ✅ EVOLVED: Returns error instead of panicking
    pub fn send_health_status(
        &self,
        service_name: &str,
        status: ServiceStatus,
    ) -> NestGateResult<()> {
        let _ = (service_name, status);
        Err(NestGateError::network_error(
            "HTTP removed - use Unix sockets via Songbird gateway for health reporting",
        ))
    }
}

/// Port allocation request for orchestration IPC.
///
/// Used in JSON-RPC `network.allocate_port` method calls to the orchestration layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortAllocationRequest {
    /// Name of the service requesting a port
    pub service_name: String,
    /// Type of port (e.g. "api", "metrics", "admin")
    pub port_type: String,
    /// Preferred port number, if any
    pub preferred_port: Option<u16>,
}

/// Port allocation response from orchestration IPC.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortAllocationResponse {
    /// Allocated port number
    pub port: u16,
    /// Optional expiry for the allocation lease
    pub expires_at: Option<String>,
}

/// Port release request for orchestration IPC.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortReleaseRequest {
    /// Name of the service releasing the port
    pub service_name: String,
    /// Port number being released
    pub port: u16,
}

/// Health status report for orchestration IPC.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatusRequest {
    /// Name of the reporting service
    pub service_name: String,
    /// Current service health status
    pub status: ServiceStatus,
    /// Timestamp of this health report
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Additional health metadata
    pub metadata: std::collections::HashMap<String, String>,
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
        orchestration_endpoint: String,
    ) -> NestGateResult<()> {
        self.orchestration_client = Some(OrchestrationCapability::new(orchestration_endpoint));
        info!("🌐 NetworkApi initialized with orchestration capability");
        Ok(())
    }

    /// Register a service
    pub async fn register_service(&self, service: ServiceInstance) -> NestGateResult<()> {
        // Register with Orchestration if available
        if let Some(orchestration) = &self.orchestration_client {
            orchestration.register_service(&service)?;
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

        let port = orchestration_client.allocate_port(service_name, port_type)?;

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

            orchestrator.release_port(service_name, port)?;
        }
        Ok(())
    }

    /// Get service status
    pub async fn get_service_status(&self, service_name: &str) -> NestGateResult<ServiceStatus> {
        let services = self.services.read().await;

        if let Some(service) = services.get(service_name) {
            Ok(service.status.clone())
        } else {
            Err(nestgate_core::NestGateError::network_error(format!(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_status_variants() {
        let statuses = [
            ServiceStatus::Starting,
            ServiceStatus::Running,
            ServiceStatus::Stopping,
            ServiceStatus::Stopped,
            ServiceStatus::Failed,
        ];
        assert_eq!(statuses.len(), 5);
        assert_eq!(ServiceStatus::Running, ServiceStatus::Running);
        assert_ne!(ServiceStatus::Running, ServiceStatus::Stopped);
    }

    #[test]
    fn test_service_instance_serialization() {
        let now = chrono::Utc::now();
        let instance = ServiceInstance {
            id: "id-1".to_string(),
            name: "test-svc".to_string(),
            host: "127.0.0.1".to_string(),
            port: 8080,
            status: ServiceStatus::Running,
            created_at: now,
            updated_at: now,
        };
        let json = serde_json::to_string(&instance).unwrap();
        assert!(json.contains("test-svc"));
        assert!(json.contains("8080"));
        let deserialized: ServiceInstance = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "test-svc");
        assert_eq!(deserialized.port, 8080);
    }

    #[test]
    fn test_orchestration_capability_new() {
        let cap = OrchestrationCapability::new("http://localhost:9000".to_string());
        assert_eq!(cap.base_url, "http://localhost:9000");
    }

    #[tokio::test]
    async fn test_orchestration_capability_register_service_returns_error() {
        let cap = OrchestrationCapability::new("http://localhost:9000".to_string());
        let instance = ServiceInstance {
            id: "id".to_string(),
            name: "svc".to_string(),
            host: "127.0.0.1".to_string(),
            port: 8080,
            status: ServiceStatus::Running,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let result = cap.register_service(&instance);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unix sockets"));
    }

    #[tokio::test]
    async fn test_orchestration_capability_allocate_port_returns_error() {
        let cap = OrchestrationCapability::new("http://localhost:9000".to_string());
        let result = cap.allocate_port("mysvc", "api");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_orchestration_capability_release_port_returns_error() {
        let cap = OrchestrationCapability::new("http://localhost:9000".to_string());
        let result = cap.release_port("mysvc", 8080);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_orchestration_capability_send_health_status_returns_error() {
        let cap = OrchestrationCapability::new("http://localhost:9000".to_string());
        let result = cap.send_health_status("mysvc", ServiceStatus::Running);
        assert!(result.is_err());
    }

    #[test]
    fn test_port_allocation_request_serialization() {
        let req = PortAllocationRequest {
            service_name: "api".to_string(),
            port_type: "http".to_string(),
            preferred_port: Some(8080),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("api"));
        let parsed: PortAllocationRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.preferred_port, Some(8080));
    }

    #[test]
    fn test_port_allocation_response_serialization() {
        let resp = PortAllocationResponse {
            port: 9090,
            expires_at: Some("2025-01-01T00:00:00Z".to_string()),
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("9090"));
    }

    #[test]
    fn test_port_release_request_serialization() {
        let req = PortReleaseRequest {
            service_name: "api".to_string(),
            port: 8080,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("api"));
    }

    #[test]
    fn test_health_status_request_serialization() {
        let req = HealthStatusRequest {
            service_name: "api".to_string(),
            status: ServiceStatus::Running,
            timestamp: chrono::Utc::now(),
            metadata: std::collections::HashMap::new(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("api"));
    }

    #[test]
    fn test_network_api_new_and_default() {
        let api = NetworkApi::new();
        let _router = api.create_router();
        let _api2 = NetworkApi::default();
    }

    #[test]
    fn test_network_api_initialize_with_orchestration() {
        let mut api = NetworkApi::new();
        let result = api.initialize_with_orchestration("unix:///tmp/orchestrator.sock".to_string());
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_network_api_register_service_without_orchestration() {
        let api = NetworkApi::new();
        let instance = ServiceInstance {
            id: "id-1".to_string(),
            name: "test-svc".to_string(),
            host: "127.0.0.1".to_string(),
            port: 8080,
            status: ServiceStatus::Running,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let result = api.register_service(instance).await;
        assert!(result.is_ok());
        let status = api.get_service_status("test-svc").await.unwrap();
        assert_eq!(status, ServiceStatus::Running);
        let services = api.list_services().await.unwrap();
        assert_eq!(services.len(), 1);
    }

    #[tokio::test]
    async fn test_network_api_allocate_port_without_orchestration_returns_error() {
        let api = NetworkApi::new();
        let result = api.allocate_port("svc", "api").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Orchestration"));
    }

    #[tokio::test]
    async fn test_network_api_get_service_status_not_found() {
        let api = NetworkApi::new();
        let result = api.get_service_status("nonexistent").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[tokio::test]
    async fn test_network_api_release_port_when_not_allocated() {
        let api = NetworkApi::new();
        let result = api.release_port("unknown-service").await;
        assert!(result.is_ok());
    }
}
