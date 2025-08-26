///
/// This module contains all connection pool-related configuration types including pool sizing,
/// health checks, and retry strategies.
/// Split from unified_types/mod.rs for better maintainability and 2000-line compliance.
use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== UNIFIED CONNECTION POOL CONFIGURATION ====================

/// Unified Connection Pool Configuration - consolidates all connection pooling settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedConnectionPoolConfig {
    /// Enable connection pooling
    pub enabled: bool,
    /// Initial pool size
    pub initial_size: u32,
    /// Maximum pool size
    pub cache_size_bytes: u32,
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
}

impl Default for UnifiedConnectionPoolConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            initial_size: 5,
            cache_size_bytes: 20,
            min_idle: 2,
            max_idle: 10,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600), // 10 minutes
            max_lifetime: Duration::from_secs(1800), // 30 minutes
            health_check_interval: Duration::from_secs(60),
            retry_config: ConnectionRetryConfig::default(),
            monitoring: PoolMonitoringConfig::default(),
            validation: ConnectionValidationConfig::default(),
        }
    }
}

// ==================== CONNECTION RETRY CONFIGURATION ====================

/// Connection retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionRetryConfig {
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Initial retry interval
    pub initial_interval: Duration,
    /// Maximum retry interval
    pub max_interval: Duration,
    /// Backoff multiplier
    pub multiplier: f64,
    /// Enable jitter
    pub jitter: bool,
    /// Retry on connection errors
    pub retry_on_connection_error: bool,
    /// Retry on timeout errors
    pub retry_on_timeout: bool,
    /// Custom retry conditions
    pub custom_conditions: Vec<RetryCondition>,
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
            custom_conditions: vec![],
        }
    }
}

/// Retry condition for connection failures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryCondition {
    /// Condition name
    pub name: String,
    /// Error pattern to match
    pub error_pattern: String,
    /// Whether to retry on this condition
    pub should_retry: bool,
    /// Custom retry delay for this condition
    pub custom_delay: Option<Duration>,
}

// ==================== POOL MONITORING CONFIGURATION ====================

/// Pool monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolMonitoringConfig {
    /// Enable pool monitoring
    pub enabled: bool,
    /// Monitoring interval
    pub interval: Duration,
    /// Metrics to collect
    pub metrics: Vec<PoolMetric>,
    /// Performance thresholds
    pub thresholds: PoolThresholds,
    /// Alert configuration
    pub alerts: PoolAlertConfig,
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
                PoolMetric::ConnectionAcquisitionTime,
                PoolMetric::ConnectionUtilization,
            ],
            thresholds: PoolThresholds::default(),
            alerts: PoolAlertConfig::default(),
        }
    }
}

/// Pool metrics to collect
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PoolMetric {
    /// Number of active connections
    ActiveConnections,
    /// Number of idle connections
    IdleConnections,
    /// Total number of connections
    TotalConnections,
    /// Time to acquire a connection
    ConnectionAcquisitionTime,
    /// Pool utilization percentage
    ConnectionUtilization,
    /// Connection creation rate
    ConnectionCreationRate,
    /// Connection failure rate
    ConnectionFailureRate,
    /// Custom pool metric
    Custom(String),
}

/// Pool performance thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolThresholds {
    /// Maximum acceptable acquisition time
    pub max_acquisition_time: Duration,
    /// Maximum pool utilization percentage
    pub max_utilization: f64,
    /// Maximum connection failure rate
    pub max_failure_rate: f64,
    /// Minimum number of healthy connections
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

/// Pool alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolAlertConfig {
    /// Enable pool alerts
    pub enabled: bool,
    /// Alert rules
    pub rules: Vec<PoolAlertRule>,
    /// Alert cooldown period
    pub cooldown: Duration,
}

impl Default for PoolAlertConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: vec![
                PoolAlertRule {
                    name: "High Pool Utilization".to_string(),
                    condition: PoolAlertCondition::UtilizationAbove(0.85),
                    severity: PoolAlertSeverity::Warning,
                    duration: Duration::from_secs(300),
                },
                PoolAlertRule {
                    name: "Connection Acquisition Timeout".to_string(),
                    condition: PoolAlertCondition::AcquisitionTimeAbove(Duration::from_secs(1)),
                    severity: PoolAlertSeverity::Critical,
                    duration: Duration::from_secs(60),
                },
            ],
            cooldown: Duration::from_secs(300),
        }
    }
}

/// Pool alert rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolAlertRule {
    /// Alert rule name
    pub name: String,
    /// Alert condition
    pub condition: PoolAlertCondition,
    /// Alert severity
    pub severity: PoolAlertSeverity,
    /// Duration condition must be true
    pub duration: Duration,
}

/// Pool alert conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolAlertCondition {
    /// Pool utilization above threshold
    UtilizationAbove(f64),
    /// Connection acquisition time above threshold
    AcquisitionTimeAbove(Duration),
    /// Connection failure rate above threshold
    FailureRateAbove(f64),
    /// Number of healthy connections below threshold
    HealthyConnectionsBelow(u32),
    /// Custom alert condition
    Custom(String),
}

/// Pool alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PoolAlertSeverity {
    /// Informational alert
    Info,
    /// Warning alert
    Warning,
    /// Critical alert
    Critical,
}

// ==================== CONNECTION VALIDATION CONFIGURATION ====================

/// Connection validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionValidationConfig {
    /// Enable connection validation
    pub enabled: bool,
    /// Validation strategy
    pub strategy: ValidationStrategy,
    /// Validation query or command
    pub validation_query: Option<String>,
    /// Validation timeout
    pub validation_timeout: Duration,
    /// Validate on borrow
    pub validate_on_borrow: bool,
    /// Validate on return
    pub validate_on_return: bool,
    /// Validate while idle
    pub validate_while_idle: bool,
    /// Validation interval for idle connections
    pub validation_interval: Duration,
}

impl Default for ConnectionValidationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strategy: ValidationStrategy::Query,
            validation_query: Some("SELECT 1".to_string()),
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
    /// Execute a validation query
    Query,
    /// Use connection's isValid method
    IsValid,
    /// Ping the connection
    Ping,
    /// Custom validation strategy
    Custom(String),
}

// ==================== CONNECTION POOL TYPES ====================

/// Connection pool types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PoolType {
    /// Database connection pool
    Database,
    /// HTTP connection pool
    Http,
    /// Redis connection pool
    Redis,
    /// Message queue connection pool
    MessageQueue,
    /// Custom connection pool type
    Custom(String),
}

// ==================== CONNECTION LIFECYCLE CONFIGURATION ====================

/// Connection lifecycle configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionLifecycleConfig {
    /// Connection initialization hooks
    pub initialization_hooks: Vec<LifecycleHook>,
    /// Connection cleanup hooks
    pub cleanup_hooks: Vec<LifecycleHook>,
    /// Connection state tracking
    pub state_tracking: ConnectionStateTracking,
    /// Connection pooling strategy
    pub pooling_strategy: PoolingStrategy,
}

impl Default for ConnectionLifecycleConfig {
    fn default() -> Self {
        Self {
            initialization_hooks: vec![],
            cleanup_hooks: vec![],
            state_tracking: ConnectionStateTracking::default(),
            pooling_strategy: PoolingStrategy::Fifo,
        }
    }
}

/// Lifecycle hook for connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleHook {
    /// Hook name
    pub name: String,
    /// Hook type
    pub hook_type: LifecycleHookType,
    /// Hook execution timeout
    pub timeout: Duration,
    /// Whether hook failure should fail the operation
    pub fail_on_error: bool,
}

/// Types of lifecycle hooks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LifecycleHookType {
    /// Pre-connection initialization
    PreInit,
    /// Post-connection initialization
    PostInit,
    /// Pre-connection cleanup
    PreCleanup,
    /// Post-connection cleanup
    PostCleanup,
    /// Custom hook type
    Custom(String),
}

/// Connection state tracking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStateTracking {
    /// Enable state tracking
    pub enabled: bool,
    /// Track connection usage statistics
    pub track_usage_stats: bool,
    /// Track connection performance metrics
    pub track_performance: bool,
    /// State history retention period
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
    /// First In, First Out
    Fifo,
    /// Last In, First Out
    Lifo,
    /// Least Recently Used
    Lru,
    /// Most Recently Used
    Mru,
    /// Random selection
    Random,
    /// Custom pooling strategy
    Custom(String),
}

// ==================== CONNECTION LOAD BALANCING ====================

/// Connection load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionLoadBalancingConfig {
    /// Enable load balancing
    pub enabled: bool,
    /// Load balancing strategy
    pub strategy: LoadBalancingStrategy,
    /// Health check configuration for endpoints
    pub health_check: LoadBalancerHealthCheck,
    /// Failover configuration
    pub failover: FailoverConfig,
}

impl Default for ConnectionLoadBalancingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: LoadBalancingStrategy::RoundRobin,
            health_check: LoadBalancerHealthCheck::default(),
            failover: FailoverConfig::default(),
        }
    }
}

/// Load balancing strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LoadBalancingStrategy {
    /// Round-robin balancing
    RoundRobin,
    /// Weighted round-robin
    WeightedRoundRobin,
    /// Least connections
    LeastConnections,
    /// Weighted least connections
    WeightedLeastConnections,
    /// Random selection
    Random,
    /// Weighted random
    WeightedRandom,
    /// Custom load balancing strategy
    Custom(String),
}

/// Load balancer health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerHealthCheck {
    /// Enable health checks
    pub enabled: bool,
    /// Health check interval
    pub interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Number of consecutive successes to mark healthy
    pub healthy_threshold: u32,
    /// Number of consecutive failures to mark unhealthy
    pub unhealthy_threshold: u32,
    /// Health check method
    pub method: HealthCheckMethod,
}

impl Default for LoadBalancerHealthCheck {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            healthy_threshold: 2,
            unhealthy_threshold: 3,
            method: HealthCheckMethod::Ping,
        }
    }
}

/// Health check methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthCheckMethod {
    /// Simple ping/pong check
    Ping,
    /// Execute a health check query
    Query,
    /// HTTP health check endpoint
    Http,
    /// TCP connection check
    Tcp,
    /// Custom health check method
    Custom(String),
}

/// Failover configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverConfig {
    /// Enable automatic failover
    pub enabled: bool,
    /// Failover strategy
    pub strategy: FailoverStrategy,
    /// Maximum failover attempts
    pub max_attempts: u32,
    /// Failover timeout
    pub timeout: Duration,
    /// Failback configuration
    pub failback: FailbackConfig,
}

impl Default for FailoverConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strategy: FailoverStrategy::Immediate,
            max_attempts: 3,
            timeout: Duration::from_secs(30),
            failback: FailbackConfig::default(),
        }
    }
}

/// Failover strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FailoverStrategy {
    /// Immediate failover
    Immediate,
    /// Graceful failover with delay
    Graceful,
    /// Manual failover only
    Manual,
    /// Custom failover strategy
    Custom(String),
}

/// Failback configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailbackConfig {
    /// Enable automatic failback
    pub enabled: bool,
    /// Failback delay after primary recovery
    pub delay: Duration,
    /// Number of consecutive health checks before failback
    pub health_check_threshold: u32,
}

impl Default for FailbackConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            delay: Duration::from_secs(60),
            health_check_threshold: 3,
        }
    }
}
