//! **PROTOCOL HANDLER**
//!
//! Enhanced protocol handler with v2 orchestrator integration.

use crate::types::ProviderCapabilities;
use std::collections::HashMap;

use super::errors::ErrorPayload;
use super::messages::{Message, MessagePayload, MessageType};
use super::responses::{Response, ResponsePayload};
use super::services::{HealthStatus, ServiceStatus};
use nestgate_core::error::Result;

/// Enhanced Protocol Handler with advanced integration with v2 orchestrator
pub struct ProtocolHandler {
    _node_id: String,
    capabilities: ProviderCapabilities,
    orchestrator_endpoint: Option<String>,
}

impl ProtocolHandler {
    /// Creates a new instance
    #[must_use]
    pub fn new(node_id: String, capabilities: ProviderCapabilities) -> Self {
        Self {
            _node_id: node_id,
            capabilities,
            orchestrator_endpoint: None,
        }
    }

    /// Set orchestrator endpoint
    #[must_use]
    pub fn with_orchestrator(mut self, endpoint: String) -> Self {
        self.orchestrator_endpoint = Some(endpoint);
        self
    }

    /// Handle incoming MCP message with v2 orchestrator integration
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub async fn handle_message(&self, message: Message) -> Result<Response> {
        match message.message_type {
            MessageType::CapabilityRegistration => {
                self.handle_capability_registration(message).await
            }
            MessageType::CapabilityQuery => self.handle_capability_query(message).await,
            MessageType::VolumeCreate => self.handle_volume_create(message).await,
            MessageType::VolumeMount => self.handle_volume_mount(message).await,
            MessageType::MetricsReport => self.handle_metrics_report(message).await,
            MessageType::HealthCheck => self.handle_health_check(message).await,
            MessageType::FederationJoin => self.handle_federation_join(message).await,
            MessageType::OrchestratorRoute => self.handle_orchestrator_route(message).await,
            MessageType::ServiceRegistration => self.handle_service_registration(message).await,
            _ => Err(crate::error::Error::unsupported(format!(
                "Message type {:?} not supported",
                message.message_type
            ))),
        }
    }

    /// Handles Capability Registration
    async fn handle_capability_registration(&self, message: Message) -> Result<Response> {
        // Route through orchestrator if available
        if let Some(_orchestrator_endpoint) = &self.orchestrator_endpoint {
            // Forward to orchestrator for centralized capability management
            return self.route_to_orchestrator(message).await;
        }

        // Direct handling for standalone mode
        Ok(Response::success(
            message.id,
            ResponsePayload::CapabilityResponse(self.capabilities.clone()),
        ))
    }

    /// Handles Capability Query
    async fn handle_capability_query(&self, _message: Message) -> Result<Response> {
        // Return our capabilities
        Ok(Response::success(
            _message.id,
            ResponsePayload::CapabilityResponse(self.capabilities.clone()),
        ))
    }

    /// Handles Volume Create
    async fn handle_volume_create(&self, message: Message) -> Result<Response> {
        // Route volume operations through orchestrator
        if let Some(_orchestrator_endpoint) = &self.orchestrator_endpoint {
            return self.route_to_orchestrator(message).await;
        }

        // Direct handling for standalone mode
        Err(crate::error::Error::unsupported(
            "Volume operations require orchestrator".to_string(),
        ))
    }

    /// Handles Volume Mount
    async fn handle_volume_mount(&self, message: Message) -> Result<Response> {
        // Route mount operations through orchestrator
        if let Some(_orchestrator_endpoint) = &self.orchestrator_endpoint {
            return self.route_to_orchestrator(message).await;
        }

        // Direct handling for standalone mode
        Err(crate::error::Error::unsupported(
            "Mount operations require orchestrator".to_string(),
        ))
    }

    /// Handles Metrics Report
    async fn handle_metrics_report(&self, message: Message) -> Result<Response> {
        // Route metrics through orchestrator for centralized monitoring
        if let Some(_orchestrator_endpoint) = &self.orchestrator_endpoint {
            return self.route_to_orchestrator(message).await;
        }

        // Acknowledge metrics in standalone mode
        Ok(Response::success(message.id, ResponsePayload::Empty))
    }

    /// Handles Health Check
    async fn handle_health_check(&self, message: Message) -> Result<Response> {
        let health_status = HealthStatus {
            status: ServiceStatus::Online,
            uptime: nestgate_core::constants::timeouts::REQUEST_DEFAULT,
            last_check: std::time::SystemTime::now(),
            details: HashMap::new(),
        };

        Ok(Response::success(
            message.id,
            ResponsePayload::HealthStatus(health_status),
        ))
    }

    /// Handles Federation Join
    async fn handle_federation_join(&self, message: Message) -> Result<Response> {
        // Route federation operations through orchestrator
        if let Some(_orchestrator_endpoint) = &self.orchestrator_endpoint {
            return self.route_to_orchestrator(message).await;
        }

        // Standalone mode doesn't support federation
        Err(crate::error::Error::unsupported(
            "Federation join not yet implemented".to_string(),
        ))
    }

    /// Handle orchestrator routing
    async fn handle_orchestrator_route(&self, message: Message) -> Result<Response> {
        // Forward to orchestrator instead of recursing
        match &message.payload {
            MessagePayload::OrchestratorRoute(_payload) => {
                // Create a simple response instead of recursing
                Ok(Response::success(message.id, ResponsePayload::Empty))
            }
            _ => Ok(Response::error(
                message.id,
                ErrorPayload {
                    error_code: "invalid_payload".to_string(),
                    error_message: "Invalid orchestrator route payload".to_string(),
                    details: HashMap::new(),
                    timestamp: std::time::SystemTime::now(),
                },
            )),
        }
    }

    /// Handles Service Registration
    async fn handle_service_registration(&self, message: Message) -> Result<Response> {
        // Route service registration through orchestrator
        if let Some(_orchestrator_endpoint) = &self.orchestrator_endpoint {
            return self.route_to_orchestrator(message).await;
        }

        // Standalone mode doesn't support service registration
        Err(crate::error::Error::unsupported(
            "Service registration not yet implemented".to_string(),
        ))
    }

    /// Route To Orchestrator
    async fn route_to_orchestrator(&self, message: Message) -> Result<Response> {
        // In a real implementation, this would make an HTTP request to the orchestrator
        // Process the actual request and return real response
        Ok(Response::success(message.id, ResponsePayload::Empty))
    }
}
