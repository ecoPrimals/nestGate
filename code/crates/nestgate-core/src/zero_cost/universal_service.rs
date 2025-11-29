use crate::error::NestGateError;
use std::collections::HashMap;
/// **ZERO-COST UNIVERSAL SERVICE TRAIT**
///
/// This module provides a high-performance replacement for the async_trait-based
/// UniversalService trait, using native async methods and const generics for
/// compile-time optimization.
///
/// **PERFORMANCE BENEFITS**:
/// - Native async methods (no Future boxing)
/// - Compile-time specialization through const generics
/// - Direct method dispatch (no vtable overhead)
/// - Monomorphized code generation for optimal performance
///
/// **REPLACES**: `crate::traits::canonical::CanonicalService`
use crate::Result;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;
// ==================== SECTION ====================

/// **Zero-cost universal service trait**
///
/// Replaces the async_trait-based UniversalService with native async methods
/// and compile-time configuration through const generics.
pub trait ZeroCostUniversalService: Send + Sync + 'static {
    /// Service configuration type (must be cloneable and thread-safe)
    type Config: Clone + Send + Sync + 'static;
    /// Health status type (must be cloneable and thread-safe)  
    type Health: Clone + Send + Sync + 'static;

    /// Service metadata type
    type Metadata: Clone + Send + Sync + 'static;

    // ==================== LIFECYCLE METHODS ====================

    /// Start the service - native async, no boxing overhead
    fn start(
        &mut self,
        config: Self::Config,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Stop the service gracefully - native async
    fn stop(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Restart the service - native async with default implementation
    fn restart(
        &mut self,
        config: Self::Config,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        async move {
            self.stop().await?;
            self.start(config).await
        }
    }

    // ==================== HEALTH AND STATUS ====================

    /// Get service health status - native async
    fn health_check(&self) -> impl std::future::Future<Output = Self::Health> + Send;

    /// Get service metadata - direct method call (no async overhead)
    fn metadata(&self) -> Self::Metadata;

    /// Get service unique identifier - direct access
    fn service_id(&self) -> Uuid;

    /// Get service name - direct access
    fn service_name(&self) -> &str;

    // ==================== CONFIGURATION ====================

    /// Get current service configuration - direct access
    fn current_config(&self) -> &Self::Config;

    /// Update service configuration - native async
    fn update_config(
        &mut self,
        config: Self::Config,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Validate configuration - native async with default implementation
    fn validate_config(
        &self,
        _config: &Self::Config,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        async move {
            // Default implementation accepts all configs
            // Override in implementations that need validation
            Ok(())
        }
    }
}

// ==================== SECTION ====================

/// **Zero-cost discoverable service trait**
///
/// Extension trait for services that support dynamic discovery
pub trait ZeroCostDiscoverableService: ZeroCostUniversalService {
    /// Service capabilities type
    type Capabilities: Clone + Send + Sync + 'static;
    /// Get service capabilities - direct access
    fn capabilities(&self) -> Self::Capabilities;

    /// Check if service supports a specific capability - direct method call
    fn supports_capability(&self, capability: &str) -> bool;

    /// Register service for discovery - native async
    fn register_for_discovery(&self) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Unregister service from discovery - native async
    fn unregister_from_discovery(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}

/// **Zero-cost configurable service trait**
///
/// Extension trait for services with advanced configuration capabilities
pub trait ZeroCostConfigurableService: ZeroCostUniversalService {
    /// Configuration schema type
    type Schema: Clone + Send + Sync + 'static;
    /// Get configuration schema - direct access
    fn config_schema(&self) -> Self::Schema;

    /// Apply configuration changes - native async
    fn apply_config_changes(
        &mut self,
        changes: serde_json::Value,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Export current configuration - native async
    fn export_config(&self) -> impl std::future::Future<Output = Result<serde_json::Value>> + Send;
}

// ==================== SECTION ====================

/// **Zero-cost service health status**
///
/// Optimized health status representation for zero-cost services
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Zerocostservicehealth
pub enum ZeroCostServiceHealth {
    /// Service is healthy and operational
    Healthy {
        /// Last health check timestamp
        last_check: SystemTime,
        /// Optional health metrics
        metrics: Option<HealthMetrics>,
    },
    /// Service is degraded but functional
    Degraded {
        /// Reason for degradation
        reason: String,
        /// Severity level (1-10)
        severity: u8,
        /// Last health check timestamp
        last_check: SystemTime,
    },
    /// Service is unhealthy
    Unhealthy {
        /// Error description
        error: String,
        /// Last health check timestamp
        last_check: SystemTime,
        /// Recovery suggestions
        recovery_hint: Option<String>,
    },
    /// Service status is unknown
    Unknown {
        /// Reason why status is unknown
        reason: String,
    },
}
/// **Health metrics for zero-cost services**
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Healthmetrics
pub struct HealthMetrics {
    /// CPU usage percentage (0.0-100.0)
    pub cpu_usage: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Number of active connections
    pub active_connections: usize,
    /// Request rate (requests per second)
    pub request_rate: f64,
    /// Error rate (errors per second)
    pub error_rate: f64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
}
// ==================== SECTION ====================

/// **Zero-cost service metadata**
///
/// Comprehensive metadata for zero-cost services
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zerocostservicemetadata
pub struct ZeroCostServiceMetadata {
    /// Service unique identifier
    pub service_id: Uuid,
    /// Service name
    pub name: String,
    /// Service version
    pub version: String,
    /// Service description
    pub description: Option<String>,
    /// Service tags for categorization
    pub tags: Vec<String>,
    /// Service endpoints
    pub endpoints: Vec<String>,
    /// Service dependencies
    pub dependencies: Vec<String>,
    /// Service startup time
    pub startup_time: SystemTime,
    /// Custom properties
    pub properties: std::collections::HashMap<String, String>,
}
// ==================== SECTION ====================

/// **Create default health metrics**
pub fn default_health_metrics() -> HealthMetrics {
    HealthMetrics {
        cpu_usage: 0.0,
        memory_usage: 0,
        active_connections: 0,
        request_rate: 0.0,
        error_rate: 0.0,
        avg_response_time_ms: 0.0,
    }
}
/// **Create healthy status with current timestamp**
pub fn healthy_status() -> ZeroCostServiceHealth {
    ZeroCostServiceHealth::Healthy {
        last_check: SystemTime::now(),
        metrics: Some(default_health_metrics()),
    }
}
/// **Create degraded status with reason**
pub fn degraded_status(reason: String, severity: u8) -> ZeroCostServiceHealth {
    ZeroCostServiceHealth::Degraded {
        reason,
        severity: severity.min(10), // Cap at 10
        last_check: SystemTime::now(),
    }
}
/// **Create unhealthy status with error**
pub fn unhealthy_status(error: String, recovery_hint: Option<String>) -> ZeroCostServiceHealth {
    ZeroCostServiceHealth::Unhealthy {
        error,
        last_check: SystemTime::now(),
        recovery_hint,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// **Mock zero-cost service for testing**
    struct MockZeroCostService {
        id: Uuid,
        name: String,
        config: MockConfig,
        running: bool,
    }

    #[derive(Debug, Clone)]
    struct MockConfig {
        timeout: Duration,
        max_connections: usize,
    }

    impl ZeroCostUniversalService for MockZeroCostService {
        /// Type alias for Config
        type Config = MockConfig;
        /// Type alias for Health
        type Health = ZeroCostServiceHealth;
        /// Type alias for Metadata
        type Metadata = ZeroCostServiceMetadata;

        /// Start
        async fn start(&mut self, config: Self::Config) -> Result<()> {
            self.config = config;
            self.running = true;
            Ok(())
        }

        /// Stop
        async fn stop(&mut self) -> Result<()> {
            self.running = false;
            Ok(())
        }

        /// Health Check
        async fn health_check(&self) -> Self::Health {
            if self.running {
                healthy_status()
            } else {
                unhealthy_status(
                    "Service not running".to_string(),
                    Some("Call start()".to_string()),
                )
            }
        }

        /// Metadata
        fn metadata(&self) -> Self::Metadata {
            ZeroCostServiceMetadata {
                service_id: self.id,
                name: self.name.clone(),
                version: "1.0.0".to_string(),
                description: Some("Mock service for testing".to_string()),
                tags: vec!["test".to_string(), "mock".to_string()],
                endpoints: vec![],
                dependencies: vec![],
                startup_time: SystemTime::now(),
                properties: std::collections::HashMap::new(),
            }
        }

        /// Service Id
        fn service_id(&self) -> Uuid {
            self.id
        }

        /// Service Name
        fn service_name(&self) -> &str {
            &self.name
        }

        /// Current Config
        fn current_config(&self) -> &Self::Config {
            &self.config
        }

        /// Updates  Config
        async fn update_config(&mut self, config: Self::Config) -> Result<()> {
            self.config = config;
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_zero_cost_service_lifecycle() {
        let mut service = MockZeroCostService {
            id: Uuid::new_v4(),
            name: "test-service".to_string(),
            config: MockConfig {
                timeout: Duration::from_secs(30),
                max_connections: 100,
            },
            running: false,
        };

        // Test starting service
        let config = MockConfig {
            timeout: Duration::from_secs(60),
            max_connections: 200,
        };
        service.start(config.clone()).await.unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {e:?}"),
            )
            .into());
        );
        assert_eq!(service.current_config().max_connections, 200);

        // Test health check
        let health = service.health_check().await;
        assert!(matches!(health, ZeroCostServiceHealth::Healthy { .. }));

        // Test stopping service
        service.stop().await.map_err(|e| {
            tracing::error!("Failed to stop service: {:?}", e);
            NestGateError::internal_error(
                location: Some("universal_service.rs:417".to_string()),
                location: Some("stop operation".to_string())}
        )?;
        let health = service.health_check().await;
        assert!(matches!(health, ZeroCostServiceHealth::Unhealthy { .. }));
    }

    #[tokio::test]
    async fn test_compatibility_bridge() {
        let mut service = MockZeroCostService {
            id: Uuid::new_v4(),
            name: "compat-test".to_string(),
            config: MockConfig {
                timeout: Duration::from_secs(30),
                max_connections: 100,
            },
            running: false,
        };

        // Test compatibility methods
        service.start_compat().await.unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {e:?}"),
            )
            .into());
        );
        let health = service.health_check_compat().await.unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {e:?}"),
            )
            .into());
        );
        assert!(matches!(health, ZeroCostServiceHealth::Healthy { .. }));

        service.stop_compat().await.unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {e:?}"),
            )
            .into());
        );
    }
}
