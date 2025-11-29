//! Service Configs module

use std::collections::HashMap;
//
// This module contains configuration structures for service-level concerns
// like discovery, load balancing, routing, and inter-service communication.

use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== SECTION ====================

/// **CANONICAL SERVICE CONFIGURATIONS**
///
/// Unifies all service-level configuration fragments including discovery,
/// routing, load balancing, and inter-service communication patterns.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::CanonicalServiceConfigs;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::CanonicalServiceConfigs; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Canonicalserviceconfigs
pub struct CanonicalServiceConfigs {
    /// Service discovery configuration
    pub discovery: CanonicalDiscoveryConfig,
    /// Load balancing configuration
    pub load_balancing: CanonicalLoadBalancingConfig,
    /// Routing and traffic management configuration
    pub routing: CanonicalRoutingConfig,
    /// Health checking configuration
    pub health_checks: CanonicalHealthCheckConfig,
    /// Circuit breaker configuration
    pub circuit_breakers: CanonicalCircuitBreakerConfig,
    /// Rate limiting configuration
    pub rate_limiting: CanonicalRateLimitingConfig,
    /// Retry and backoff configuration
    pub retry: CanonicalRetryConfig,
}
// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::CanonicalDiscoveryConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::CanonicalDiscoveryConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for CanonicalDiscovery
pub struct CanonicalDiscoveryConfig {
    /// Discovery mechanism
    pub discovery_enabled: bool,
    #[deprecated(since = "3.0.0", note = "Use capability-based discovery instead of vendor-specific service discovery")]
    #[deprecated(since = "3.0.0", note = "Use capability-based storage instead of vendor-specific key-value stores")]
    #[deprecated(since = "3.0.0", note = "Use capability-based orchestration instead of vendor-specific container platforms")]
    /// Discovery Method
    pub discovery_method: String, // "dns", "service_discovery".to_string(), "keyvalue_store".to_string(), "container_orchestrator".to_string()
    /// Discovery Endpoint
    pub discovery_endpoint: Option<String>,
    /// Service registration
    pub auto_register: bool,
    /// Service name
    pub service_name: String,
    /// Service Tags
    pub service_tags: Vec<String>,
    /// Discovery intervals
    pub discovery_interval: Duration,
    /// Registration Ttl
    pub registration_ttl: Duration,
    /// Refresh Interval
    pub refresh_interval: Duration,
    /// Health integration
    pub health_check_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for CanonicalLoadBalancing
pub struct CanonicalLoadBalancingConfig {
    /// Load balancing strategy
    pub strategy: String, // "round_robin", "least_connections", "weighted", "ip_hash"
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Backend configuration
    pub backends: Vec<BackendConfig>,
    /// Backend Health Check
    pub backend_health_check: bool,
    /// Sticky sessions
    pub sticky_sessions: bool,
    /// Session Cookie name
    pub session_cookie_name: String,
    /// Failover
    pub failover_enabled: bool,
    /// Max Failures
    pub max_failures: usize,
    /// Failure Timeout
    pub failure_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Backend
pub struct BackendConfig {
    /// Endpoint
    pub endpoint: String,
    /// Port
    pub port: u16,
    /// Weight
    pub weight: u32,
    /// Max Connections
    pub max_connections: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::CanonicalRoutingConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::CanonicalRoutingConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for CanonicalRouting
pub struct CanonicalRoutingConfig {
    /// Routing rules
    pub rules: Vec<RoutingRule>,
    /// Default Backend
    pub default_backend: Option<String>,
    /// Path rewriting
    pub path_rewrite_enabled: bool,
    /// Path Rewrite Rules
    pub path_rewrite_rules: HashMap<String, String>,
    /// Header manipulation
    pub add_headers: HashMap<String, String>,
    /// Remove Headers
    pub remove_headers: Vec<String>,
    /// Timeouts
    pub request_timeout: Duration,
    /// Upstream Timeout
    pub upstream_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Routingrule
pub struct RoutingRule {
    /// Path Pattern
    pub path_pattern: String,
    /// Method
    pub method: Option<String>,
    /// Headers
    pub headers: Option<HashMap<String, String>>,
    /// Backend
    pub backend: String,
    /// Priority
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for CanonicalHealthCheck
pub struct CanonicalHealthCheckConfig {
    /// Health check settings
    pub enabled: bool,
    /// Endpoint
    pub endpoint: String,
    /// Method
    pub method: String,
    /// Expected Status
    pub expected_status: u16,
    /// Timing configuration
    pub interval: Duration,
    /// Timeout
    pub timeout: Duration,
    /// Startup Delay
    pub startup_delay: Duration,
    /// Failure handling
    pub failure_threshold: usize,
    /// Success Threshold
    pub success_threshold: usize,
    /// Health check types
    pub tcp_check: bool,
    /// Http Check
    pub http_check: bool,
    /// Custom Check
    pub custom_check: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for CanonicalCircuitBreaker
pub struct CanonicalCircuitBreakerConfig {
    /// Circuit breaker settings
    pub enabled: bool,
    /// Failure Threshold
    pub failure_threshold: usize,
    /// Recovery Timeout
    pub recovery_timeout: Duration,
    /// Half Open Max Calls
    pub half_open_max_calls: usize,
    /// Monitoring
    pub metrics_enabled: bool,
    /// State Change Callback
    pub state_change_callback: Option<String>,
    /// Per-service configuration
    pub per_service_config: HashMap<String, ServiceCircuitBreakerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::config::ServiceCircuitBreakerConfig;
/// 
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::ServiceCircuitBreakerConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for ServiceCircuitBreaker
pub struct ServiceCircuitBreakerConfig {
    /// Failure Threshold
    pub failure_threshold: usize,
    /// Recovery Timeout
    pub recovery_timeout: Duration,
    /// Half Open Max Calls
    pub half_open_max_calls: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::CanonicalRateLimitingConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::CanonicalRateLimitingConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for CanonicalRateLimiting
pub struct CanonicalRateLimitingConfig {
    /// Rate limiting settings
    pub enabled: bool,
    /// Global Rate Limit
    pub global_rate_limit: Option<RateLimit>,
    /// Per Ip Rate Limit
    pub per_ip_rate_limit: Option<RateLimit>,
    /// Per User Rate Limit
    pub per_user_rate_limit: Option<RateLimit>,
    /// Rate limiting backend
    #[deprecated(since = "3.0.0", note = "Use capability-based caching instead of vendor-specific cache implementations")]
    /// Backend
    pub backend: String, // "memory", "cache_store".to_string(), "database"
    /// Configuration for backend
    pub backend_config: HashMap<String, String>,
    /// Response configuration
    pub rate_limit_headers: bool,
    /// Custom Response
    pub custom_response: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Ratelimit
pub struct RateLimit {
    /// Requests Per Second
    pub requests_per_second: u32,
    /// Size of burst
    pub burst_size: u32,
    /// Size of window
    pub window_size: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for CanonicalRetry
pub struct CanonicalRetryConfig {
    /// Retry settings
    pub enabled: bool,
    /// Max Attempts
    pub max_attempts: usize,
    /// Initial Delay
    pub initial_delay: Duration,
    /// Max Delay
    pub max_delay: Duration,
    /// Backoff strategy
    pub backoff_strategy: String, // "exponential", "linear", "fixed"
    /// Backoff Multiplier
    pub backoff_multiplier: f64,
    /// Jitter
    pub jitter: bool,
    /// Retry conditions
    pub retry_on_status_codes: Vec<u16>,
    /// Retry On Timeouts
    pub retry_on_timeouts: bool,
    /// Retry On Connection Errors
    pub retry_on_connection_errors: bool,
}

// ==================== SECTION ====================

impl Default for BackendConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            endpoint: crate::constants::hardcoding::addresses::LOCALHOST_NAME.to_string(),
            port: crate::constants::hardcoding::ports::HTTP_DEFAULT,
            weight: 1,
            max_connections: None,
        }
    }
}

impl Default for RoutingRule {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            path_pattern: "/*".to_string(),
            method: None,
            headers: None,
            backend: "default".to_string(),
            priority: 100,
        }
    }
}

impl Default for ServiceCircuitBreakerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(30),
            half_open_max_calls: 3,
        }
    }
}

impl Default for RateLimit {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            requests_per_second: 100,
            burst_size: 200,
            window_size: Duration::from_secs(1),
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
#[allow(deprecated)]
/// Type alias for Canonicalserviceconfigscanonical
pub type CanonicalServiceConfigsCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using CanonicalServiceConfigs (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Canonicaldiscoveryconfigcanonical
pub type CanonicalDiscoveryConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using CanonicalDiscoveryConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Canonicalratelimitingconfigcanonical
pub type CanonicalRateLimitingConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using CanonicalRateLimitingConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Canonicalroutingconfigcanonical
pub type CanonicalRoutingConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using CanonicalRoutingConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Servicecircuitbreakerconfigcanonical
pub type ServiceCircuitBreakerConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using ServiceCircuitBreakerConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

