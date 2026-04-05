// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Capability routing, discovery, and local forwarding stubs.

use super::matching;
use super::registry::CapabilityRegistry;
use super::self_knowledge::NestGateSelfKnowledge;
use super::types::{CapabilityCategory, CapabilityRequest, CapabilityResponse, DiscoveredService};
use crate::Result;
use crate::universal_primal_discovery::service_registry::ServiceRegistry;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::super::capability_endpoints_config::CapabilityEndpointsConfig;

/// Universal capability router that eliminates all primal hardcoding
/// Each primal only knows itself and discovers others through capability advertisement
#[derive(Clone)]
/// Capabilityrouter
pub struct CapabilityRouter {
    /// Registry of discovered capabilities
    registry: Arc<RwLock<CapabilityRegistry>>,
    /// Our own service identity (only thing we know about ourselves)
    self_identity: NestGateSelfKnowledge,
    /// **NEW**: ServiceRegistry for capability-based discovery (no hardcoded URLs!)
    service_registry: Option<Arc<ServiceRegistry>>,
}

impl CapabilityRouter {
    /// Create a new capability router with self-knowledge only
    #[must_use]
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(CapabilityRegistry::new())),
            self_identity: NestGateSelfKnowledge::default(),
            service_registry: None,
        }
    }

    /// Set the service registry for capability-based discovery
    ///
    /// This enables the router to discover services dynamically instead of
    /// using hardcoded endpoints.
    #[must_use]
    pub fn with_service_registry(mut self, registry: Arc<ServiceRegistry>) -> Self {
        self.service_registry = Some(registry);
        self
    }

    /// Route capability request without knowing specific primal names
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn route_capability_request(
        &self,
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse> {
        // 1. Check if we can handle this capability ourselves
        if self
            .self_identity
            .can_handle_capability(&request.category, &request.operation)
        {
            return self.handle_locally(request);
        }

        // 2. Discover capable services through universal adapter
        let capable_services = self.discover_capable_services(&request).await?;

        if capable_services.is_empty() {
            return Err(crate::NestGateError::validation_error(format!(
                "No capable services discovered for {:?}::{}",
                request.category, request.operation
            )));
        }

        // 3. Route to best available service (no hardcoded primal names)
        let selected_service = self.select_best_service(&capable_services)?;
        self.forward_request_to_service(selected_service, request)
            .await
    }

    /// Discover services that can handle a capability (no primal hardcoding)
    async fn discover_capable_services(
        &self,
        request: &CapabilityRequest,
    ) -> Result<Vec<DiscoveredService>> {
        let registry = self.registry.read().await;
        let services = registry.find_providers(&request.category, &request.operation);

        // Filter by availability and capability match
        let mut capable_services = Vec::new();
        for service in services {
            if service.provides_capability(&request.category, &request.operation) && service.healthy
            {
                capable_services.push(service.clone());
            }
        }

        Ok(capable_services)
    }

    /// Select best service based on capability metrics (not primal identity)
    fn select_best_service<'a>(
        &self,
        services: &'a [DiscoveredService],
    ) -> Result<&'a DiscoveredService> {
        matching::select_best_by_recency(services).ok_or_else(|| {
            crate::NestGateError::internal_error("No suitable service found", "capability_routing")
        })
    }

    /// Forward request to selected service using universal protocol
    async fn forward_request_to_service(
        &self,
        service: &DiscoveredService,
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse> {
        // **EVOLUTION**: Use ServiceRegistry for dynamic discovery (no hardcoded URLs!)
        let endpoint = if let Some(registry) = &self.service_registry {
            // Try capability-based discovery first
            if let Ok(discovered_service) = registry
                .find_by_capability(&request.category.to_primal_capability())
                .await
            {
                discovered_service.url()
            } else {
                // Fallback to environment config if discovery fails
                let config = CapabilityEndpointsConfig::from_env();
                config
                    .service_endpoint()
                    .map(|s| s.to_string())
                    .ok_or_else(|| {
                        crate::NestGateError::not_found(format!(
                            "No endpoint found for capability: {:?}",
                            request.category
                        ))
                    })?
            }
        } else {
            // No registry configured - fall back to environment config only
            let config = CapabilityEndpointsConfig::from_env();
            config
                .service_endpoint()
                .map(|s| s.to_string())
                .ok_or_else(|| {
                    crate::NestGateError::not_found(
                        "No service registry configured and no environment endpoint set",
                    )
                })?
        };

        // Generic capability request - works with any primal
        self.send_universal_request(&endpoint, &service.endpoint, request)
    }

    /// Handle capability locally (NestGate's own capabilities)
    fn handle_locally(&self, request: CapabilityRequest) -> Result<CapabilityResponse> {
        // Handle storage capabilities that NestGate provides
        match request.category {
            CapabilityCategory::Storage => self.handle_storage_capability(request),
            _ => Err(crate::NestGateError::validation_error(format!(
                "Local capability not implemented: {:?}",
                request.category
            ))),
        }
    }

    /// Handle storage capabilities (NestGate's domain)
    fn handle_storage_capability(&self, request: CapabilityRequest) -> Result<CapabilityResponse> {
        let mut response_data = serde_json::Map::new();

        match request.operation.as_str() {
            "create_dataset" => {
                response_data.insert(
                    "dataset_id".to_string(),
                    Value::String("zfs-dataset-123".to_string()),
                );
                response_data.insert("status".to_string(), Value::String("created".to_string()));
            }
            "list_datasets" => {
                response_data.insert("datasets".to_string(), Value::Array(vec![]));
            }
            _ => {
                return Err(crate::NestGateError::validation_error(format!(
                    "Storage operation not implemented: {}",
                    request.operation
                )));
            }
        }

        Ok(CapabilityResponse {
            request_id: request.request_id,
            success: true,
            data: Value::Object(response_data),
            error: None,
            metadata: HashMap::new(),
            execution_time_ms: 10,
        })
    }

    /// Send universal capability request (works with any primal)
    fn send_universal_request(
        &self,
        _endpoint: &str,
        _capability_endpoint: &str,
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse> {
        // Simplified implementation - in production this would use HTTP/gRPC
        Ok(CapabilityResponse {
            request_id: request.request_id,
            success: true,
            data: Value::Object(serde_json::Map::new()),
            error: None,
            metadata: HashMap::new(),
            execution_time_ms: 50,
        })
    }
}

impl Default for CapabilityRouter {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}
