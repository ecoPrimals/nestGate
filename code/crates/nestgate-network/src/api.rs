//! Network API for NestGate with Songbird orchestrator integration
//!
//! This module provides network services and port management through Songbird orchestration.

use axum::{extract::State, http::StatusCode, response::Json, routing::get, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
// Removed unused tracing import

use nestgate_core::error::Result;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::warn;

/// Service status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
}

/// Service instance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub status: ServiceStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Network API state
type NetworkApiState = Arc<RwLock<HashMap<String, ServiceInstance>>>;

/// Songbird orchestrator client for network operations
#[derive(Debug, Clone)]
pub struct SongbirdClient {
    /// Base URL for Songbird orchestrator
    pub base_url: String,
    /// HTTP client for communication
    pub client: reqwest::Client,
}

impl SongbirdClient {
    /// Create a new Songbird client
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    /// Register a service with Songbird
    pub async fn register_service(&self, service: &ServiceInstance) -> Result<()> {
        let url = format!("{}/api/v1/services/register", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(service)
            .send()
            .await
            .map_err(|e| {
                nestgate_core::NestGateError::network_error(
                    &format!("Failed to register service: {e}"),
                    "service_registration",
                    Some("service_registry"),
                )
            })?;

        if response.status().is_success() {
            info!("✅ Service registered with Songbird: {}", service.name);
            Ok(())
        } else {
            let error_msg = format!("Failed to register service: HTTP {}", response.status());
            error!("{}", error_msg);
            Err(nestgate_core::NestGateError::network_error(
                &error_msg,
                "service_registration",
                Some("service_registry"),
            ))
        }
    }

    /// Request port allocation from Songbird
    pub async fn allocate_port(&self, service_name: &str, port_type: &str) -> Result<u16> {
        let url = format!("{}/api/v1/ports/allocate", self.base_url);

        let request = PortAllocationRequest {
            service_name: service_name.to_string(),
            port_type: port_type.to_string(),
            preferred_port: None,
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                nestgate_core::NestGateError::network_error(
                    &format!("Failed to allocate port: {e}"),
                    "port_allocation",
                    Some("port-allocation"),
                )
            })?;

        if response.status().is_success() {
            let allocation: PortAllocationResponse = response.json().await.map_err(|e| {
                nestgate_core::NestGateError::network_error(
                    &format!("Failed to parse port allocation response: {e}"),
                    "port_allocation_parsing",
                    Some("port-allocation"),
                )
            })?;

            info!(
                "✅ Port allocated by Songbird: {} -> {}",
                service_name, allocation.port
            );
            Ok(allocation.port)
        } else {
            let error_msg = format!("Failed to allocate port: HTTP {}", response.status());
            error!("{}", error_msg);
            Err(nestgate_core::NestGateError::network_error(
                &error_msg,
                "port_allocation",
                Some("port-allocation"),
            ))
        }
    }

    /// Release port allocation
    pub async fn release_port(&self, service_name: &str, port: u16) -> Result<()> {
        let url = format!("{}/api/v1/ports/release", self.base_url);

        let request = PortReleaseRequest {
            service_name: service_name.to_string(),
            port,
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                nestgate_core::NestGateError::network_error(
                    &format!("Failed to release port: {e}"),
                    "port_release",
                    Some("port-release"),
                )
            })?;

        if response.status().is_success() {
            info!("✅ Port released by Songbird: {} -> {}", service_name, port);
            Ok(())
        } else {
            warn!(
                "Failed to release port {}: HTTP {}",
                port,
                response.status()
            );
            Ok(()) // Don't fail on port release errors
        }
    }

    /// Send health status to Songbird
    pub async fn send_health_status(
        &self,
        service_name: &str,
        status: ServiceStatus,
    ) -> Result<()> {
        let url = format!("{}/api/v1/services/{}/health", self.base_url, service_name);

        let request = HealthStatusRequest {
            service_name: service_name.to_string(),
            status: status.clone(),
            timestamp: chrono::Utc::now(),
            metadata: std::collections::HashMap::new(),
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                nestgate_core::NestGateError::network_error(
                    &format!("Failed to send health status: {e}"),
                    "health_status_send",
                    Some("health-status"),
                )
            })?;

        if response.status().is_success() {
            debug!(
                "✅ Health status sent to Songbird: {} -> {:?}",
                service_name, status
            );
            Ok(())
        } else {
            let error_msg = format!("Failed to send health status: HTTP {}", response.status());
            debug!("{}", error_msg);
            Err(nestgate_core::NestGateError::network_error(
                &error_msg,
                "health_status",
                Some("health-status"),
            ))
        }
    }
}

/// Port allocation request for Songbird
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PortAllocationRequest {
    service_name: String,
    port_type: String,
    preferred_port: Option<u16>,
}

/// Port allocation response from Songbird
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PortAllocationResponse {
    port: u16,
    expires_at: Option<String>,
}

/// Port release request for Songbird
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PortReleaseRequest {
    service_name: String,
    port: u16,
}

/// Health status request for Songbird
#[derive(Debug, Clone, Serialize, Deserialize)]
struct HealthStatusRequest {
    service_name: String,
    status: ServiceStatus,
    timestamp: chrono::DateTime<chrono::Utc>,
    metadata: std::collections::HashMap<String, String>,
}

/// Network API with Songbird integration
#[derive(Debug)]
pub struct NetworkApi {
    /// Songbird client for orchestration
    orchestration_client: Option<SongbirdClient>,
    /// Local service registry
    services: Arc<RwLock<HashMap<String, ServiceInstance>>>,
    /// Allocated ports
    allocated_ports: Arc<RwLock<HashMap<String, u16>>>,
}

impl NetworkApi {
    /// Create a new NetworkApi instance
    pub fn new() -> Self {
        Self {
            orchestration_client: None,
            services: Arc::new(RwLock::new(HashMap::new())),
            allocated_ports: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initialize with orchestration capability
    pub async fn initialize_with_orchestration(
        &mut self,
        orchestration_endpoint: String,
    ) -> Result<()> {
        self.orchestration_client = Some(SongbirdClient::new(orchestration_endpoint));
        info!("🌐 NetworkApi initialized with orchestration capability");
        Ok(())
    }

    /// Register a service
    pub async fn register_service(&self, service: ServiceInstance) -> Result<()> {
        // Register with Songbird if available
        if let Some(songbird) = &self.orchestration_client {
            songbird.register_service(&service).await?;
        }

        // Store locally
        let mut services = self.services.write().await;
        services.insert(service.name.clone(), service.clone());

        info!("✅ Service registered: {}", service.name);
        Ok(())
    }

    /// Allocate a port for a service
    pub async fn allocate_port(&self, service_name: &str, port_type: &str) -> Result<u16> {
        // ✅ ORCHESTRATION CAPABILITY IS MANDATORY - NO LOCAL FALLBACK
        let orchestration_client = self.orchestration_client.as_ref()
            .ok_or_else(|| nestgate_core::NestGateError::network_error(
                "Orchestration capability is required for port allocation. Initialize with initialize_with_orchestration() first.",
                "orchestration_check",
                Some("orchestration")
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
    pub async fn release_port(&self, service_name: &str) -> Result<()> {
        let port = {
            let mut allocated = self.allocated_ports.write().await;
            allocated.remove(service_name)
        };

        if let Some(port) = port {
            // ✅ ORCHESTRATION CAPABILITY IS MANDATORY - NO LOCAL FALLBACK
            let orchestrator = self.orchestration_client.as_ref().ok_or_else(|| {
                nestgate_core::NestGateError::network_error(
                    "Orchestration capability is required for port release.",
                    "orchestration_check",
                    Some("orchestration-service"),
                )
            })?;

            orchestrator.release_port(service_name, port).await?;
        }
        Ok(())
    }

    /// Get service status
    pub async fn get_service_status(&self, service_name: &str) -> Result<ServiceStatus> {
        let services = self.services.read().await;

        if let Some(service) = services.get(service_name) {
            Ok(service.status.clone())
        } else {
            Err(nestgate_core::NestGateError::network_error(
                &format!("Service not found: {service_name}"),
                "service_discovery",
                Some(service_name),
            ))
        }
    }

    /// List all registered services
    pub async fn list_services(&self) -> Result<Vec<ServiceInstance>> {
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
    fn default() -> Self {
        Self::new()
    }
}

/// Re-export universal API response from nestgate-core to eliminate duplication
pub use nestgate_core::response::ApiResponse;

// API Handlers

async fn health_check() -> (StatusCode, Json<ApiResponse<String>>) {
    (StatusCode::OK, Json(ApiResponse::success("OK".to_string())))
}

async fn list_services_handler(
    State(state): State<NetworkApiState>,
) -> (StatusCode, Json<ApiResponse<Vec<ServiceInstance>>>) {
    let services = state.read().await;
    let service_list: Vec<ServiceInstance> = services.values().cloned().collect();

    (StatusCode::OK, Json(ApiResponse::success(service_list)))
}
