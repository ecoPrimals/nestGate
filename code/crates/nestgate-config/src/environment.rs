// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::collections::HashMap;

// Import the configuration module for concurrent-safe access
use crate::environment_config::EnvironmentConfig;

/// Environment detection and configuration for `NestGate`
///
/// Supports two primary modes:
/// 1. **Orchestration-Enhanced**: All networking, ports, and service discovery handled by orchestration module
/// 2. **Standalone**: Self-contained operation with direct network protocols
///
/// The system automatically detects the environment and configures itself accordingly.
/// **CANONICAL MODERNIZATION** - Use canonical service configuration
// Consolidation: `ServiceConfig` may move to `nestgate-types` alongside other shared service DTOs.
pub use crate::config::canonical_primary::service::ServiceConfig;

/// Operating mode for `NestGate`
#[derive(Debug, Clone, PartialEq, Eq)]
/// Operationmode
pub enum OperationMode {
    /// Standalone mode: Direct networking, self-contained
    Standalone,
    /// Orchestration-enhanced mode: Orchestration module handles networking
    OrchestrationEnhanced,
}

/// Environment detection and configuration
///
/// **MODERNIZED**: Simplified structure - removed deprecated `NetworkConfig`
/// For comprehensive network configuration, use `config::canonical_primary::domains::network::CanonicalNetworkConfig`
#[derive(Debug, Clone)]
/// Environment
pub struct Environment {
    /// Current operation mode
    pub mode: OperationMode,
    /// Service configuration
    pub service: ServiceConfig,
    /// Network bind interface (127.0.0.1 for standalone, 0.0.0.0 for orchestration)
    pub bind_interface: String,
    /// Network port
    pub port: u16,
    /// Service name for orchestration mode
    pub service_name: String,
    /// Enable service discovery
    pub discovery_enabled: bool,
    /// External service endpoints
    pub external_services: HashMap<String, String>,
}
impl Default for Environment {
    /// Returns the default instance
    fn default() -> Self {
        Self::detect()
    }
}

impl Environment {
    /// Detect the current environment and configure accordingly
    #[must_use]
    pub fn detect() -> Self {
        let mode = Self::detect_mode();
        let service = Self::detect_service_config();
        let (bind_interface, port, service_name, discovery_enabled) =
            Self::detect_network_settings(&mode);
        let external_services = Self::detect_external_services(&mode);

        Self {
            mode,
            service,
            bind_interface,
            port,
            service_name,
            discovery_enabled,
            external_services,
        }
    }

    /// Detect operation mode based on environment variables
    fn detect_mode() -> OperationMode {
        let config = EnvironmentConfig::from_env();
        if config.is_orchestration_mode() {
            OperationMode::OrchestrationEnhanced
        } else {
            OperationMode::Standalone
        }
    }

    /// Detect service configuration
    fn detect_service_config() -> ServiceConfig {
        use crate::config::canonical_primary::service::ServiceType;
        // Consolidation: ServiceId / ServiceState and related enums may live in `nestgate-types`.
        let config = EnvironmentConfig::from_env();

        let mut metadata = std::collections::HashMap::new();
        metadata.insert("environment".to_string(), config.environment());

        ServiceConfig {
            name: config.service_name(),
            service_name: config.service_name(),
            service_type: ServiceType::Api,
            metadata,
            ..Default::default()
        }
    }

    /// Detect network settings based on mode
    /// Returns: (`bind_interface`, port, `service_name`, `discovery_enabled`)
    fn detect_network_settings(mode: &OperationMode) -> (String, u16, String, bool) {
        let config = EnvironmentConfig::from_env();
        let orchestration_mode = matches!(mode, OperationMode::OrchestrationEnhanced);

        let bind_interface = config.bind_interface(orchestration_mode);
        let port = config.port();
        let service_name = config.nestgate_service_name();
        let discovery_enabled = config.discovery_enabled(orchestration_mode);

        (bind_interface, port, service_name, discovery_enabled)
    }

    /// Detect external service endpoints
    fn detect_external_services(mode: &OperationMode) -> HashMap<String, String> {
        let config = EnvironmentConfig::from_env();
        let orchestration_mode = matches!(mode, OperationMode::OrchestrationEnhanced);
        config.external_services(orchestration_mode)
    }
}
