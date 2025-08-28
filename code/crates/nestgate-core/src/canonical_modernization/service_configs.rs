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
pub struct CanonicalDiscoveryConfig {
    /// Discovery mechanism
    pub discovery_enabled: bool,
    pub discovery_method: String, // "dns", "consul", "etcd", "k8s"
    pub discovery_endpoint: Option<String>,
    /// Service registration
    pub auto_register: bool,
    pub service_name: String,
    pub service_tags: Vec<String>,
    /// Discovery intervals
    pub discovery_interval: Duration,
    pub registration_ttl: Duration,
    pub refresh_interval: Duration,
    /// Health integration
    pub health_check_enabled: bool,
    pub health_check_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalLoadBalancingConfig {
    /// Load balancing strategy
    pub strategy: String, // "round_robin", "least_connections", "weighted", "ip_hash"
    pub enabled: bool,
    /// Backend configuration
    pub backends: Vec<BackendConfig>,
    pub backend_health_check: bool,
    /// Sticky sessions
    pub sticky_sessions: bool,
    pub session_cookie_name: String,
    /// Failover
    pub failover_enabled: bool,
    pub max_failures: usize,
    pub failure_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    pub address: String,
    pub port: u16,
    pub weight: u32,
    pub max_connections: Option<usize>,
    pub health_check_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalRoutingConfig {
    /// Routing rules
    pub rules: Vec<RoutingRule>,
    pub default_backend: Option<String>,
    /// Path rewriting
    pub path_rewrite_enabled: bool,
    pub path_rewrite_rules: HashMap<String, String>,
    /// Header manipulation
    pub add_headers: HashMap<String, String>,
    pub remove_headers: Vec<String>,
    /// Timeouts
    pub request_timeout: Duration,
    pub upstream_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRule {
    pub path_pattern: String,
    pub method: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub backend: String,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalHealthCheckConfig {
    /// Health check settings
    pub enabled: bool,
    pub endpoint: String,
    pub method: String,
    pub expected_status: u16,
    /// Timing configuration
    pub interval: Duration,
    pub timeout: Duration,
    pub startup_delay: Duration,
    /// Failure handling
    pub failure_threshold: usize,
    pub success_threshold: usize,
    /// Health check types
    pub tcp_check: bool,
    pub http_check: bool,
    pub custom_check: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalCircuitBreakerConfig {
    /// Circuit breaker settings
    pub enabled: bool,
    pub failure_threshold: usize,
    pub recovery_timeout: Duration,
    pub half_open_max_calls: usize,
    /// Monitoring
    pub metrics_enabled: bool,
    pub state_change_callback: Option<String>,
    /// Per-service configuration
    pub per_service_config: HashMap<String, ServiceCircuitBreakerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCircuitBreakerConfig {
    pub failure_threshold: usize,
    pub recovery_timeout: Duration,
    pub half_open_max_calls: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalRateLimitingConfig {
    /// Rate limiting settings
    pub enabled: bool,
    pub global_rate_limit: Option<RateLimit>,
    pub per_ip_rate_limit: Option<RateLimit>,
    pub per_user_rate_limit: Option<RateLimit>,
    /// Rate limiting backend
    pub backend: String, // "memory", "redis", "database"
    pub backend_config: HashMap<String, String>,
    /// Response configuration
    pub rate_limit_headers: bool,
    pub custom_response: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_second: u32,
    pub burst_size: u32,
    pub window_size: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalRetryConfig {
    /// Retry settings
    pub enabled: bool,
    pub max_attempts: usize,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    /// Backoff strategy
    pub backoff_strategy: String, // "exponential", "linear", "fixed"
    pub backoff_multiplier: f64,
    pub jitter: bool,
    /// Retry conditions
    pub retry_on_status_codes: Vec<u16>,
    pub retry_on_timeouts: bool,
    pub retry_on_connection_errors: bool,
}

// ==================== SECTION ====================

impl Default for BackendConfig {
    fn default() -> Self {
        Self {
            address: "localhost".to_string(),
            port: 8080,
            weight: 1,
            max_connections: None,
            health_check_path: Some("/health".to_string()),
        }
    }
}

impl Default for RoutingRule {
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
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(30),
            half_open_max_calls: 3,
        }
    }
}

impl Default for RateLimit {
    fn default() -> Self {
        Self {
            requests_per_second: 100,
            burst_size: 200,
            window_size: Duration::from_secs(1),
        }
    }
} 