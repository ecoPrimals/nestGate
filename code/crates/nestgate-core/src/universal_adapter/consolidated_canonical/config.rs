//! Configuration types for the consolidated canonical adapter
//!
//! This module contains all configuration structures used to customize
//! the behavior of the adapter, including discovery, requests, monitoring,
//! security, and performance settings.

use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

use super::enums::{DiscoveryMethod, RetryBackoff};

// ==================== MAIN CONFIGURATION ====================

/// Canonical adapter configuration - unified from all implementations
///
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::CanonicalAdapterConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
#[allow(deprecated)]
pub struct CanonicalAdapterConfig {
    /// Service identification
    pub service_id: String,
    /// Service name
    pub service_name: String,
    /// Service version
    pub service_version: String,

    /// Discovery configuration
    pub discovery: DiscoveryConfig,

    /// Request handling configuration
    pub requests: RequestConfig,

    /// Monitoring and metrics configuration
    pub monitoring: MonitoringConfig,

    /// Security configuration
    pub security: SecurityConfig,

    /// Performance configuration
    pub performance: PerformanceConfig,
}

#[allow(deprecated)]
impl Default for CanonicalAdapterConfig {
    fn default() -> Self {
        Self {
            service_id: Uuid::new_v4().to_string(),
            service_name: "nestgate".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            discovery: DiscoveryConfig::default(),
            requests: RequestConfig::default(),
            monitoring: MonitoringConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

// ==================== DISCOVERY CONFIGURATION ====================

/// Configuration for service discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Enable automatic discovery
    pub auto_discovery: bool,
    /// Discovery interval
    pub discovery_interval: Duration,
    /// Discovery methods to use
    pub discovery_methods: Vec<DiscoveryMethod>,
    /// Discovery timeout
    pub discovery_timeout: Duration,
    /// Number of retry attempts
    pub retry_attempts: u32,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            auto_discovery: true,
            discovery_interval: Duration::from_secs(30),
            discovery_methods: vec![DiscoveryMethod::Environment, DiscoveryMethod::ServiceRegistry],
            discovery_timeout: Duration::from_secs(10),
            retry_attempts: 3,
        }
    }
}

// ==================== REQUEST CONFIGURATION ====================

/// Configuration for request handling
///
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct RequestConfig {
    /// Request timeout
    pub timeout: Duration,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Retry backoff strategy
    pub retry_backoff: RetryBackoff,
    /// Maximum concurrent requests
    pub max_concurrent_requests: u32,
    /// Size of request queue
    pub request_queue_size: u32,
}

#[allow(deprecated)]
impl Default for RequestConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_backoff: RetryBackoff::Exponential {
                base: Duration::from_millis(100),
                max: Duration::from_secs(10),
            },
            max_concurrent_requests: 100,
            request_queue_size: 1000,
        }
    }
}

// ==================== MONITORING CONFIGURATION ====================

/// Configuration for monitoring and metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable metrics collection
    pub metrics_enabled: bool,
    /// Enable health checks
    pub health_checks_enabled: bool,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Metrics collection interval
    pub metrics_interval: Duration,
    /// Alert thresholds
    pub alert_thresholds: AlertThresholds,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            health_checks_enabled: true,
            health_check_interval: Duration::from_secs(30),
            metrics_interval: Duration::from_secs(60),
            alert_thresholds: AlertThresholds::default(),
        }
    }
}

// ==================== SECURITY CONFIGURATION ====================

/// Configuration for security settings
///
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(since = "0.11.0", note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
#[allow(deprecated)]
pub struct SecurityConfig {
    /// Enable authentication
    pub auth_enabled: bool,
    /// API key for authentication
    pub api_key: Option<String>,
    /// Enable TLS
    pub tls_enabled: bool,
    /// Verify certificates
    pub verify_certificates: bool,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitConfig,
}

#[allow(deprecated)]
impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            auth_enabled: true,
            api_key: None,
            tls_enabled: true,
            verify_certificates: true,
            rate_limiting: RateLimitConfig::default(),
        }
    }
}

// ==================== PERFORMANCE CONFIGURATION ====================

/// Configuration for performance tuning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Connection pool size
    pub connection_pool_size: u32,
    /// Keep-alive timeout
    pub keep_alive_timeout: Duration,
    /// Enable compression
    pub compression_enabled: bool,
    /// Enable caching
    pub caching_enabled: bool,
    /// Cache TTL
    pub cache_ttl: Duration,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            connection_pool_size: 10,
            keep_alive_timeout: Duration::from_secs(30),
            compression_enabled: true,
            caching_enabled: true,
            cache_ttl: Duration::from_secs(300),
        }
    }
}

// ==================== SUPPORTING CONFIGURATIONS ====================

/// Alert threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// Response time threshold in milliseconds
    pub response_time_ms: u64,
    /// Error rate threshold as percentage
    pub error_rate_percent: f64,
    /// Resource usage threshold as percentage
    pub resource_usage_percent: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            response_time_ms: 1000,
            error_rate_percent: 5.0,
            resource_usage_percent: 80.0,
        }
    }
}

/// Rate limiting configuration
///
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct RateLimitConfig {
    /// Requests per second allowed
    pub requests_per_second: u32,
    /// Burst size allowed
    pub burst_size: u32,
    /// Time window size
    pub window_size: Duration,
}

#[allow(deprecated)]
impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 100,
            burst_size: 200,
            window_size: Duration::from_secs(60),
        }
    }
}
