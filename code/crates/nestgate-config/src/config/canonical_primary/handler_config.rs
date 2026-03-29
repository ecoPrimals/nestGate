// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **CANONICAL HANDLER CONFIGURATION MODULE**
//! Handler Config functionality and utilities.
// This module consolidates ALL scattered handler configuration structures
//! into a single canonical system, eliminating 50+ fragmented config structs.
//! Handler Config functionality and utilities.
// **CONSOLIDATES AND REPLACES**:
//! - `ZfsServiceConfig` from nestgate-api/src/handlers/zfs/universal_zfs/config.rs
//! - `UnifiedHandlerConfig` from nestgate-api/src/handlers/unified_handler_config.rs
//! - `PerformanceAnalysisConfig` from nestgate-api/src/handlers/performance_analyzer.rs
//! - `LoadTestConfig` from nestgate-api/src/handlers/load_testing.rs
//! - All other handler-specific configuration structs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ==================== SECTION ====================

/// **CANONICAL HANDLER CONFIGURATIONS**
///
/// Single source of truth for ALL handler configurations across `NestGate`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Canonicalhandlerconfigs
pub struct CanonicalHandlerConfigs {
    /// Global settings applied to all handlers
    pub global: GlobalHandlerConfig,

    /// ZFS handler configuration
    pub zfs: ZfsHandlerConfig,

    /// Performance analysis handler configuration
    pub performance: PerformanceHandlerConfig,

    /// Load testing handler configuration
    pub load_testing: LoadTestHandlerConfig,

    /// Workspace management handler configuration
    pub workspace: WorkspaceHandlerConfig,

    /// Hardware tuning handler configuration
    pub hardware_tuning: HardwareTuningHandlerConfig,

    /// Compliance handler configuration
    pub compliance: ComplianceHandlerConfig,

    /// Dashboard handler configuration
    pub dashboard: DashboardHandlerConfig,

    /// Custom handler configurations (for extensibility)
    pub custom: HashMap<String, CustomHandlerConfig>,
}

// ==================== SECTION ====================

/// **GLOBAL HANDLER CONFIGURATION**
///
/// Settings applied to all handlers by default
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `GlobalHandler`
pub struct GlobalHandlerConfig {
    /// Default timeout for handler operations
    pub default_timeout: Duration,
    /// Maximum request body size (bytes)
    pub max_request_size: usize,
    /// Enable request/response logging
    pub enable_logging: bool,
    /// Enable metrics collection
    pub enable_metrics: bool,
    /// Default content type for responses
    pub default_content_type: String,
    /// Enable CORS for all handlers
    pub enable_cors: bool,
    /// Allowed CORS origins
    pub cors_origins: Vec<String>,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitConfig,
    /// Security configuration
    pub security: HandlerSecurityConfig,
}
impl Default for GlobalHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            default_timeout: Duration::from_secs(30),
            max_request_size: 10 * 1024 * 1024, // 10MB
            enable_logging: true,
            enable_metrics: true,
            default_content_type: "application/json".to_string(),
            enable_cors: true,
            cors_origins: vec!["*".to_string()],
            rate_limiting: RateLimitConfig::default(),
            security: HandlerSecurityConfig::default(),
        }
    }
}

// ==================== SECTION ====================

/// **ZFS HANDLER CONFIGURATION**
///
/// Consolidates `ZfsServiceConfig` and related ZFS handler configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ZfsHandler`
pub struct ZfsHandlerConfig {
    /// Service name for ZFS handler
    pub service_name: String,
    /// Backend configuration
    pub backend: ZfsBackendConfig,
    /// Fail-safe configuration
    pub fail_safe: ZfsFailSafeConfig,
    /// Observability configuration
    pub observability: ZfsObservabilityConfig,
    /// Performance configuration
    pub performance: ZfsPerformanceConfig,
    /// Security configuration
    pub security: ZfsSecurityConfig,
    /// Custom properties
    pub custom_properties: HashMap<String, String>,
}
impl Default for ZfsHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            service_name: "canonical-zfs-handler".to_string(),
            backend: ZfsBackendConfig::Auto,
            fail_safe: ZfsFailSafeConfig::default(),
            observability: ZfsObservabilityConfig::default(),
            performance: ZfsPerformanceConfig::default(),
            security: ZfsSecurityConfig::default(),
            custom_properties: HashMap::new(),
        }
    }
}

impl ZfsHandlerConfig {
    /// Validate the configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn validate(&self) -> Result<(), String> {
        if self.service_name.is_empty() {
            return Err("Service name cannot be empty".to_string());
        }
        // Additional validation can be added here
        Ok(())
    }
}

/// **ZFS BACKEND CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zfsbackendconfig
pub enum ZfsBackendConfig {
    /// Automatically detect the best backend
    Auto,
    /// Use native ZFS commands
    Native,
    /// Use development environment compatibility layer
    Development,
    /// Use remote ZFS service
    Remote {
        /// Remote service endpoint URL
        endpoint: String,
        /// Connection timeout duration
        timeout: Duration,
    },
}
/// **ZFS FAIL-SAFE CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ZfsFailSafe`
pub struct ZfsFailSafeConfig {
    /// Enable circuit breaker
    pub enable_circuit_breaker: bool,
    /// Circuit breaker failure threshold
    pub failure_threshold: u32,
    /// Circuit breaker timeout
    pub circuit_timeout: Duration,
    /// Enable graceful degradation
    pub enable_graceful_degradation: bool,
    /// Circuit breaker configuration (for compatibility)
    pub circuit_breaker: CircuitBreakerConfig,
    /// Retry policy configuration (for compatibility)
    pub retry_policy: RetryPolicyConfig,
    /// Fallback enabled (for compatibility)
    pub fallback_enabled: bool,
}

/// Circuit breaker configuration for fault tolerance
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `CircuitBreaker`
pub struct CircuitBreakerConfig {
    /// Whether circuit breaker is enabled
    pub enabled: bool,
}

/// Retry policy configuration for resilience
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `RetryPolicy`
pub struct RetryPolicyConfig {
    /// Whether retry policy is enabled
    pub enabled: bool,
}
impl Default for ZfsFailSafeConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enable_circuit_breaker: true,
            failure_threshold: 5,
            circuit_timeout: Duration::from_secs(60),
            enable_graceful_degradation: true,
            circuit_breaker: CircuitBreakerConfig { enabled: true },
            retry_policy: RetryPolicyConfig { enabled: true },
            fallback_enabled: true,
        }
    }
}

impl Default for CircuitBreakerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for RetryPolicyConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// **ZFS OBSERVABILITY CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ZfsObservability`
pub struct ZfsObservabilityConfig {
    /// Enable metrics collection
    pub enable_metrics: bool,
    /// Enable distributed tracing
    pub enable_tracing: bool,
    /// Metrics collection interval
    pub metrics_interval: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
}
impl Default for ZfsObservabilityConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enable_metrics: true,
            enable_tracing: true,
            metrics_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
        }
    }
}

/// **ZFS PERFORMANCE CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ZfsPerformance`
pub struct ZfsPerformanceConfig {
    /// Connection pool size
    pub pool_size: usize,
    /// Request timeout
    pub request_timeout: Duration,
    /// Enable connection pooling
    pub enable_pooling: bool,
    /// Cache configuration
    pub cache_size: usize,
    /// Enable zero-copy operations
    pub enable_zero_copy: bool,
}
impl Default for ZfsPerformanceConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            pool_size: 10,
            request_timeout: Duration::from_secs(30),
            enable_pooling: true,
            cache_size: 1000,
            enable_zero_copy: true,
        }
    }
}

/// **ZFS SECURITY CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ZfsSecurity`
pub struct ZfsSecurityConfig {
    /// Require authentication
    pub require_auth: bool,
    /// Enable request signing
    pub enable_signing: bool,
    /// Enable encryption
    pub enable_encryption: bool,
    /// Allowed operations
    pub allowed_operations: Vec<String>,
}
impl Default for ZfsSecurityConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            require_auth: true,
            enable_signing: false,
            enable_encryption: true,
            allowed_operations: vec![
                "list_pools".to_string(),
                "create_dataset".to_string(),
                "snapshot".to_string(),
            ],
        }
    }
}

// ==================== SECTION ====================

/// **PERFORMANCE HANDLER CONFIGURATION**
///
/// Consolidates `PerformanceAnalysisConfig` and related performance handler configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `PerformanceHandler`
pub struct PerformanceHandlerConfig {
    /// Enable performance monitoring
    pub enable_monitoring: bool,
    /// Monitoring interval
    pub monitoring_interval: Duration,
    /// Enable benchmarking
    pub enable_benchmarking: bool,
    /// Benchmark configuration
    pub benchmark: BenchmarkConfig,
    /// Analysis configuration
    pub analysis: AnalysisConfig,
}
impl Default for PerformanceHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enable_monitoring: true,
            monitoring_interval: Duration::from_secs(60),
            enable_benchmarking: true,
            benchmark: BenchmarkConfig::default(),
            analysis: AnalysisConfig::default(),
        }
    }
}

/// **BENCHMARK CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Benchmark
pub struct BenchmarkConfig {
    /// Number of benchmark iterations
    pub iterations: u32,
    /// Benchmark timeout
    pub timeout: Duration,
    /// Enable detailed profiling
    pub enable_profiling: bool,
}
impl Default for BenchmarkConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            iterations: 100,
            timeout: Duration::from_secs(300),
            enable_profiling: false,
        }
    }
}

/// **ANALYSIS CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Analysis
pub struct AnalysisConfig {
    /// Enable statistical analysis
    pub enable_statistics: bool,
    /// Enable trend analysis
    pub enable_trends: bool,
    /// Analysis window size
    pub window_size: usize,
}
impl Default for AnalysisConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enable_statistics: true,
            enable_trends: true,
            window_size: 1000,
        }
    }
}

// ==================== SECTION ====================

/// **LOAD TEST HANDLER CONFIGURATION**
///
/// Consolidates `LoadTestConfig` and related load testing configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `LoadTestHandler`
pub struct LoadTestHandlerConfig {
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Test duration
    pub test_duration: Duration,
    /// Ramp-up duration
    pub ramp_up_duration: Duration,
    /// Target requests per second
    pub target_rps: u32,
    /// Enable real-time metrics
    pub enable_real_time_metrics: bool,
}
impl Default for LoadTestHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            max_connections: 100,
            test_duration: Duration::from_secs(300),
            ramp_up_duration: Duration::from_secs(60),
            target_rps: 100,
            enable_real_time_metrics: true,
        }
    }
}

/// **WORKSPACE HANDLER CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `WorkspaceHandler`
pub struct WorkspaceHandlerConfig {
    /// Default workspace size limit
    pub default_size_limit: u64,
    /// Enable workspace sharing
    pub enable_sharing: bool,
    /// Enable workspace templates
    pub enable_templates: bool,
    /// Cleanup policy
    pub cleanup_policy: WorkspaceCleanupPolicy,
}
impl Default for WorkspaceHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            default_size_limit: 10 * 1024 * 1024 * 1024, // 10GB
            enable_sharing: false, // Intentionally disabled - requires external auth
            enable_templates: false, // Low priority feature
            cleanup_policy: WorkspaceCleanupPolicy::Manual,
        }
    }
}

/// **WORKSPACE CLEANUP POLICY**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Workspacecleanuppolicy
pub enum WorkspaceCleanupPolicy {
    /// Manual cleanup - requires explicit user action
    Manual,
    /// Automatic cleanup based on retention period
    Automatic {
        /// Number of days to retain workspace data
        retention_days: u32,
    },
    /// On-demand cleanup - triggered by system events
    OnDemand,
}
/// **HARDWARE TUNING HANDLER CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `HardwareTuningHandler`
pub struct HardwareTuningHandlerConfig {
    /// Enable automatic tuning
    pub enable_auto_tuning: bool,
    /// Tuning interval
    pub tuning_interval: Duration,
    /// Enable CPU optimization
    pub enable_cpu_optimization: bool,
    /// Enable memory optimization
    pub enable_memory_optimization: bool,
}
impl Default for HardwareTuningHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enable_auto_tuning: true,
            tuning_interval: Duration::from_secs(300),
            enable_cpu_optimization: true,
            enable_memory_optimization: true,
        }
    }
}

/// **COMPLIANCE HANDLER CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ComplianceHandler`
pub struct ComplianceHandlerConfig {
    /// Enable compliance monitoring
    pub enable_monitoring: bool,
    /// Compliance standards to check
    pub standards: Vec<String>,
    /// Audit interval
    pub audit_interval: Duration,
}
impl Default for ComplianceHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enable_monitoring: true,
            standards: vec!["ISO27001".to_string(), "GDPR".to_string()],
            audit_interval: Duration::from_secs(3600),
        }
    }
}

/// **DASHBOARD HANDLER CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `DashboardHandler`
pub struct DashboardHandlerConfig {
    /// Enable real-time updates
    pub enable_real_time: bool,
    /// Update interval
    pub update_interval: Duration,
    /// Enable caching
    pub enable_caching: bool,
    /// Cache TTL
    pub cache_ttl: Duration,
}
impl Default for DashboardHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enable_real_time: true,
            update_interval: Duration::from_secs(5),
            enable_caching: true,
            cache_ttl: Duration::from_secs(60),
        }
    }
}

// ==================== SECTION ====================

/// **CUSTOM HANDLER CONFIGURATION**
///
/// Generic configuration for custom handlers
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `CustomHandler`
pub struct CustomHandlerConfig {
    /// Handler name
    pub name: String,
    /// Handler-specific settings
    pub settings: HashMap<String, serde_json::Value>,
    /// Handler timeout
    pub timeout: Duration,
    /// Enable metrics for this handler
    pub enable_metrics: bool,
}
/// **RATE LIMIT CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `RateLimit`
pub struct RateLimitConfig {
    /// Requests per minute
    pub requests_per_minute: u32,
    /// Burst size
    pub burst_size: u32,
    /// Enable rate limiting
    pub enabled: bool,
}
impl Default for RateLimitConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            requests_per_minute: 1000,
            burst_size: 100,
            enabled: true,
        }
    }
}

/// **HANDLER SECURITY CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `HandlerSecurity`
pub struct HandlerSecurityConfig {
    /// Require authentication
    pub require_auth: bool,
    /// Enable CSRF protection
    pub enable_csrf: bool,
    /// Enable request signing
    pub enable_signing: bool,
    /// Security headers
    pub security_headers: HashMap<String, String>,
}
impl Default for HandlerSecurityConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            require_auth: true,
            enable_csrf: true,
            enable_signing: false,
            security_headers: {
                let mut headers = HashMap::new();
                headers.insert("X-Content-Type-Options".to_string(), "nosniff".to_string());
                headers.insert("X-Frame-Options".to_string(), "DENY".to_string());
                headers.insert("X-XSS-Protection".to_string(), "1; mode=block".to_string());
                headers
            },
        }
    }
}
