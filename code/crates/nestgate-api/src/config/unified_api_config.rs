/// Unified API Configuration
/// Consolidates fragmented API config structs (PerformanceConfig, HealthCheckConfig, 
/// MetricsConfig, TlsConfig, CorsConfig, etc.) into the standardized pattern.
/// **PROBLEM SOLVED**: Eliminates 6+ duplicate config structs with unified approach.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

// Import the standardized config pattern
use nestgate_core::unified_config_consolidation::StandardDomainConfig;

// ==================== API-SPECIFIC EXTENSIONS ====================

/// API-specific configuration extensions
/// Domain-specific fields that don't belong in unified base configs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiExtensions {
    /// Performance monitoring settings
    pub performance: ApiPerformanceSettings,
    /// Health check configuration
    pub health_checks: ApiHealthCheckSettings,
    /// Metrics and observability
    pub metrics: ApiMetricsSettings,
    /// TLS/SSL configuration
    pub tls: ApiTlsSettings,
    /// CORS configuration
    pub cors: ApiCorsSettings,
    /// Authentication settings
    pub auth: ApiAuthSettings,
    /// Server-specific settings
    pub server: ApiServerSettings,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPerformanceSettings {
    pub enable_cpu_monitoring: bool,
    pub enable_memory_monitoring: bool,
    pub enable_disk_monitoring: bool,
    pub enable_network_monitoring: bool,
    pub sample_interval_seconds: u64,
    pub alert_thresholds: HashMap<String, f64>,
    pub performance_targets: HashMap<String, f64>,
    pub enable_profiling: bool,
    pub profiling_interval_seconds: u64,
    pub max_performance_history: usize,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiHealthCheckSettings {
    pub enabled: bool,
    pub check_interval: Duration,
    pub timeout: Duration,
    pub failure_threshold: u32,
    pub success_threshold: u32,
    pub startup_delay: Duration,
    pub health_check_endpoint: String,
    pub custom_checks: Vec<String>,
    pub notify_on_failure: bool,
    pub retry_attempts: u32,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMetricsSettings {
    pub enabled: bool,
    pub metrics_port: u16,
    pub metrics_path: String,
    pub export_interval: Duration,
    pub retention_period: Duration,
    pub enable_histogram: bool,
    pub enable_counter: bool,
    pub enable_gauge: bool,
    pub custom_metrics: Vec<String>,
    pub prometheus_enabled: bool,
    pub grafana_endpoint: Option<String>,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiTlsSettings {
    pub enabled: bool,
    pub cert_file: Option<PathBuf>,
    pub key_file: Option<PathBuf>,
    pub ca_file: Option<PathBuf>,
    pub verify_client: bool,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCorsSettings {
    pub enabled: bool,
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub allow_credentials: bool,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuthSettings {
    pub auth_required: bool,
    pub token_expiration_hours: u64,
    pub refresh_token_enabled: bool,
    pub max_sessions_per_user: u32,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiServerSettings {
    pub host: String,
    pub port: u16,
    pub workers: Option<u32>,
    pub max_connections: Option<u32>,
    pub request_timeout_ms: Option<u64>,
    }

impl Default for ApiExtensions {
    fn default() -> Self {
        Self {
            performance: ApiPerformanceSettings {
                enable_cpu_monitoring: true,
                enable_memory_monitoring: true,
                enable_disk_monitoring: true,
                enable_network_monitoring: true,
                sample_interval_seconds: 30,
                alert_thresholds: {
                    let mut thresholds = HashMap::new();
                    thresholds.insert("cpu_usage".to_string(), 80.0);
                    thresholds.insert("memory_usage".to_string(), 85.0);
                    thresholds.insert("disk_usage".to_string(), 90.0);
                    thresholds
                },
                performance_targets: {
                    let mut targets = HashMap::new();
                    targets.insert("response_time_ms".to_string(), 200.0);
                    targets.insert("throughput_rps".to_string(), 1000.0);
                    targets
                },
                enable_profiling: false,
                profiling_interval_seconds: 300,
                max_performance_history: 1000,
            },
            health_checks: ApiHealthCheckSettings {
                enabled: true,
                check_interval: Duration::from_secs(30),
                timeout: Duration::from_secs(10),
                failure_threshold: 3,
                success_threshold: 1,
                startup_delay: Duration::from_secs(10),
                health_check_endpoint: "/health".to_string(),
                custom_checks: Vec::new(),
                notify_on_failure: true,
                retry_attempts: 3,
            },
            metrics: ApiMetricsSettings {
                enabled: true,
                metrics_port: 9090,
                metrics_path: "/metrics".to_string(),
                export_interval: Duration::from_secs(15),
                retention_period: Duration::from_secs(3600),
                enable_histogram: true,
                enable_counter: true,
                enable_gauge: true,
                custom_metrics: Vec::new(),
                prometheus_enabled: true,
                grafana_endpoint: None,
            },
            tls: ApiTlsSettings {
                enabled: false,
                cert_file: None,
                key_file: None,
                ca_file: None,
                verify_client: false,
            },
            cors: ApiCorsSettings {
                enabled: true,
                allowed_origins: vec!["*".to_string()],
                allowed_methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
                allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
                allow_credentials: false,
            },
            auth: ApiAuthSettings {
                auth_required: true,
                token_expiration_hours: 24,
                refresh_token_enabled: true,
                max_sessions_per_user: 5,
            },
            server: ApiServerSettings {
                host: "0.0.0.0".to_string(),
                port: 8080,
                workers: None,
                max_connections: Some(1000),
                request_timeout_ms: Some(30000),
            },
    }
    }
    }

// ==================== TYPE ALIAS FOR UNIFIED API CONFIG ====================

/// **THE** unified API configuration using the standardized pattern
/// Replaces all fragmented API config structs with a single, comprehensive configuration
pub type UnifiedApiConfig = StandardDomainConfig<ApiExtensions>;

// ==================== MIGRATION UTILITIES ====================

/// Migration utilities for converting legacy API configs to unified pattern
pub mod migration {
    use super::*;
    use std::collections::HashMap;

    // Import legacy types for migration
    // REMOVED: Imports from deleted deprecated config modules
// All configuration now handled through UnifiedApiConfig and ApiExtensions
    use crate::config::network::ServerConfig;

    /// Migrate legacy API configs to unified pattern
    pub fn migrate_api_configs(
        performance: Option<PerformanceConfig>,
        health_check: Option<HealthCheckConfig>,
        metrics: Option<MetricsConfig>,
        tls: Option<TlsConfig>,
        cors: Option<CorsConfig>,
        auth: Option<PrimalAuthConfig>,
        server: Option<ServerConfig>,
    ) -> UnifiedApiConfig {
        let extensions = ApiExtensions {
            performance: performance.map(|p| ApiPerformanceSettings {
                enable_cpu_monitoring: p.enable_cpu_monitoring,
                enable_memory_monitoring: p.enable_memory_monitoring,
                enable_disk_monitoring: p.enable_disk_monitoring,
                enable_network_monitoring: p.enable_network_monitoring,
                sample_interval_seconds: p.sample_interval_seconds,
                alert_thresholds: p.alert_thresholds,
                performance_targets: p.performance_targets,
                enable_profiling: p.enable_profiling,
                profiling_interval_seconds: p.profiling_interval_seconds,
                max_performance_history: p.max_performance_history,
            }).unwrap_or_default(),
            
            health_checks: health_check.map(|h| ApiHealthCheckSettings {
                enabled: h.enabled,
                check_interval: h.check_interval,
                timeout: h.timeout,
                failure_threshold: h.failure_threshold,
                success_threshold: h.success_threshold,
                startup_delay: h.startup_delay,
                health_check_endpoint: h.health_check_endpoint,
                custom_checks: h.custom_checks,
                notify_on_failure: h.notify_on_failure,
                retry_attempts: h.retry_attempts,
            }).unwrap_or_default(),
            
            metrics: metrics.map(|m| ApiMetricsSettings {
                enabled: m.enabled,
                metrics_port: m.metrics_port,
                metrics_path: m.metrics_path,
                export_interval: m.export_interval,
                retention_period: m.retention_period,
                enable_histogram: m.enable_histogram,
                enable_counter: m.enable_counter,
                enable_gauge: m.enable_gauge,
                custom_metrics: m.custom_metrics,
                prometheus_enabled: m.prometheus_enabled,
                grafana_endpoint: m.grafana_endpoint,
            }).unwrap_or_default(),
            
            tls: tls.map(|t| ApiTlsSettings {
                enabled: t.enabled,
                cert_file: t.cert_file,
                key_file: t.key_file,
                ca_file: t.ca_file,
                verify_client: t.verify_client,
            }).unwrap_or_default(),
            
            cors: cors.map(|c| ApiCorsSettings {
                enabled: c.enabled,
                allowed_origins: c.allowed_origins,
                allowed_methods: c.allowed_methods,
                allowed_headers: c.allowed_headers,
                allow_credentials: c.allow_credentials,
            }).unwrap_or_default(),
            
            auth: auth.map(|a| ApiAuthSettings {
                auth_required: a.auth_required,
                token_expiration_hours: a.token_expiration_hours,
                refresh_token_enabled: a.refresh_token_enabled,
                max_sessions_per_user: a.max_sessions_per_user,
            }).unwrap_or_default(),
            
            server: server.map(|s| ApiServerSettings {
                host: s.host,
                port: s.port,
                workers: s.workers,
                max_connections: s.max_connections,
                request_timeout_ms: s.request_timeout_ms,
            }).unwrap_or_default(),
        };

        StandardDomainConfig::with_service(extensions, "nestgate-api", env!("CARGO_PKG_VERSION"))
    }

    /// Convenience function for creating production API config
    pub fn create_production_api_config() -> UnifiedApiConfig {
        let mut config = StandardDomainConfig::with_service(
            ApiExtensions::default(),
            "nestgate-api",
            env!("CARGO_PKG_VERSION")
        );

        // Production optimizations
        config.network.max_connections = 2000;
        config.security.enable_tls = true;
        config.monitoring.enable_metrics = true;
        config.monitoring.enable_tracing = true;
        
        // Production-specific API settings
        config.extensions.performance.enable_profiling = true;
        config.extensions.metrics.prometheus_enabled = true;
        config.extensions.tls.enabled = true;
        config.extensions.auth.auth_required = true;

        config
    }
    }

// ==================== HELPER FUNCTIONS ====================

impl UnifiedApiConfig {
    /// Create a comprehensive API configuration for development
    pub fn development() -> Self {
        let extensions = ApiExtensions::default();
        StandardDomainConfig::with_service(extensions, "nestgate-api", env!("CARGO_PKG_VERSION"))
    }

    /// Create a production-ready API configuration
    pub fn production() -> Self {
        let mut config = Self::development();
        
        // Production-specific settings
        config.extensions.performance.enable_profiling = false;
        config.extensions.performance.sample_interval_seconds = 60;
        config.extensions.health_checks.failure_threshold = 5;
        config.extensions.metrics.prometheus_enabled = true;
        config.extensions.tls.enabled = true;
        config.extensions.auth.auth_required = true;
        
        config
    }

    /// Create a high-performance API configuration
    pub fn high_performance() -> Self {
        let mut config = Self::development();
        
        // High performance settings
        config.extensions.performance.enable_profiling = true;
        config.extensions.performance.sample_interval_seconds = 10;
        config.extensions.performance.max_performance_history = 10000;
        config.extensions.server.enable_compression = true;
        config.extensions.server.max_request_size = 10 * 1024 * 1024; // 10MB
        
        config
    }

    /// Create a secure API configuration
    pub fn secure() -> Self {
        let mut config = Self::development();
        
        // Security-focused settings
        config.extensions.tls.enabled = true;
        config.extensions.tls.verify_client = true;
        config.extensions.auth.auth_required = true;
        config.extensions.auth.token_expiration_hours = 1; // Short-lived tokens
        config.extensions.cors.enabled = false; // Disable CORS for security
        config.security.enable_rate_limiting = true;
        
        config
    }

    /// Validate API-specific configuration
    pub fn validate_api_config(&self) -> Result<(), String> {
        // Use base validation
        nestgate_core::unified_config_consolidation::validation::validate_domain_config(self)?;

        // API-specific validations
        if self.extensions.server.port == 0 {
            return Err("Server port must be specified".to_string());
    }

        if self.extensions.tls.enabled && self.extensions.tls.cert_file.is_none() {
            return Err("TLS cert file must be specified when TLS is enabled".to_string());
    }

        if self.extensions.auth.auth_required && self.extensions.auth.token_expiration_hours == 0 {
            return Err("Token expiration must be specified when auth is required".to_string());
    }

    }
    }

/// **UNIFIED API CONFIGURATION USAGE EXAMPLES**
/// 
/// The unified API configuration system eliminates fragmented API handler configs
/// and provides a consistent, comprehensive configuration interface.
/// 
/// # Basic Usage
/// 
/// ```rust
/// use crate::UnifiedApiConfig;
/// 
/// // Create development API configuration
/// let config = UnifiedApiConfig::development();
/// 
/// // Access different API settings
/// let performance_settings = &config.extensions.performance;
/// let health_settings = &config.extensions.health_checks;
/// let metrics_settings = &config.extensions.metrics;
/// let tls_settings = &config.extensions.tls;
/// let cors_settings = &config.extensions.cors;
/// let auth_settings = &config.extensions.auth;
/// 
/// // Access unified base configurations
/// let network_config = &config.network;
/// let security_config = &config.security;
/// let monitoring_config = &config.monitoring;
/// ```
/// 
/// # Specialized API Configurations
/// 
/// ```rust
/// // Production API configuration
/// let prod_config = UnifiedApiConfig::production();
/// assert!(prod_config.extensions.tls.enabled);
/// assert!(prod_config.extensions.auth.auth_required);
/// 
/// // High-performance API configuration  
/// let perf_config = UnifiedApiConfig::high_performance();
/// assert!(perf_config.extensions.performance.enable_profiling);
/// assert!(perf_config.extensions.server.enable_compression);
/// 
/// // Secure API configuration
/// let secure_config = UnifiedApiConfig::secure();
/// assert!(secure_config.extensions.tls.verify_client);
/// assert_eq!(secure_config.extensions.auth.token_expiration_hours, 1);
/// ```
/// 
/// # Migration from Legacy Configs
/// 
/// ```rust
/// // OLD: Fragmented configs (now deprecated)
/// // use crate::config::monitoring::{PerformanceConfig, HealthCheckConfig, MetricsConfig};
/// // use crate::config::security::{TlsConfig, CorsConfig, PrimalAuthConfig};
/// // use crate::handlers::zfs::universal_zfs::config::{FailSafeConfig, ObservabilityConfig};
/// 
/// // NEW: Unified configuration system
/// use crate::UnifiedApiConfig;
/// 
/// // All API settings use the same unified config
/// let config = UnifiedApiConfig::production();
/// 
/// // Access any setting through extensions or base config
/// let performance_monitoring = config.extensions.performance.enable_cpu_monitoring;
/// let health_checks_enabled = config.extensions.health_checks.enabled;
/// let tls_enabled = config.extensions.tls.enabled;
/// let cors_origins = &config.extensions.cors.allowed_origins;
/// let auth_required = config.extensions.auth.auth_required;
/// ```
/// 
/// # Custom Configuration
/// 
/// ```rust
/// let mut config = UnifiedApiConfig::development();
/// 
/// // Customize for specific API needs
/// config.extensions.performance.sample_interval_seconds = 30;
/// config.extensions.health_checks.check_interval = Duration::from_secs(60);
/// config.extensions.metrics.prometheus_enabled = true;
/// config.extensions.tls.enabled = true;
/// config.extensions.cors.allowed_origins = vec!["https://example.com".to_string()];
/// 
/// // Set API-specific environment variables
/// config.environment.insert("API_VERSION".to_string(), 
///                          serde_json::Value::String("v1".to_string()));
/// 
/// // Enable/disable API features
/// config.features.insert("advanced_metrics".to_string(), true);
/// config.features.insert("experimental_endpoints".to_string(), false);
/// ```
/// 
/// # Benefits of Unified API Configuration
/// 
/// - **Consistency**: All API handlers use the same configuration patterns
/// - **Comprehensive**: Covers performance, health, metrics, TLS, CORS, auth, and server settings
/// - **Discoverable**: All settings documented and accessible from one place
/// - **Type Safe**: Compile-time validation of all configuration fields
/// - **Environment Ready**: Proper defaults and environment variable support
/// - **Extensible**: Easy to add new API-specific settings without breaking changes

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_config_creation() {
        let config = UnifiedApiConfig::development();
        assert_eq!(config.service.name, "nestgate-api");
        assert!(!config.extensions.auth.auth_required);
        assert!(!config.extensions.tls.enabled);
    }

    #[test]
    fn test_production_config() {
        let config = UnifiedApiConfig::production();
        assert!(config.extensions.auth.auth_required);
        assert!(config.extensions.tls.enabled);
        assert!(config.extensions.performance.enable_profiling);
    }

    #[test]
    fn test_config_validation() {
        let mut config = UnifiedApiConfig::development();
        config.extensions.server.port = 8080;
        
        let result = config.validate_api_config();
        assert!(result.is_ok());
    }

    #[test]
    fn test_migration_from_legacy() {
        // REMOVED: Import from deleted monitoring config module
        
        let legacy_perf = PerformanceConfig::default();
        let unified = migration::migrate_api_configs(
            Some(legacy_perf),
            None, None, None, None, None, None
        );
        
        assert_eq!(unified.service.name, "nestgate-api");
        assert!(unified.extensions.performance.enable_cpu_monitoring);
    }
} 