// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **CANONICAL API CONFIGURATION MODULE**
//!
//! The single source of truth for all API configuration across NestGate.
//! Consolidates NetworkApiConfig, UnifiedApiConfig, and handler configs.

use nestgate_types::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;

/// **CANONICAL API CONFIGURATION**
///
/// Consolidates all API configuration patterns into a single comprehensive struct.
///
/// **Replaces**:
/// - `NetworkApiConfig` (domains/network/api.rs)
/// - `UnifiedApiConfig` (nestgate-api/config/unified_api_config.rs)
/// - `ApiConfig` (`canonical_primary/api_config.rs`)
/// - Handler-specific configs (`unified_api_config/handlers.rs`)
///
/// Configuration for Api
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    // ==================== CORE NETWORK SETTINGS ====================
    /// API server bind address
    pub bind_address: IpAddr,

    /// API server port
    pub port: u16,

    /// Maximum concurrent connections
    pub max_connections: u32,

    /// Request timeout
    pub request_timeout: Duration,

    /// Connection timeout
    pub connection_timeout: Duration,

    /// Port allocation range start (for dynamic service ports)
    pub port_range_start: u16,

    /// Port allocation range end (for dynamic service ports)
    pub port_range_end: u16,

    // ==================== API METADATA ====================
    /// Enable API server
    pub enabled: bool,

    /// API version string
    pub version: String,

    /// Custom API settings (extensibility)
    pub api_settings: HashMap<String, serde_json::Value>,

    // ==================== SECURITY CONFIGURATION ====================
    /// TLS/SSL configuration
    pub tls: TlsConfig,

    /// Security settings
    pub security: ApiSecurityConfig,

    // ==================== PERFORMANCE CONFIGURATION ====================
    /// Performance optimization settings
    pub performance: ApiPerformanceConfig,

    // ==================== RATE LIMITING ====================
    /// Rate limiting configuration
    pub rate_limiting: RateLimitingConfig,

    // ==================== MONITORING & HEALTH ====================
    /// Monitoring and observability settings
    pub monitoring: ApiMonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Tls
pub struct TlsConfig {
    /// Cert Path
    pub cert_path: String,
    /// Key Path
    pub key_path: String,
    /// Ca Path
    pub ca_path: Option<String>,
    /// Verify Client
    pub verify_client: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `RateLimiting`
pub struct RateLimitingConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Requests Per Second
    pub requests_per_second: u32,
    /// Size of burst
    pub burst_size: u32,
}

/// **API SECURITY CONFIGURATION**
///
/// Authentication, authorization, and security settings for the API.
/// Consolidates security patterns from `UnifiedApiConfig`.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ApiSecurity`
pub struct ApiSecurityConfig {
    /// Enable authentication for API endpoints
    pub auth_enabled: bool,

    /// JWT secret for token signing (None = load from environment)
    pub jwt_secret: Option<String>,

    /// API key authentication enabled
    pub api_key_enabled: bool,

    /// Allowed CORS origins
    pub cors_origins: Vec<String>,

    /// Enable request signing
    pub request_signing_enabled: bool,

    /// Enable audit logging for security events
    pub audit_logging_enabled: bool,
}

/// **API PERFORMANCE CONFIGURATION**
///
/// Performance optimization and resource management settings.
/// Consolidates performance patterns from `UnifiedApiConfig`.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ApiPerformance`
pub struct ApiPerformanceConfig {
    /// Request buffer size in bytes
    pub buffer_size: usize,

    /// Thread pool size for request handling
    pub thread_pool_size: usize,

    /// Response cache size in bytes
    pub cache_size: u64,

    /// Enable response compression (gzip/brotli)
    pub compression_enabled: bool,

    /// Enable HTTP/2 support
    pub http2_enabled: bool,

    /// Enable connection pooling
    pub connection_pooling_enabled: bool,

    /// Maximum request body size in bytes
    pub max_request_body_size: usize,
}

/// **API MONITORING CONFIGURATION**
///
/// Observability, metrics, and health check settings.
/// Consolidates monitoring patterns from `UnifiedApiConfig`.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ApiMonitoring`
pub struct ApiMonitoringConfig {
    /// Enable Prometheus metrics collection
    pub metrics_enabled: bool,

    /// Metrics endpoint path
    pub metrics_path: String,

    /// Enable health check endpoint
    pub health_checks_enabled: bool,

    /// Health check endpoint path
    pub health_path: String,

    /// Enable distributed tracing (OpenTelemetry)
    pub tracing_enabled: bool,

    /// Enable request/response logging
    pub request_logging_enabled: bool,

    /// Enable performance profiling
    pub profiling_enabled: bool,

    /// Alert thresholds configuration
    pub alerts: ApiAlertConfig,
}

/// **API ALERT CONFIGURATION**
///
/// Threshold-based alerting for API health and performance.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ApiAlert`
pub struct ApiAlertConfig {
    /// Alert when error rate exceeds this percentage
    pub error_rate_threshold: f64,

    /// Alert when response time exceeds this duration (ms)
    pub response_time_threshold_ms: u64,

    /// Alert when CPU usage exceeds this percentage
    pub cpu_threshold: f64,

    /// Alert when memory usage exceeds this percentage
    pub memory_threshold: f64,
}

impl ApiConfig {
    /// Creates a development-optimized API configuration
    ///
    /// Returns an `ApiConfig` with relaxed security, verbose logging, permissive rate limiting,
    /// and debugging features enabled suitable for local development environments.
    #[must_use]
    pub fn development_optimized() -> Self {
        let discovery_config = crate::config::discovery_config::ServiceDiscoveryConfig::default();
        Self {
            // Network settings
            bind_address: discovery_config
                .discovery_host
                .parse()
                .unwrap_or(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)),
            port: discovery_config.discovery_base_port,
            max_connections: 100,
            request_timeout: Duration::from_secs(30),
            connection_timeout: Duration::from_secs(10),
            port_range_start: 9000,
            port_range_end: 9999,

            // Metadata
            enabled: true,
            version: "v1".to_string(),
            api_settings: HashMap::new(),

            // Security (relaxed for development)
            tls: TlsConfig::default(),
            security: ApiSecurityConfig::development(),

            // Performance (balanced)
            performance: ApiPerformanceConfig::development(),

            // Rate limiting (permissive)
            rate_limiting: RateLimitingConfig::development(),

            // Monitoring (verbose)
            monitoring: ApiMonitoringConfig::development(),
        }
    }

    /// Creates a production-hardened API configuration
    ///
    /// Returns an `ApiConfig` with strict security, TLS enabled, rate limiting, authentication,
    /// and production-grade monitoring suitable for production deployments.
    #[must_use]
    pub fn production_hardened() -> Self {
        use crate::constants::hardcoding::{addresses, runtime_fallback_ports};
        Self {
            // Network settings
            bind_address: addresses::BIND_ALL_IPV4
                .parse()
                .unwrap_or(std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED)),
            port: runtime_fallback_ports::HTTPS,
            max_connections: 1000,
            request_timeout: Duration::from_secs(60),
            connection_timeout: Duration::from_secs(30),
            port_range_start: 10000,
            port_range_end: 19999,

            // Metadata
            enabled: true,
            version: "v1".to_string(),
            api_settings: HashMap::new(),

            // Security (strict)
            tls: TlsConfig::production(),
            security: ApiSecurityConfig::production(),

            // Performance (optimized)
            performance: ApiPerformanceConfig::production(),

            // Rate limiting (strict)
            rate_limiting: RateLimitingConfig::production(),

            // Monitoring (production-grade)
            monitoring: ApiMonitoringConfig::production(),
        }
    }

    /// Validates the API configuration for correctness
    ///
    /// Checks that port is non-zero, max connections is non-zero, and port range is valid.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails (port is zero, max connections is zero, or
    /// port range start is greater than or equal to port range end).
    pub fn validate(&self) -> Result<()> {
        if self.port == 0 {
            return Err(NestGateError::validation_error("Port cannot be zero"));
        }
        if self.max_connections == 0 {
            return Err(NestGateError::validation_error(
                "Max connections cannot be zero",
            ));
        }
        if self.port_range_start >= self.port_range_end {
            return Err(NestGateError::validation_error(
                "Port range start must be less than port range end",
            ));
        }
        Ok(())
    }

    /// Merges this configuration with another, preferring values from `other`
    ///
    /// Takes all values from `other` and overwrites the current configuration.
    /// Useful for layering configurations (e.g., defaults + environment overrides).
    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.bind_address = other.bind_address;
        self.port = other.port;
        self.max_connections = other.max_connections;
        self.request_timeout = other.request_timeout;
        self.connection_timeout = other.connection_timeout;
        self.port_range_start = other.port_range_start;
        self.port_range_end = other.port_range_end;
        self.enabled = other.enabled;
        self.version = other.version;
        self.api_settings = other.api_settings;
        self.tls = other.tls;
        self.security = other.security;
        self.performance = other.performance;
        self.rate_limiting = other.rate_limiting;
        self.monitoring = other.monitoring;
        self
    }
}

impl Default for TlsConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            cert_path: "/etc/ssl/certs/nestgate.pem".to_string(),
            key_path: "/etc/ssl/private/nestgate.key".to_string(),
            ca_path: None,
            verify_client: false,
        }
    }
}

impl TlsConfig {
    /// Creates a production TLS configuration with strict security
    ///
    /// Returns a `TlsConfig` with production certificates, CA bundle, and client
    /// verification enabled for secure production deployments.
    #[must_use]
    pub fn production() -> Self {
        Self {
            cert_path: "/etc/ssl/certs/nestgate-prod.pem".to_string(),
            key_path: "/etc/ssl/private/nestgate-prod.key".to_string(),
            ca_path: Some("/etc/ssl/certs/ca-bundle.pem".to_string()),
            verify_client: true,
        }
    }
}

impl Default for RateLimitingConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::development()
    }
}

impl RateLimitingConfig {
    /// Creates a development rate limiting configuration with relaxed limits
    ///
    /// Returns a `RateLimitingConfig` with rate limiting disabled and high limits
    /// suitable for local development and testing.
    #[must_use]
    pub const fn development() -> Self {
        Self {
            enabled: false,
            requests_per_second: 1000,
            burst_size: 2000,
        }
    }

    /// Creates a production rate limiting configuration with strict limits
    ///
    /// Returns a `RateLimitingConfig` with rate limiting enabled and conservative limits
    /// to protect against abuse in production environments.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            enabled: true,
            requests_per_second: 100,
            burst_size: 200,
        }
    }
}

impl Default for ApiSecurityConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::development()
    }
}

impl ApiSecurityConfig {
    /// Creates a development API security configuration with relaxed settings
    ///
    /// Returns an `ApiSecurityConfig` with authentication disabled, CORS permissive,
    /// and security features relaxed for local development.
    #[must_use]
    pub fn development() -> Self {
        Self {
            auth_enabled: false,
            jwt_secret: None,
            api_key_enabled: false,
            cors_origins: vec!["*".to_string()],
            request_signing_enabled: false,
            audit_logging_enabled: true,
        }
    }

    /// Creates a production API security configuration with strict authentication
    ///
    /// Returns an `ApiSecurityConfig` with all security features enabled, authentication
    /// required, and strict CORS policies for production deployments.
    #[must_use]
    pub fn production() -> Self {
        Self {
            auth_enabled: true,
            jwt_secret: std::env::var("NESTGATE_JWT_SECRET").ok(),
            api_key_enabled: true,
            cors_origins: vec![], // Must be explicitly configured
            request_signing_enabled: true,
            audit_logging_enabled: true,
        }
    }
}

impl Default for ApiPerformanceConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::development()
    }
}

impl ApiPerformanceConfig {
    /// Creates a development API performance configuration
    ///
    /// Returns an `ApiPerformanceConfig` with balanced settings, compression disabled,
    /// and debugging features enabled for local development.
    #[must_use]
    pub const fn development() -> Self {
        use crate::constants::canonical_defaults::{
            concurrency::DEFAULT_THREAD_POOL_SIZE,
            sizes::{DEFAULT_BUFFER_SIZE, DEFAULT_CACHE_SIZE},
        };
        Self {
            buffer_size: DEFAULT_BUFFER_SIZE,
            thread_pool_size: DEFAULT_THREAD_POOL_SIZE,
            cache_size: DEFAULT_CACHE_SIZE,
            compression_enabled: false,
            http2_enabled: false,
            connection_pooling_enabled: true,
            max_request_body_size: 10 * 1024 * 1024, // 10 MB
        }
    }

    /// Creates a production API performance configuration with optimized settings
    ///
    /// Returns an `ApiPerformanceConfig` with larger buffers, more threads, compression
    /// enabled, HTTP/2 support, and production-grade caching.
    #[must_use]
    pub const fn production() -> Self {
        use crate::constants::canonical_defaults::{
            concurrency::DEFAULT_THREAD_POOL_SIZE,
            sizes::{DEFAULT_BUFFER_SIZE, DEFAULT_CACHE_SIZE},
        };
        Self {
            buffer_size: DEFAULT_BUFFER_SIZE * 2,
            thread_pool_size: DEFAULT_THREAD_POOL_SIZE * 2,
            cache_size: DEFAULT_CACHE_SIZE * 4,
            compression_enabled: true,
            http2_enabled: true,
            connection_pooling_enabled: true,
            max_request_body_size: 100 * 1024 * 1024, // 100 MB
        }
    }
}

impl Default for ApiMonitoringConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::development()
    }
}

impl ApiMonitoringConfig {
    /// Creates a development API monitoring configuration
    ///
    /// Returns an `ApiMonitoringConfig` with all monitoring features enabled,
    /// verbose logging, and profiling for local development and debugging.
    #[must_use]
    pub fn development() -> Self {
        Self {
            metrics_enabled: true,
            metrics_path: "/api/v1/monitoring/metrics".to_string(),
            health_checks_enabled: true,
            health_path: "/health".to_string(),
            tracing_enabled: true,
            request_logging_enabled: true,
            profiling_enabled: true,
            alerts: ApiAlertConfig::default(),
        }
    }

    /// Creates a production API monitoring configuration with comprehensive checks
    ///
    /// Returns an `ApiMonitoringConfig` with metrics, health checks, and tracing enabled,
    /// but with verbose logging and profiling disabled for production efficiency.
    #[must_use]
    pub fn production() -> Self {
        Self {
            metrics_enabled: true,
            metrics_path: "/api/v1/monitoring/metrics".to_string(),
            health_checks_enabled: true,
            health_path: "/health".to_string(),
            tracing_enabled: true,
            request_logging_enabled: false, // Reduce overhead in production
            profiling_enabled: false,       // Disable unless debugging
            alerts: ApiAlertConfig::production(),
        }
    }
}

impl Default for ApiAlertConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            error_rate_threshold: 5.0,        // 5%
            response_time_threshold_ms: 1000, // 1 second
            cpu_threshold: 80.0,              // 80%
            memory_threshold: 85.0,           // 85%
        }
    }
}

impl ApiAlertConfig {
    /// Creates a production alert configuration with strict thresholds
    ///
    /// Returns an `ApiAlertConfig` with conservative thresholds for error rates,
    /// response times, CPU, and memory usage suitable for production alerting.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            error_rate_threshold: 1.0,       // 1% (stricter)
            response_time_threshold_ms: 500, // 500ms (stricter)
            cpu_threshold: 70.0,             // 70% (stricter)
            memory_threshold: 75.0,          // 75% (stricter)
        }
    }
}

impl Default for ApiConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::development_optimized()
    }
}
