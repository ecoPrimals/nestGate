// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **INTEGRATION CONFIGURATIONS**
//!
//! Consolidates all external service and protocol integrations including:
//! - External service configurations
//! - Protocol-specific settings (HTTP, gRPC, WebSocket)
//! - Ecosystem integrations
//! - Development and testing integrations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ==================== INTEGRATION CONFIGURATIONS ====================

/// **INTEGRATION CONFIGURATIONS**
///
/// Consolidates all external service and protocol integrations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConsolidatedIntegrationConfigs {
    /// External service integrations
    pub external_services: HashMap<String, ExternalServiceConfig>,

    /// Protocol-specific configurations
    pub protocols: ProtocolConfigs,

    /// Ecosystem integrations (Management, etc.)
    pub ecosystem: EcosystemConfig,

    /// Development and testing integrations
    pub development: DevelopmentIntegrationConfig,
}

// ==================== EXTERNAL SERVICES ====================

/// Configuration for external service integrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalServiceConfig {
    /// Service endpoint URL
    pub endpoint: String,

    /// Authentication configuration
    pub auth: ExternalAuthConfig,

    /// Connection settings
    pub connection: ExternalConnectionConfig,

    /// Retry configuration
    pub retry: ExternalRetryConfig,

    /// Health check configuration
    pub health_check: ExternalHealthCheckConfig,
}

/// External authentication configuration (reserved for future use)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalAuthConfig {}

/// External connection configuration (reserved for future use)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalConnectionConfig {}

/// External retry configuration (reserved for future use)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalRetryConfig {}

/// External health check configuration (reserved for future use)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalHealthCheckConfig {}

// ==================== PROTOCOL CONFIGURATIONS ====================

/// Container for all protocol-specific configurations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(clippy::zero_sized_map_values)]
pub struct ProtocolConfigs {
    /// HTTP/HTTPS configuration
    pub http: HttpProtocolConfig,

    /// gRPC configuration
    pub grpc: GrpcProtocolConfig,

    /// WebSocket configuration
    pub websocket: WebSocketProtocolConfig,

    /// Custom protocol configurations
    pub custom: HashMap<String, CustomProtocolConfig>,
}

/// HTTP protocol configuration (reserved for future use)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HttpProtocolConfig {}

/// gRPC protocol configuration (reserved for future use)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GrpcProtocolConfig {}

/// WebSocket protocol configuration (reserved for future use)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebSocketProtocolConfig {}

/// Custom protocol configuration (reserved for future use)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomProtocolConfig {}

// ==================== ECOSYSTEM CONFIGURATION ====================

/// Configuration for ecosystem-wide settings and integrations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EcosystemConfig {
    /// Management integration settings
    pub management: BiomeOsIntegrationConfig,

    /// Primal ecosystem settings
    pub primal_ecosystem: PrimalEcosystemConfig,

    /// Service discovery settings
    pub service_discovery: ServiceDiscoveryConfig,

    /// Capability-based routing
    pub capability_routing: CapabilityRoutingConfig,
}

/// Ecosystem platform integration configuration (reserved for future use).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BiomeOsIntegrationConfig {}

/// Primal ecosystem configuration (reserved for future use)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrimalEcosystemConfig {}

/// Service discovery configuration (reserved for future use)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceDiscoveryConfig {}

/// Capability routing configuration (reserved for future use)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CapabilityRoutingConfig {}

// ==================== DEVELOPMENT INTEGRATION ====================

/// Development and testing integration configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DevelopmentIntegrationConfig {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_configs_default() {
        let config = ConsolidatedIntegrationConfigs::default();
        assert!(config.external_services.is_empty());
    }

    #[test]
    fn test_protocol_configs() {
        let protocols = ProtocolConfigs::default();
        assert!(protocols.custom.is_empty());
    }
}
