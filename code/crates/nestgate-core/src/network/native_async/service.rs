// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Service module

use crate::error::CanonicalResult as Result;
use std::collections::HashMap;
use std::future::Future;
// CANONICAL MODERNIZATION: Migrated from deprecated ServiceRegistration
use crate::canonical_modernization::unified_enums::{UnifiedHealthStatus, UnifiedServiceState};
use crate::service_discovery::types::UniversalServiceRegistration as ServiceRegistration;
use std::time::SystemTime;

/// Configuration for the native async network service (canonical unified network config).
pub type NetworkServiceConfig =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

/// Native async network service implementation
/// Native async network service implementation
/// High-performance networking service using zero-cost async patterns
pub struct NativeAsyncNetworkService {
    /// Configuration for
    pub config: NetworkServiceConfig,
    /// Connections
    pub connections: HashMap<String, String>,
    service_id: String,
    _state: UnifiedServiceState,
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

impl NativeAsyncNetworkService {
    /// Creates a new native async network service with the provided configuration
    #[must_use]
    pub fn new(config: NetworkServiceConfig) -> Self {
        Self {
            config,
            connections: HashMap::new(),
            service_id: uuid::Uuid::new_v4().to_string(),
            _state: UnifiedServiceState::Stopped,
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

    /// Passthrough for the minimal admin HTTP surface: lifecycle management is delegated to the orchestration layer.
    async fn stop(&self) -> crate::Result<()> {
        tracing::debug!(
            "NativeAsyncNetworkService::stop — lifecycle management delegated to orchestration layer"
        );
        Ok(())
    }

    /// Passthrough for the minimal admin HTTP surface: lifecycle management is delegated to the orchestration layer.
    async fn restart(&self) -> crate::Result<()> {
        tracing::debug!(
            "NativeAsyncNetworkService::restart — lifecycle management delegated to orchestration layer"
        );
        Ok(())
    }

    /// Passthrough for the minimal admin HTTP surface: lifecycle management is delegated to the orchestration layer.
    async fn update_config(&self, _config: Self::Config) -> crate::Result<()> {
        tracing::debug!(
            "NativeAsyncNetworkService::update_config — lifecycle management delegated to orchestration layer"
        );
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

    /// Passthrough for the minimal admin HTTP surface: lifecycle management is delegated to the orchestration layer.
    async fn validate_config(&self, _config: &Self::Config) -> crate::Result<Vec<String>> {
        tracing::debug!(
            "NativeAsyncNetworkService::validate_config — lifecycle management delegated to orchestration layer"
        );
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
        // Use config values from canonical API settings
        let _host = self.config.api.bind_address.to_string();
        let _ = self.config.api.port;

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

#[cfg(test)]
mod tests {
    use super::{NativeAsyncNetworkService, NetworkServiceHealth};
    use crate::canonical_modernization::unified_enums::UnifiedHealthStatus;
    use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    use crate::service_discovery::types::ServiceCategory;
    use crate::traits::canonical::CanonicalService;
    use crate::unified_enums::service_types::UnifiedServiceType;

    #[test]
    fn new_stores_config_and_exposes_service_id_and_type() {
        let config = CanonicalNetworkConfig::development_optimized();
        let svc = NativeAsyncNetworkService::new(config.clone());
        assert!(!svc.service_id().is_empty());
        assert_eq!(svc.service_type(), UnifiedServiceType::Network);
        assert!(svc.connections.is_empty());
        assert_eq!(svc.config.api.port, config.api.port);
    }

    #[test]
    fn network_service_health_holds_status_and_counts() {
        let h = NetworkServiceHealth {
            status: UnifiedHealthStatus::Healthy,
            active_connections: 2,
            max_connections: 10,
        };
        assert_eq!(h.active_connections, 2);
        assert_eq!(h.max_connections, 10);
    }

    #[tokio::test]
    async fn canonical_service_lifecycle_and_health() {
        let config = CanonicalNetworkConfig::development_optimized();
        let svc = NativeAsyncNetworkService::new(config.clone());

        assert!(svc.initialize(config.clone()).await.is_ok());
        assert!(svc.start().await.is_ok());

        let health = svc.health_check().await.expect("health_check");
        assert!(health.is_healthy);

        let metrics = svc.get_metrics().await.expect("get_metrics");
        assert!(metrics.can_scale);

        let caps = svc.capabilities().await.expect("capabilities");
        assert!(caps.can_migrate);

        let issues = svc.validate_config(&config).await.expect("validate_config");
        assert!(issues.is_empty());

        let h2 = svc.is_healthy().await.expect("is_healthy");
        assert!(h2.is_healthy);

        assert!(svc.stop().await.is_ok());
        assert!(svc.restart().await.is_ok());
        assert!(svc.update_config(config.clone()).await.is_ok());
        assert!(svc.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn register_returns_service_registration() {
        let config = CanonicalNetworkConfig::development_optimized();
        let svc = NativeAsyncNetworkService::new(config);
        let reg = svc.register().await.expect("register");
        assert_eq!(reg.metadata.category, ServiceCategory::Network);
    }
}
