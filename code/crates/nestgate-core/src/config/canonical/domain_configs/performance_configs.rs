/// Performance Configuration Domain
///
/// Replaces: PerformanceConfig, MetricsConfig, MonitoringConfig, AlertsConfig,
/// BenchmarkConfig, and 8+ other performance config structures
use super::CanonicalDomainConfig;
use crate::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **CANONICAL PERFORMANCE CONFIGURATION**
/// Replaces: PerformanceConfig, MetricsConfig, MonitoringConfig, AlertsConfig,
/// BenchmarkConfig, and 8+ other performance config structures
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalPerformanceConfig {
    /// Metrics collection settings
    pub metrics: PerformanceMetrics,
    /// Monitoring settings
    pub monitoring: PerformanceMonitoring,
    /// Alerting settings
    pub alerts: PerformanceAlerts,
    /// Benchmarking settings
    pub benchmarks: PerformanceBenchmarks,
    /// Optimization settings
    pub optimization: PerformanceOptimization,
    /// Resource limits
    pub limits: PerformanceLimits,
    /// Environment-specific overrides
    pub environment_overrides: HashMap<String, serde_json::Value>,
}

impl CanonicalDomainConfig for CanonicalPerformanceConfig {
    fn domain() -> &'static str {
        "performance"
    }

    fn validate(&self) -> Result<()> {
        if self.metrics.collection_interval.as_secs() == 0 {
            return Err(NestGateError::config_error(
                "metrics.collection_interval",
                "must be greater than 0",
            ));
        }
        Ok(())
    }

    fn merge(mut self, other: Self) -> Self {
        self.environment_overrides
            .extend(other.environment_overrides);
        self
    }

    fn from_environment() -> Result<Self> {
        Ok(Self::default())
    }

    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "metrics": {"type": "object", "description": "Metrics collection settings"},
                "monitoring": {"type": "object", "description": "Monitoring settings"}
            }
        })
    }
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub enabled: bool,
    pub collection_interval: Duration,
    pub retention_period: Duration,
    pub export_endpoints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMonitoring {
    pub health_checks_enabled: bool,
    pub health_check_interval: Duration,
    pub dashboard_enabled: bool,
    pub real_time_monitoring: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlerts {
    pub enabled: bool,
    pub cpu_threshold: f64,
    pub memory_threshold: f64,
    pub disk_threshold: f64,
    pub network_threshold: f64,
    pub notification_channels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBenchmarks {
    pub enabled: bool,
    pub benchmark_suites: Vec<String>,
    pub baseline_comparison: bool,
    pub performance_regression_detection: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceOptimization {
    pub auto_scaling_enabled: bool,
    pub caching_enabled: bool,
    pub compression_enabled: bool,
    pub connection_pooling: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceLimits {
    pub max_cpu_usage: f64,
    pub max_memory_usage: u64,
    pub max_disk_usage: u64,
    pub max_network_bandwidth: u64,
    pub max_concurrent_requests: u32,
}

// Default implementations
impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(60),
            retention_period: Duration::from_secs(86400 * 7), // 7 days
            export_endpoints: Vec::new(),
        }
    }
}

impl Default for PerformanceMonitoring {
    fn default() -> Self {
        Self {
            health_checks_enabled: true,
            health_check_interval: Duration::from_secs(30),
            dashboard_enabled: true,
            real_time_monitoring: false,
        }
    }
}

impl Default for PerformanceAlerts {
    fn default() -> Self {
        Self {
            enabled: true,
            cpu_threshold: 80.0,
            memory_threshold: 85.0,
            disk_threshold: 90.0,
            network_threshold: 75.0,
            notification_channels: Vec::new(),
        }
    }
}

impl Default for PerformanceBenchmarks {
    fn default() -> Self {
        Self {
            enabled: false,
            benchmark_suites: Vec::new(),
            baseline_comparison: true,
            performance_regression_detection: true,
        }
    }
}

impl Default for PerformanceOptimization {
    fn default() -> Self {
        Self {
            auto_scaling_enabled: false,
            caching_enabled: true,
            compression_enabled: true,
            connection_pooling: true,
        }
    }
}

impl Default for PerformanceLimits {
    fn default() -> Self {
        Self {
            max_cpu_usage: 90.0,
            max_memory_usage: 8_589_934_592,      // 8GB
            max_disk_usage: 107_374_182_400,      // 100GB
            max_network_bandwidth: 1_073_741_824, // 1GB/s
            max_concurrent_requests: 10000,
        }
    }
}
