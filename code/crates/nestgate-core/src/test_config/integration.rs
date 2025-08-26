use std::collections::HashMap;
///
/// This module provides configuration for integration testing including external services,
/// databases, message queues, service mesh, and service discovery.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ==================== INTEGRATION TESTING CONFIGURATION ====================

/// **Unified integration testing configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestIntegrationConfig {
    /// External service configuration
    pub external_services: ExternalServiceConfig,
    /// Database configuration
    pub database: DatabaseConfig,
    /// Message queue configuration
    pub message_queue: MessageQueueConfig,
    /// Service mesh configuration
    pub service_mesh: ServiceMeshConfig,
}

/// **External service configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalServiceConfig {
    /// Enable external service tests
    pub enabled: bool,
    /// Service endpoints
    pub endpoints: HashMap<String, String>,
    /// Authentication configuration
    pub auth: ServiceAuthConfig,
    /// Service timeouts
    pub timeouts: HashMap<String, Duration>,
}

/// **Service authentication configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceAuthConfig {
    /// OAuth configuration
    pub oauth: Option<OAuthConfig>,
    /// API keys
    pub api_keys: HashMap<String, String>,
    /// Basic auth credentials
    pub basic_auth: HashMap<String, String>,
}

/// **OAuth configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OAuthConfig {
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Token endpoint
    pub token_endpoint: String,
}

/// **Database configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseConfig {
    /// Enable database tests
    pub enabled: bool,
    /// Database connections
    pub connections: HashMap<String, String>,
    /// Migration configuration
    pub migrations: MigrationConfig,
    /// Test data setup
    pub test_data: HashMap<String, String>,
}

/// **Migration configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MigrationConfig {
    /// Enable migrations
    pub enabled: bool,
    /// Migration directory
    pub migration_dir: String,
    /// Migration timeout
    pub timeout: Duration,
}

/// **Message queue configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageQueueConfig {
    /// Enable message queue tests
    pub enabled: bool,
    /// Queue connections
    pub connections: HashMap<String, String>,
    /// Test topics/queues
    pub test_queues: Vec<String>,
    /// Message timeout
    pub message_timeout: Duration,
}

/// **Service mesh configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceMeshConfig {
    /// Enable service mesh tests
    pub enabled: bool,
    /// Service discovery configuration
    pub discovery: ServiceDiscoveryConfig,
    /// Load balancing configuration
    pub load_balancing: LoadBalancingConfig,
    /// Health check configuration
    pub health_checks: HealthCheckConfig,
}

/// **Service discovery configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceDiscoveryConfig {
    /// Enable service discovery
    pub enabled: bool,
    /// Discovery endpoints
    pub endpoints: Vec<String>,
    /// Discovery timeout
    pub timeout: Duration,
}

/// **Load balancing configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadBalancingConfig {
    /// Enable load balancing tests
    pub enabled: bool,
    /// Load balancing strategy
    pub strategy: String,
    /// Health check interval
    pub health_check_interval: Duration,
}

/// **Health check configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthCheckConfig {
    /// Enable health checks
    pub enabled: bool,
    /// Health check endpoints
    pub endpoints: HashMap<String, String>,
    /// Check interval
    pub interval: Duration,
}

/// **Circuit breaker configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CircuitBreakerConfig {
    /// Enable circuit breaker tests
    pub enabled: bool,
    /// Failure threshold
    pub failure_threshold: u32,
    /// Recovery timeout
    pub recovery_timeout: Duration,
}

/// **Retry configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetryConfig {
    /// Enable retry tests
    pub enabled: bool,
    /// Maximum retries
    pub max_retries: u32,
    /// Retry delay
    pub retry_delay: Duration,
}
