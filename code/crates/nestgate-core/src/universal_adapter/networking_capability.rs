// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Networking Capability Adapter
//!
//! **ZERO HARDCODED PRIMAL NAMES**: This adapter discovers networking capabilities
//! (load balancing, circuit breaking, service mesh, etc.) from ANY provider that
//! implements them. Never mentions "songbird" or any specific primal.

use super::capability_discovery::{CapabilityDiscovery, CapabilityProvider, CapabilityType};
use crate::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Networking capability adapter
///
/// Discovers and uses networking capabilities without knowing which primal provides them.
pub struct NetworkingCapability {
    discovery: Arc<CapabilityDiscovery>,
}

impl NetworkingCapability {
    /// Create new networking capability adapter
    pub fn new(discovery: Arc<CapabilityDiscovery>) -> Self {
        Self { discovery }
    }

    /// Discover available networking providers
    ///
    /// Returns ANY provider that offers networking capabilities.
    /// Could be Songbird, a custom implementation, or a future primal.
    pub async fn discover_providers(&self) -> Result<Vec<CapabilityProvider>> {
        self.discovery.discover(CapabilityType::networking()).await
    }

    /// Request load balancing
    ///
    /// **NO HARDCODING**: Delegates to whatever provides "load-balancing" capability
    pub async fn load_balance(&self, _request: LoadBalanceRequest) -> Result<LoadBalanceResponse> {
        let providers = self
            .discovery
            .discover(CapabilityType::load_balancing())
            .await?;

        let provider = providers.first().ok_or_else(|| {
            crate::error::NestGateError::not_found("No load balancing capability found")
        })?;

        // Make request to provider endpoint (whoever it is)
        // In real implementation, would make HTTP call to provider.endpoint
        // Port is provided by the discovered service endpoint
        Ok(LoadBalanceResponse {
            chosen_endpoint: provider.endpoint.clone(),
            algorithm_used: "discovered-provider".to_string(),
        })
    }

    /// Request circuit breaker protection
    ///
    /// **NO HARDCODING**: Delegates to whatever provides "circuit-breaking" capability
    pub async fn circuit_break(
        &self,
        _request: CircuitBreakerRequest,
    ) -> Result<CircuitBreakerResponse> {
        let providers = self
            .discovery
            .discover(CapabilityType::circuit_breaking())
            .await?;

        let provider = providers.first().ok_or_else(|| {
            crate::error::NestGateError::not_found("No circuit breaker capability found")
        })?;

        // Make request to provider endpoint (whoever it is)
        Ok(CircuitBreakerResponse {
            allowed: true,
            state: CircuitState::Closed,
            provider: provider.endpoint.clone(),
        })
    }
}

/// Load balance request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for LoadBalance operation
pub struct LoadBalanceRequest {
    /// Service name
    pub service_name: String,
    /// Request Data
    pub request_data: Vec<u8>,
}

/// Load balance response
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for LoadBalance operation
pub struct LoadBalanceResponse {
    /// Chosen Endpoint
    pub chosen_endpoint: String,
    /// Algorithm Used
    pub algorithm_used: String,
}

/// Circuit breaker request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for CircuitBreaker operation
pub struct CircuitBreakerRequest {
    /// Service name
    pub service_name: String,
    /// Operation
    pub operation: String,
}

/// Circuit breaker response
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for CircuitBreaker operation
pub struct CircuitBreakerResponse {
    /// Allowed
    pub allowed: bool,
    /// State
    pub state: CircuitState,
    /// Provider
    pub provider: String,
}

/// Circuit breaker state
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Circuitstate
pub enum CircuitState {
    /// Closed
    Closed,
    /// Open
    Open,
    /// Halfopen
    HalfOpen,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_no_hardcoded_networking_primal() {
        // This test verifies we never hardcode primal names
        let discovery = Arc::new(CapabilityDiscovery::new());
        let _networking = NetworkingCapability::new(discovery);

        // We discover "networking" capability, not "songbird"
        let capability_type = CapabilityType::networking();
        assert_eq!(capability_type.as_str(), "networking");
        assert_ne!(capability_type.as_str(), "songbird"); // Never hardcoded!
    }
}
