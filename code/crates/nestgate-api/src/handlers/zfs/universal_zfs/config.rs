//! Universal ZFS Service Configuration
//!
//! Comprehensive configuration system supporting multiple backends,
//! fail-safe mechanisms, and runtime configuration updates.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::time::Duration;

/// Main configuration for ZFS service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsServiceConfig {
    /// Name of the ZFS service instance
    pub service_name: String,
    /// Version of the ZFS service
    pub service_version: String,
    /// Backend configuration for ZFS operations
    pub backend: ZfsBackend,
    /// Fail-safe and resilience configuration
    pub fail_safe: FailSafeConfig,
    /// Observability and monitoring configuration
    pub observability: ObservabilityConfig,
    /// Performance optimization settings
    pub performance: PerformanceConfig,
    /// Security configuration and policies
    pub security: SecurityConfig,
    /// Custom properties for service configuration
    pub custom_properties: HashMap<String, String>,
}

impl Default for ZfsServiceConfig {
    fn default() -> Self {
        Self {
            service_name: "universal-zfs".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            backend: ZfsBackend::Auto,
            fail_safe: FailSafeConfig::default(),
            observability: ObservabilityConfig::default(),
            performance: PerformanceConfig::default(),
            security: SecurityConfig::default(),
            custom_properties: HashMap::new(),
        }
    }
}

impl ZfsServiceConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        let backend = match env::var("ZFS_BACKEND").as_deref() {
            Ok("native") => ZfsBackend::Native,
            Ok("mock") => ZfsBackend::Mock,
            Ok(url) if url.starts_with("http") => ZfsBackend::Remote(RemoteConfig {
                endpoint: url.to_string(),
                timeout: Duration::from_secs(30),
                auth: None,
            }),
            Ok("auto") => ZfsBackend::Auto,
            _ => ZfsBackend::Auto,
        };

        let service_name =
            env::var("ZFS_SERVICE_NAME").unwrap_or_else(|_| "universal-zfs".to_string());

        Self {
            service_name,
            backend,
            fail_safe: FailSafeConfig::from_env(),
            observability: ObservabilityConfig::from_env(),
            performance: PerformanceConfig::from_env(),
            security: SecurityConfig::from_env(),
            ..Default::default()
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.service_name.is_empty() {
            return Err("Service name cannot be empty".to_string());
        }

        self.backend.validate()?;
        self.fail_safe.validate()?;
        self.observability.validate()?;
        self.performance.validate()?;
        self.security.validate()?;

        Ok(())
    }
}

/// Backend configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsBackend {
    /// Automatically detect the best backend
    Auto,
    /// Use native ZFS commands
    Native,
    /// Use mock responses for testing
    Mock,
    /// Use remote ZFS service
    Remote(RemoteConfig),
    /// Use multiple backends with load balancing
    LoadBalanced(Vec<ZfsBackend>),
    /// Use primary backend with fallback
    Failover {
        /// Primary backend to use first
        primary: Box<ZfsBackend>,
        /// Fallback backend if primary fails
        fallback: Box<ZfsBackend>,
    },
}

impl ZfsBackend {
    /// Validates the ZFS backend configuration
    pub fn validate(&self) -> Result<(), String> {
        match self {
            ZfsBackend::Auto | ZfsBackend::Native | ZfsBackend::Mock => Ok(()),
            ZfsBackend::Remote(config) => config.validate(),
            ZfsBackend::LoadBalanced(backends) => {
                if backends.is_empty() {
                    return Err("Load balanced backend must have at least one backend".to_string());
                }
                for backend in backends {
                    backend.validate()?;
                }
                Ok(())
            }
            ZfsBackend::Failover { primary, fallback } => {
                primary.validate()?;
                fallback.validate()?;
                Ok(())
            }
        }
    }
}

/// Remote service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteConfig {
    /// Remote service endpoint URL
    pub endpoint: String,
    /// Request timeout duration
    pub timeout: Duration,
    /// Authentication configuration (optional)
    pub auth: Option<AuthConfig>,
}

impl RemoteConfig {
    fn validate(&self) -> Result<(), String> {
        if self.endpoint.is_empty() {
            return Err("Remote endpoint cannot be empty".to_string());
        }
        if !self.endpoint.starts_with("http://") && !self.endpoint.starts_with("https://") {
            return Err("Remote endpoint must be a valid HTTP/HTTPS URL".to_string());
        }
        Ok(())
    }
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthConfig {
    /// No authentication required
    None,
    /// API key authentication
    ApiKey(String),
    /// Bearer token authentication
    Bearer(String),
    /// Basic username/password authentication
    Basic {
        /// Username for basic auth
        username: String,
        /// Password for basic auth
        password: String,
    },
}

/// Fail-safe configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailSafeConfig {
    /// Circuit breaker configuration
    pub circuit_breaker: CircuitBreakerConfig,
    /// Retry policy configuration
    pub retry_policy: RetryPolicy,
    /// Timeout configuration
    pub timeout: TimeoutConfig,
    /// Enable graceful degradation on failures
    pub graceful_degradation: bool,
    /// Enable fallback mechanisms
    pub fallback_enabled: bool,
}

impl Default for FailSafeConfig {
    fn default() -> Self {
        Self {
            circuit_breaker: CircuitBreakerConfig::default(),
            retry_policy: RetryPolicy::default(),
            timeout: TimeoutConfig::default(),
            graceful_degradation: true,
            fallback_enabled: true,
        }
    }
}

impl FailSafeConfig {
    fn from_env() -> Self {
        let graceful_degradation =
            env::var("ZFS_GRACEFUL_DEGRADATION").unwrap_or_else(|_| "true".to_string()) == "true";

        let fallback_enabled =
            env::var("ZFS_FALLBACK_ENABLED").unwrap_or_else(|_| "true".to_string()) == "true";

        Self {
            circuit_breaker: CircuitBreakerConfig::from_env(),
            retry_policy: RetryPolicy::from_env(),
            timeout: TimeoutConfig::from_env(),
            graceful_degradation,
            fallback_enabled,
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.circuit_breaker.validate()?;
        self.retry_policy.validate()?;
        self.timeout.validate()?;
        Ok(())
    }
}

/// Circuit breaker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening the circuit
    pub failure_threshold: u32,
    /// Time to wait before attempting recovery
    pub recovery_timeout: Duration,
    /// Maximum calls allowed in half-open state
    pub half_open_max_calls: u32,
    /// Whether circuit breaker is enabled
    pub enabled: bool,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(60),
            half_open_max_calls: 3,
            enabled: true,
        }
    }
}

impl CircuitBreakerConfig {
    fn from_env() -> Self {
        let failure_threshold = env::var("ZFS_CIRCUIT_BREAKER_FAILURE_THRESHOLD")
            .unwrap_or_else(|_| "5".to_string())
            .parse()
            .unwrap_or(5);

        let recovery_timeout = env::var("ZFS_CIRCUIT_BREAKER_RECOVERY_TIMEOUT")
            .unwrap_or_else(|_| "60".to_string())
            .parse()
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(60));

        let enabled = env::var("ZFS_CIRCUIT_BREAKER_ENABLED")
            .unwrap_or_else(|_| "true".to_string())
            == "true";

        Self {
            failure_threshold,
            recovery_timeout,
            half_open_max_calls: 3,
            enabled,
        }
    }

    fn validate(&self) -> Result<(), String> {
        if self.failure_threshold == 0 {
            return Err("Circuit breaker failure threshold must be greater than 0".to_string());
        }
        if self.recovery_timeout < Duration::from_secs(1) {
            return Err("Circuit breaker recovery timeout must be at least 1 second".to_string());
        }
        if self.half_open_max_calls == 0 {
            return Err("Circuit breaker half-open max calls must be greater than 0".to_string());
        }
        Ok(())
    }
}

/// Retry policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Initial delay before first retry
    pub initial_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Multiplier for exponential backoff
    pub backoff_multiplier: f64,
    /// Whether retry policy is enabled
    pub enabled: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
            enabled: true,
        }
    }
}

impl RetryPolicy {
    fn from_env() -> Self {
        let max_attempts = env::var("ZFS_RETRY_MAX_ATTEMPTS")
            .unwrap_or_else(|_| "3".to_string())
            .parse()
            .unwrap_or(3);

        let initial_delay = env::var("ZFS_RETRY_INITIAL_DELAY")
            .unwrap_or_else(|_| "100".to_string())
            .parse()
            .map(Duration::from_millis)
            .unwrap_or(Duration::from_millis(100));

        let enabled =
            env::var("ZFS_RETRY_ENABLED").unwrap_or_else(|_| "true".to_string()) == "true";

        Self {
            max_attempts,
            initial_delay,
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
            enabled,
        }
    }

    fn validate(&self) -> Result<(), String> {
        if self.max_attempts == 0 {
            return Err("Retry max attempts must be greater than 0".to_string());
        }
        if self.initial_delay < Duration::from_millis(1) {
            return Err("Retry initial delay must be at least 1ms".to_string());
        }
        if self.max_delay < self.initial_delay {
            return Err("Retry max delay must be greater than initial delay".to_string());
        }
        if self.backoff_multiplier < 1.0 {
            return Err("Retry backoff multiplier must be at least 1.0".to_string());
        }
        Ok(())
    }
}

/// Timeout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfig {
    /// Timeout for ZFS operations
    pub operation_timeout: Duration,
    /// Timeout for establishing connections
    pub connection_timeout: Duration,
    /// Timeout for read operations
    pub read_timeout: Duration,
    /// Timeout for write operations
    pub write_timeout: Duration,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            operation_timeout: Duration::from_secs(30),
            connection_timeout: Duration::from_secs(10),
            read_timeout: Duration::from_secs(15),
            write_timeout: Duration::from_secs(15),
        }
    }
}

impl TimeoutConfig {
    fn from_env() -> Self {
        let operation_timeout = env::var("ZFS_OPERATION_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(30));

        Self {
            operation_timeout,
            connection_timeout: Duration::from_secs(10),
            read_timeout: Duration::from_secs(15),
            write_timeout: Duration::from_secs(15),
        }
    }

    fn validate(&self) -> Result<(), String> {
        if self.operation_timeout < Duration::from_secs(1) {
            return Err("Operation timeout must be at least 1 second".to_string());
        }
        if self.connection_timeout < Duration::from_secs(1) {
            return Err("Connection timeout must be at least 1 second".to_string());
        }
        Ok(())
    }
}

/// Observability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    /// Enable metrics collection
    pub metrics_enabled: bool,
    /// Enable distributed tracing
    pub tracing_enabled: bool,
    /// Interval between health checks
    pub health_check_interval: Duration,
    /// Logging level (trace, debug, info, warn, error)
    pub log_level: String,
    /// Enable structured JSON logging
    pub structured_logging: bool,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            tracing_enabled: true,
            health_check_interval: Duration::from_secs(30),
            log_level: "info".to_string(),
            structured_logging: true,
        }
    }
}

impl ObservabilityConfig {
    fn from_env() -> Self {
        let metrics_enabled =
            env::var("ZFS_METRICS_ENABLED").unwrap_or_else(|_| "true".to_string()) == "true";

        let tracing_enabled =
            env::var("ZFS_TRACING_ENABLED").unwrap_or_else(|_| "true".to_string()) == "true";

        let log_level = env::var("ZFS_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

        Self {
            metrics_enabled,
            tracing_enabled,
            health_check_interval: Duration::from_secs(30),
            log_level,
            structured_logging: true,
        }
    }

    fn validate(&self) -> Result<(), String> {
        let valid_log_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_log_levels.contains(&self.log_level.as_str()) {
            return Err(format!(
                "Invalid log level '{}'. Must be one of: {:?}",
                self.log_level, valid_log_levels
            ));
        }
        Ok(())
    }
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Size of the connection pool
    pub connection_pool_size: u32,
    /// Maximum number of concurrent operations
    pub max_concurrent_operations: u32,
    /// Enable caching for improved performance
    pub cache_enabled: bool,
    /// Cache time-to-live duration
    pub cache_ttl: Duration,
    /// Batch size for bulk operations
    pub batch_size: u32,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            connection_pool_size: 10,
            max_concurrent_operations: 100,
            cache_enabled: true,
            cache_ttl: Duration::from_secs(300),
            batch_size: 10,
        }
    }
}

impl PerformanceConfig {
    fn from_env() -> Self {
        let connection_pool_size = env::var("ZFS_CONNECTION_POOL_SIZE")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10);

        let max_concurrent_operations = env::var("ZFS_MAX_CONCURRENT_OPERATIONS")
            .unwrap_or_else(|_| "100".to_string())
            .parse()
            .unwrap_or(100);

        let cache_enabled =
            env::var("ZFS_CACHE_ENABLED").unwrap_or_else(|_| "true".to_string()) == "true";

        Self {
            connection_pool_size,
            max_concurrent_operations,
            cache_enabled,
            cache_ttl: Duration::from_secs(300),
            batch_size: 10,
        }
    }

    fn validate(&self) -> Result<(), String> {
        if self.connection_pool_size == 0 {
            return Err("Connection pool size must be greater than 0".to_string());
        }
        if self.max_concurrent_operations == 0 {
            return Err("Max concurrent operations must be greater than 0".to_string());
        }
        if self.batch_size == 0 {
            return Err("Batch size must be greater than 0".to_string());
        }
        Ok(())
    }
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable TLS encryption
    pub tls_enabled: bool,
    /// Require authentication for access
    pub auth_required: bool,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitConfig,
    /// List of allowed CORS origins
    pub allowed_origins: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            tls_enabled: false,
            auth_required: false,
            rate_limiting: RateLimitConfig::default(),
            allowed_origins: vec!["*".to_string()],
        }
    }
}

impl SecurityConfig {
    fn from_env() -> Self {
        let tls_enabled =
            env::var("ZFS_TLS_ENABLED").unwrap_or_else(|_| "false".to_string()) == "true";

        let auth_required =
            env::var("ZFS_AUTH_REQUIRED").unwrap_or_else(|_| "false".to_string()) == "true";

        Self {
            tls_enabled,
            auth_required,
            rate_limiting: RateLimitConfig::from_env(),
            allowed_origins: vec!["*".to_string()],
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.rate_limiting.validate()?;
        Ok(())
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Maximum requests per second
    pub requests_per_second: u32,
    /// Burst capacity for traffic spikes
    pub burst_size: u32,
    /// Time window for rate limiting
    pub window: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            requests_per_second: 100,
            burst_size: 10,
            window: Duration::from_secs(60),
        }
    }
}

impl RateLimitConfig {
    fn from_env() -> Self {
        let enabled =
            env::var("ZFS_RATE_LIMIT_ENABLED").unwrap_or_else(|_| "false".to_string()) == "true";

        let requests_per_second = env::var("ZFS_RATE_LIMIT_RPS")
            .unwrap_or_else(|_| "100".to_string())
            .parse()
            .unwrap_or(100);

        Self {
            enabled,
            requests_per_second,
            burst_size: 10,
            window: Duration::from_secs(60),
        }
    }

    fn validate(&self) -> Result<(), String> {
        if self.requests_per_second == 0 {
            return Err("Rate limit requests per second must be greater than 0".to_string());
        }
        if self.burst_size == 0 {
            return Err("Rate limit burst size must be greater than 0".to_string());
        }
        Ok(())
    }
}
