//! **Canonical Connection Pool Configuration**
//!
//! This is the single source of truth for connection pool configuration in NestGate.
//! All connection pool-related settings are consolidated here.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **THE** Canonical Connection Pool Configuration
///
/// Consolidates all connection pooling settings from across the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolConfig {
    /// Enable connection pooling
    pub enabled: bool,
    /// Initial pool size
    pub initial_size: u32,
    /// Maximum pool size
    pub max_size: u32,
    /// Minimum idle connections
    pub min_idle: u32,
    /// Maximum idle connections
    pub max_idle: u32,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Idle timeout
    pub idle_timeout: Duration,
    /// Maximum connection lifetime
    pub max_lifetime: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Retry configuration for connections
    pub retry_config: ConnectionRetryConfig,
    /// Pool monitoring
    pub monitoring: PoolMonitoringConfig,
    /// Connection validation
    pub validation: ConnectionValidationConfig,
    /// Connection lifecycle
    pub lifecycle: ConnectionLifecycleConfig,
    /// Load balancing (optional)
    pub load_balancing: Option<ConnectionLoadBalancingConfig>,
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            initial_size: 5,
            max_size: 20,
            min_idle: 2,
            max_idle: 10,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600), // 10 minutes
            max_lifetime: Duration::from_secs(1800), // 30 minutes
            health_check_interval: Duration::from_secs(60),
            retry_config: ConnectionRetryConfig::default(),
            monitoring: PoolMonitoringConfig::default(),
            validation: ConnectionValidationConfig::default(),
            lifecycle: ConnectionLifecycleConfig::default(),
            load_balancing: None,
        }
    }
}

// ==================== Supporting Types ====================

/// Connection retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionRetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Initial retry interval
    pub initial_interval: Duration,
    /// Maximum retry interval
    pub max_interval: Duration,
    /// Backoff multiplier
    pub multiplier: f64,
    /// Whether to add jitter to retry intervals
    pub jitter: bool,
    /// Whether to retry on connection errors
    pub retry_on_connection_error: bool,
    /// Whether to retry on timeout
    pub retry_on_timeout: bool,
}

impl Default for ConnectionRetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_interval: Duration::from_millis(100),
            max_interval: Duration::from_secs(30),
            multiplier: 2.0,
            jitter: true,
            retry_on_connection_error: true,
            retry_on_timeout: true,
        }
    }
}

/// Pool monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolMonitoringConfig {
    /// Whether monitoring is enabled
    pub enabled: bool,
    /// Monitoring interval duration
    pub interval: Duration,
    /// List of metrics to collect
    pub metrics: Vec<PoolMetric>,
    /// Performance thresholds for alerting
    pub thresholds: PoolThresholds,
}

impl Default for PoolMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(30),
            metrics: vec![
                PoolMetric::ActiveConnections,
                PoolMetric::IdleConnections,
                PoolMetric::TotalConnections,
            ],
            thresholds: PoolThresholds::default(),
        }
    }
}

/// Pool metrics to collect
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PoolMetric {
    /// Number of currently active connections
    ActiveConnections,
    /// Number of idle connections in the pool
    IdleConnections,
    /// Total number of connections (active + idle)
    TotalConnections,
    /// Time taken to acquire a connection from the pool
    ConnectionAcquisitionTime,
    /// Percentage of pool capacity currently in use
    ConnectionUtilization,
    /// Rate at which new connections are being created
    ConnectionCreationRate,
    /// Rate at which connection attempts are failing
    ConnectionFailureRate,
}

/// Pool performance thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolThresholds {
    /// Maximum acceptable time to acquire a connection
    pub max_acquisition_time: Duration,
    /// Maximum acceptable pool utilization (0.0 to 1.0)
    pub max_utilization: f64,
    /// Maximum acceptable failure rate (0.0 to 1.0)
    pub max_failure_rate: f64,
    /// Minimum number of healthy connections required
    pub min_healthy_connections: u32,
}

impl Default for PoolThresholds {
    fn default() -> Self {
        Self {
            max_acquisition_time: Duration::from_millis(500),
            max_utilization: 0.9,  // 90%
            max_failure_rate: 0.1, // 10%
            min_healthy_connections: 1,
        }
    }
}

/// Connection validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionValidationConfig {
    /// Whether connection validation is enabled
    pub enabled: bool,
    /// Strategy to use for validating connections
    pub strategy: ValidationStrategy,
    /// Timeout duration for validation operations
    pub validation_timeout: Duration,
    /// Validate connection when borrowing from pool
    pub validate_on_borrow: bool,
    /// Validate connection when returning to pool
    pub validate_on_return: bool,
    /// Validate connections while they are idle in the pool
    pub validate_while_idle: bool,
    /// Interval between idle connection validations
    pub validation_interval: Duration,
}

impl Default for ConnectionValidationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strategy: ValidationStrategy::Ping,
            validation_timeout: Duration::from_secs(5),
            validate_on_borrow: true,
            validate_on_return: false,
            validate_while_idle: true,
            validation_interval: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// Connection validation strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidationStrategy {
    /// Execute a validation query to check connection health
    Query,
    /// Use driver's isValid() method for validation
    IsValid,
    /// Send a simple ping to validate connection
    Ping,
}

/// Connection lifecycle configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionLifecycleConfig {
    /// Configuration for tracking connection state
    pub state_tracking: ConnectionStateTracking,
    /// Strategy for managing connection pooling (FIFO, LIFO, etc.)
    pub pooling_strategy: PoolingStrategy,
}

impl Default for ConnectionLifecycleConfig {
    fn default() -> Self {
        Self {
            state_tracking: ConnectionStateTracking::default(),
            pooling_strategy: PoolingStrategy::Fifo,
        }
    }
}

/// Connection state tracking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStateTracking {
    /// Whether state tracking is enabled
    pub enabled: bool,
    /// Track connection usage statistics
    pub track_usage_stats: bool,
    /// Track connection performance metrics
    pub track_performance: bool,
    /// Duration to retain historical state data
    pub history_retention: Duration,
}

impl Default for ConnectionStateTracking {
    fn default() -> Self {
        Self {
            enabled: true,
            track_usage_stats: true,
            track_performance: true,
            history_retention: Duration::from_secs(86400), // 1 day
        }
    }
}

/// Connection pooling strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PoolingStrategy {
    Fifo,
    Lifo,
    Lru,
    Mru,
    Random,
}

/// Connection load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionLoadBalancingConfig {
    pub enabled: bool,
    pub strategy: LoadBalancingStrategy,
    pub health_check: LoadBalancerHealthCheck,
}

impl Default for ConnectionLoadBalancingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: LoadBalancingStrategy::RoundRobin,
            health_check: LoadBalancerHealthCheck::default(),
        }
    }
}

/// Load balancing strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    Random,
}

/// Load balancer health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerHealthCheck {
    pub enabled: bool,
    pub interval: Duration,
    pub timeout: Duration,
    pub healthy_threshold: u32,
    pub unhealthy_threshold: u32,
}

impl Default for LoadBalancerHealthCheck {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            healthy_threshold: 2,
            unhealthy_threshold: 3,
        }
    }
}

// ==================== Type Aliases for Backward Compatibility ====================

/// Backward compatibility alias
pub type UnifiedConnectionPoolConfig = ConnectionPoolConfig;
