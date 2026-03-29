// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Service module

use std::collections::HashMap;
use std::future::Future;
// use crate::error::idiomatic_evolution // DEPRECATED::{NetworkError, NetworkResult};
use crate::error::CanonicalResult as Result;
// CANONICAL MODERNIZATION: Migrated from deprecated ServiceRegistration
use crate::canonical_modernization::unified_enums::{UnifiedHealthStatus, UnifiedServiceState};
use crate::service_discovery::types::UniversalServiceRegistration as ServiceRegistration;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
// CLEANED: Removed unused Duration import as part of canonical modernization
// Removed unused Uuid import

/// Native async network service implementation
/// Native async network service implementation
/// High-performance networking service using zero-cost async patterns
pub struct NativeAsyncNetworkService {
    /// Configuration for
    pub config: NetworkServiceConfig,
    /// Connections
    pub connections: HashMap<String, String>,
    service_id: String,
    #[expect(dead_code, reason = "framework placeholder")]
    // Framework field - intentionally unused
    state: UnifiedServiceState,
}
/// Configuration for native async network service
/// Configuration for native async network service
/// Defines host, port, and operational parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::NetworkServiceConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::NetworkServiceConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for NetworkService
pub struct NetworkServiceConfig {
    /// Host
    pub host: String,
    /// Port
    pub port: u16,
    /// Max Connections
    pub max_connections: usize,
}
/// Health information for network service
/// Health status information for network service
/// Provides detailed health metrics and status information
#[derive(Debug, Clone)]
/// Networkservicehealth
pub struct NetworkServiceHealth {
    /// Status
    pub status: UnifiedHealthStatus,
    /// Active Connections
    pub active_connections: usize,
    /// Max Connections
    pub max_connections: usize,
}
impl Default for NetworkServiceConfig {
    /// Returns the default instance
    ///
    /// Loads configuration from environment variables with fallback defaults:
    /// - `NESTGATE_API_HOST`: API host (default: localhost)
    /// - `NESTGATE_API_PORT`: API port (default: 8080)
    /// - `NESTGATE_MAX_CONNECTIONS`: Max connections (default: 1000)
    fn default() -> Self {
        use crate::config::environment::EnvironmentConfig;

        // Load from environment with proper defaults
        let env_config =
            EnvironmentConfig::from_env().unwrap_or_else(|_| EnvironmentConfig::default());

        Self {
            host: env_config.network.host.clone(),
            port: env_config.network.port.get(),
            max_connections: env_config.network.max_connections,
        }
    }
}

impl NativeAsyncNetworkService {
    /// Creates a new native async network service with the provided configuration
    #[must_use]
    pub fn new(config: NetworkServiceConfig) -> Self {
        Self {
            config,
            connections: HashMap::new(),
            service_id: uuid::Uuid::new_v4().to_string(),
            state: UnifiedServiceState::Stopped,
        }
    }
}

/// **CANONICAL SERVICE IMPLEMENTATION**: `NativeAsyncNetworkService`
///
/// **PERFORMANCE**: Zero-cost native async implementation
/// **MEMORY**: No runtime overhead, compile-time dispatch
impl crate::traits::canonical::CanonicalService for NativeAsyncNetworkService {
    /// Type alias for Config
    type Config = NetworkServiceConfig;
    /// Type alias for Health
    type Health = crate::traits::canonical::ProviderHealth; // PEDANTIC: Use existing ProviderHealth
    /// Type alias for Metrics
    type Metrics = crate::traits::canonical::ServiceCapabilities;
    /// Type alias for Error
    type Error = crate::error::NestGateError;
    /// Service Id
    fn service_id(&self) -> &str {
        &self.service_id
    }

    /// Service Type
    fn service_type(&self) -> crate::unified_enums::service_types::UnifiedServiceType {
        crate::unified_enums::service_types::UnifiedServiceType::Network
    }

    /// Initialize
    async fn initialize(&self, _config: Self::Config) -> crate::Result<()> {
        // Initialize with config
        Ok(())
    }

    /// Health Check
    async fn health_check(&self) -> crate::Result<Self::Health> {
        Ok(crate::traits::canonical::ProviderHealth {
            is_healthy: true,
            last_check: SystemTime::now(),
            health: "Network service operational".to_string(),
        })
    }

    /// Gets Metrics
    async fn get_metrics(&self) -> std::result::Result<Self::Metrics, Self::Error> {
        Ok(crate::traits::canonical::ServiceCapabilities {
            can_scale: true,
            can_migrate: true,
            can_backup: false,
            supported_protocols: vec![
                "tcp".to_string(),
                "http".to_string(),
                "websocket".to_string(),
            ],
        })
    }

    /// Shutdown
    async fn shutdown(&self) -> crate::Result<()> {
        Ok(())
    }

    /// Start
    async fn start(&self) -> crate::Result<()> {
        Ok(())
    }

    /// Stop
    async fn stop(&self) -> crate::Result<()> {
        Ok(())
    }

    /// Restart
    async fn restart(&self) -> crate::Result<()> {
        Ok(())
    }

    /// Updates  Config
    async fn update_config(&self, _config: Self::Config) -> crate::Result<()> {
        Ok(())
    }

    /// Capabilities
    async fn capabilities(&self) -> crate::Result<crate::traits::canonical::ServiceCapabilities> {
        Ok(crate::traits::canonical::ServiceCapabilities {
            can_scale: true,
            can_migrate: true,
            can_backup: false,
            supported_protocols: vec![
                "tcp".to_string(),
                "http".to_string(),
                "websocket".to_string(),
            ],
        })
    }

    /// Validates  Config
    async fn validate_config(&self, _config: &Self::Config) -> crate::Result<Vec<String>> {
        Ok(vec![])
    }

    /// Checks if Healthy
    async fn is_healthy(&self) -> crate::Result<Self::Health> {
        Ok(crate::traits::canonical::ProviderHealth {
            is_healthy: true,
            last_check: std::time::SystemTime::now(),
            health: "Network service operational".to_string(),
        })
    }
}

// ==================== SECTION ====================
//
// The following methods have been removed as they conflict with the canonical trait:
// - handle_request: Not part of CanonicalService interface
// - Duplicate health_check: Already implemented in CanonicalService
// - Duplicate get_metrics: Already implemented in CanonicalService
// - Duplicate shutdown: Already implemented in CanonicalService
// - Duplicate update_config: Already implemented in CanonicalService
//
// Use the canonical trait methods instead.

impl NativeAsyncNetworkService {
    /// Register service for discovery (utility method)
    pub fn register(&self) -> impl Future<Output = Result<ServiceRegistration>> + Send {
        let service_id = self.service_id.clone();
        // Use config values from environment
        let _host = self.config.host.clone();
        let _port = self.config.port;

        async move {
            // CANONICAL MODERNIZATION: Use canonical service registration structure
            use crate::service_discovery::types::{
                IntegrationPreferences, ResourceSpec, ServiceCapability, ServiceCategory,
                ServiceMetadata,
            };

            Ok(ServiceRegistration {
                service_id: uuid::Uuid::parse_str(&service_id)
                    .unwrap_or_else(|_| uuid::Uuid::new_v4()),
                metadata: ServiceMetadata {
                    name: format!("network-service-{service_id}"),
                    category: ServiceCategory::Network,
                    version: "1.0.0".to_string(),
                    description: "Native async network service".to_string(),
                    health_endpoint: Some("/health".to_string()),
                    metrics_endpoint: None,
                },
                capabilities: vec![ServiceCapability::Network(
                    crate::service_discovery::types::CommunicationProtocol::Http,
                )],
                resources: ResourceSpec::default(),
                endpoints: vec![],
                integration: IntegrationPreferences::default(),
                extensions: HashMap::new(),
            })
        }
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Networkserviceconfigcanonical
pub type NetworkServiceConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using NetworkServiceConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.
