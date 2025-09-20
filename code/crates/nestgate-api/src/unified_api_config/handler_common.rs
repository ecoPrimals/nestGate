/// 
/// Shared configuration structures used across all handler types.
/// This module contains the base configuration patterns and common types.
use nestgate_core::unified_final_config::supporting_types::StandardDomainConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ==================== SECTION ====================

/// **THE** standardized handler configuration pattern
/// This replaces all scattered handler config structs with a single, consistent pattern
/// CANONICAL MODERNIZATION: Simplified type alias without type parameters
pub type HandlerConfig = StandardDomainConfig;
/// Handler-specific configuration extensions
/// Generic type parameter allows for handler-specific configuration data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandlerExtensions<T> {
    /// Common handler configuration
    pub common: CommonHandlerConfig,
    /// Handler-specific configuration
    pub specific: T,
}
/// Common configuration shared by all handlers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonHandlerConfig {
    /// Handler name/identifier
    pub handler_name: String,
    /// Enable handler
    pub enabled: bool,
    /// Request timeout
    pub timeout: Duration,
    /// Maximum concurrent requests
    pub max_concurrent_requests: usize,
    /// Rate limiting configuration
    pub rate_limit: RateLimitConfig,
    /// Retry configuration
    pub retry: RetryConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Metrics configuration
    pub metrics: MetricsConfig,
    /// Environment-specific overrides
    pub environment_overrides: HashMap<String, String>,
}
impl Default for CommonHandlerConfig {
    fn default() -> Self { Self {
            handler_name: "default".to_string(),
            enabled: true,
            timeout: Duration::from_secs(30),
            max_concurrent_requests: 100,
            rate_limit: RateLimitConfig::default(),
            retry: RetryConfig::default(),
            logging: LoggingConfig::default(),
            metrics: MetricsConfig::default(),
            environment_overrides: HashMap::new(),
         }
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Requests per second limit
    pub requests_per_second: u32,
    /// Burst size for rate limiting
    pub burst_size: u32,
    /// Rate limit window duration
    pub window_duration: Duration,
    /// Action when rate limit exceeded
    pub action: RateLimitAction,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RateLimitAction {
    Block,
    Queue,
    Throttle,
}
impl Default for RateLimitConfig {
    fn default() -> Self { Self {
            enabled: true,
            requests_per_second: 100,
            burst_size: 200,
            window_duration: Duration::from_secs(60),
            action: RateLimitAction::Block,
         }
}

/// Retry configuration for failed requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Enable retry logic
    pub enabled: bool,
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Initial delay between retries
    pub initial_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Backoff multiplier for exponential backoff
    pub backoff_multiplier: f64,
    /// Jitter to add to retry delays
    pub jitter: bool,
}
impl Default for RetryConfig {
    fn default() -> Self { Self {
            enabled: true,
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter: true,
         }
}

/// Logging configuration for handlers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Enable logging
    pub enabled: bool,
    /// Log level
    pub level: String,
    /// Log format
    pub format: LogFormat,
    /// Include request/response bodies
    pub include_bodies: bool,
    /// Maximum body size to log
    pub max_body_size: usize,
    /// Log sensitive headers
    pub log_sensitive_headers: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFormat {
    Json,
    Text,
    Structured,
}
impl Default for LoggingConfig {
    fn default() -> Self { Self {
            enabled: true,
            level: "info".to_string(),
            format: LogFormat::Json,
            include_bodies: false,
            max_body_size: 1024,
            log_sensitive_headers: false,
         }
}

/// **CANONICAL MODERNIZATION** - Use canonical metrics configuration
pub use nestgate_core::CanonicalMetricsConfig as MetricsConfig; 
// ==================== SECTION ====================
