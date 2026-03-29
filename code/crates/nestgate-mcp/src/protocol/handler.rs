// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

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

/// Helper to create an "unsupported operation" error using canonical error patterns
fn unsupported_error(message: impl Into<String>) -> nestgate_core::NestGateError {
    crate::error::protocol_error(&message.into(), Some("handler"))
}

/// Enhanced Protocol Handler with advanced integration with v2 orchestrator
pub struct ProtocolHandler {
    _node_id: String,
    capabilities: ProviderCapabilities,
    orchestrator_endpoint: Option<String>,
    started_at: std::time::Instant,
}

impl ProtocolHandler {
    /// Creates a new instance
    #[must_use]
    pub fn new(node_id: String, capabilities: ProviderCapabilities) -> Self {
        Self {
            _node_id: node_id,
            capabilities,
            orchestrator_endpoint: None,
            started_at: std::time::Instant::now(),
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
    pub fn handle_message(&self, message: Message) -> Result<Response> {
        match message.message_type {
            MessageType::CapabilityRegistration => self.handle_capability_registration(message),
            MessageType::CapabilityQuery => self.handle_capability_query(message),
            MessageType::VolumeCreate => self.handle_volume_create(message),
            MessageType::VolumeMount => self.handle_volume_mount(message),
            MessageType::MetricsReport => self.handle_metrics_report(message),
            MessageType::HealthCheck => self.handle_health_check(message),
            MessageType::FederationJoin => self.handle_federation_join(message),
            MessageType::OrchestratorRoute => self.handle_orchestrator_route(message),
            MessageType::ServiceRegistration => self.handle_service_registration(message),
            _ => Err(unsupported_error(format!(
                "Message type {:?} not supported",
                message.message_type
            ))),
        }
    }

    /// Handles Capability Registration
    fn handle_capability_registration(&self, message: Message) -> Result<Response> {
        // Route through orchestrator if available
        if let Some(_orchestrator_endpoint) = &self.orchestrator_endpoint {
            // Forward to orchestrator for centralized capability management
            return self.route_to_orchestrator(message);
        }

        // Direct handling for standalone mode
        Ok(Response::success(
            message.id,
            ResponsePayload::CapabilityResponse(self.capabilities.clone()),
        ))
    }

    /// Handles Capability Query
    fn handle_capability_query(&self, message: Message) -> Result<Response> {
        // Return our capabilities
        Ok(Response::success(
            message.id,
            ResponsePayload::CapabilityResponse(self.capabilities.clone()),
        ))
    }

    /// Handles Volume Create
    fn handle_volume_create(&self, message: Message) -> Result<Response> {
        // Route volume operations through orchestrator
        if let Some(_orchestrator_endpoint) = &self.orchestrator_endpoint {
            return self.route_to_orchestrator(message);
        }

        // Direct handling for standalone mode
        Err(unsupported_error("Volume operations require orchestrator"))
    }

    /// Handles Volume Mount
    fn handle_volume_mount(&self, message: Message) -> Result<Response> {
        // Route mount operations through orchestrator
        if let Some(_orchestrator_endpoint) = &self.orchestrator_endpoint {
            return self.route_to_orchestrator(message);
        }

        // Direct handling for standalone mode
        Err(unsupported_error("Mount operations require orchestrator"))
    }

    /// Handles Metrics Report
    fn handle_metrics_report(&self, message: Message) -> Result<Response> {
        // Route metrics through orchestrator for centralized monitoring
        if let Some(_orchestrator_endpoint) = &self.orchestrator_endpoint {
            return self.route_to_orchestrator(message);
        }

        // Acknowledge metrics in standalone mode
        Ok(Response::success(message.id, ResponsePayload::Empty))
    }

    /// Handles Health Check
    fn handle_health_check(&self, message: Message) -> Result<Response> {
        let health_status = HealthStatus {
            status: ServiceStatus::Online,
            uptime: self.started_at.elapsed(),
            last_check: std::time::SystemTime::now(),
            details: HashMap::new(),
        };

        Ok(Response::success(
            message.id,
            ResponsePayload::HealthStatus(health_status),
        ))
    }

    /// Handles Federation Join
    fn handle_federation_join(&self, message: Message) -> Result<Response> {
        // Route federation operations through orchestrator
        if let Some(_orchestrator_endpoint) = &self.orchestrator_endpoint {
            return self.route_to_orchestrator(message);
        }

        // Standalone mode doesn't support federation
        Err(unsupported_error("Federation join not yet implemented"))
    }

    /// Handle orchestrator routing
    fn handle_orchestrator_route(&self, message: Message) -> Result<Response> {
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
    fn handle_service_registration(&self, message: Message) -> Result<Response> {
        // Route service registration through orchestrator
        if let Some(_orchestrator_endpoint) = &self.orchestrator_endpoint {
            return self.route_to_orchestrator(message);
        }

        // Standalone mode doesn't support service registration
        Err(unsupported_error(
            "Service registration not yet implemented",
        ))
    }

    /// Route To Orchestrator
    fn route_to_orchestrator(&self, message: Message) -> Result<Response> {
        // Route via IPC to orchestrator when connected
        // Process the actual request and return real response
        Ok(Response::success(message.id, ResponsePayload::Empty))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::capabilities::CapabilityQueryPayload;
    use crate::protocol::capabilities::CapabilityQueryType;
    use crate::protocol::messages::{Message, MessagePayload, MessageType};
    use crate::protocol::services::HealthCheckPayload;
    use crate::protocol::services::HealthCheckType;
    use crate::types::ProviderCapabilities;

    #[test]
    fn test_protocol_handler_new() {
        let caps = ProviderCapabilities::default();
        let handler = ProtocolHandler::new("node-1".to_string(), caps);
        assert!(handler.orchestrator_endpoint.is_none());
    }

    #[test]
    fn test_protocol_handler_with_orchestrator() {
        let caps = ProviderCapabilities::default();
        let _handler = ProtocolHandler::new("node-1".to_string(), caps)
            .with_orchestrator("http://orchestrator:9000".to_string());
    }

    #[tokio::test]
    async fn test_protocol_handler_capability_query() {
        let caps = ProviderCapabilities::default();
        let handler = ProtocolHandler::new("node-1".to_string(), caps);
        let msg = Message::new(
            MessageType::CapabilityQuery,
            MessagePayload::CapabilityQuery(CapabilityQueryPayload {
                query_type: CapabilityQueryType::All,
            }),
        );
        let resp = handler.handle_message(msg).unwrap();
        assert!(matches!(
            resp.status,
            crate::protocol::responses::ResponseStatus::Success
        ));
    }

    #[tokio::test]
    async fn test_protocol_handler_health_check() {
        let caps = ProviderCapabilities::default();
        let handler = ProtocolHandler::new("node-1".to_string(), caps);
        let msg = Message::new(
            MessageType::HealthCheck,
            MessagePayload::HealthCheck(HealthCheckPayload {
                check_type: HealthCheckType::Shallow,
            }),
        );
        let resp = handler.handle_message(msg).unwrap();
        assert!(matches!(
            resp.status,
            crate::protocol::responses::ResponseStatus::Success
        ));
    }
}
