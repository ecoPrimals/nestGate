// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **CANONICAL MONITORING CONFIGURATION TYPES**
//!
//! Metrics, logging, tracing, health checks, alerting,
//! dashboard, and performance monitoring configuration types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for MonitoringMetrics
pub struct MonitoringMetricsConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Collection Interval
    pub collection_interval: Duration,
    /// Retention Days
    pub retention_days: usize,
    /// Export Enabled
    pub export_enabled: bool,
    /// Export Endpoints
    pub export_endpoints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for MonitoringLogging
pub struct MonitoringLoggingConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Level
    pub level: String,
    /// Format
    pub format: String,
    /// Output Destinations
    pub output_destinations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Logrotationpolicy
pub struct LogRotationPolicy {
    /// Max Size in megabytes
    pub max_size_mb: usize,
    /// Max Files
    pub max_files: usize,
    /// Compress
    pub compress: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for MonitoringTracing
pub struct MonitoringTracingConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Sampling Rate
    pub sampling_rate: f64,
    /// Trace Endpoint
    pub trace_endpoint: Option<String>,
    /// Service name
    pub service_name: String,
    /// Tags
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for MonitoringHealthCheck
pub struct MonitoringHealthCheckConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Endpoint
    pub endpoint: String,
    /// Interval
    pub interval: Duration,
    /// Timeout
    pub timeout: Duration,
    /// Failure Threshold
    pub failure_threshold: usize,
    /// Success Threshold
    pub success_threshold: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for MonitoringAlerting
pub struct MonitoringAlertingConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Alert Channels
    pub alert_channels: Vec<AlertChannel>,
    /// Alert Rules
    pub alert_rules: Vec<AlertRule>,
    /// Escalation Policies
    pub escalation_policies: Vec<EscalationPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Alertchannel
pub struct AlertChannel {
    /// Name
    pub name: String,
    /// Channel Type
    pub channel_type: String,
    /// Configuration for uration
    pub configuration: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Alertrule
pub struct AlertRule {
    /// Name
    pub name: String,
    /// Condition
    pub condition: String,
    /// Threshold
    pub threshold: f64,
    /// Duration
    pub duration: Duration,
    /// Severity
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Escalationpolicy
pub struct EscalationPolicy {
    /// Name
    pub name: String,
    /// Levels
    pub levels: Vec<EscalationLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Escalationlevel
pub struct EscalationLevel {
    /// Delay
    pub delay: Duration,
    /// Channels
    pub channels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for MonitoringDashboard
pub struct MonitoringDashboardConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Dashboard Endpoint
    pub dashboard_endpoint: Option<String>,
    /// Custom Dashboards
    pub custom_dashboards: Vec<DashboardConfig>,
    /// Auto Refresh Interval
    pub auto_refresh_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Dashboard
pub struct DashboardConfig {
    /// Name
    pub name: String,
    /// Panels
    pub panels: Vec<PanelConfig>,
    /// Refresh Interval
    pub refresh_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Panel
pub struct PanelConfig {
    /// Title
    pub title: String,
    /// Panel Type
    pub panel_type: String,
    /// Query
    pub query: String,
    /// Visualization
    pub visualization: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for MonitoringPerformance
pub struct MonitoringPerformanceConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Profiling Enabled
    pub profiling_enabled: bool,
    /// Benchmark Enabled
    pub benchmark_enabled: bool,
    /// Performance Targets
    pub performance_targets: HashMap<String, f64>,
}

/// **CANONICAL CONFIGURATION TYPE REGISTRY**
///
/// This struct provides access to all canonical configuration types
/// and includes utilities for migration from legacy configurations.
pub struct CanonicalConfigTypeRegistry;
impl CanonicalConfigTypeRegistry {
    /// Get the canonical storage configuration type
    #[must_use]
    pub fn storage_config() -> &'static str {
        "CanonicalStorageConfig"
    }

    /// Get the canonical network configuration type
    #[must_use]
    pub fn network_config() -> &'static str {
        "CanonicalNetworkConfig"
    }

    /// Get the canonical security configuration type
    #[must_use]
    pub fn security_config() -> &'static str {
        "CanonicalSecurityConfig"
    }

    /// Get the canonical monitoring configuration type
    #[must_use]
    pub fn monitoring_config() -> &'static str {
        "CanonicalMonitoringConfig"
    }

    /// List all legacy configuration types that should be migrated
    #[must_use]
    pub fn legacy_types() -> Vec<&'static str> {
        vec![
            "StorageConfig", // From various modules
            "StorageResourceConfig",
            "StorageTierConfig",
            "NetworkConfig",
            "SecurityConfig",
            "MonitoringConfig",
            "FsMonitorStorageSettings",
            "McpStorageConfig",
            // Add more as we find them
        ]
    }

    /// Check if a type name is a legacy configuration type
    #[must_use]
    pub fn is_legacy_type(type_name: &str) -> bool {
        Self::legacy_types().contains(&type_name)
    }
}

// ==================== CANONICAL TYPE ALIASES ====================
// Backward-compatible aliases to `CanonicalNetworkConfig` while migrating from deprecated structs.
#[allow(deprecated, missing_docs)]
mod deprecated_canonical_aliases {
    pub type NetworkInterfaceConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type NetworkProtocolsConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type NetworkConnectionConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type NetworkSecurityConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type NetworkPerformanceConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type NetworkBufferConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type NetworkLoadBalancingConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type NetworkServiceDiscoveryConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type NetworkMonitoringConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type StorageConnectionConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type StoragePerformanceConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type StorageSecurityConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type StorageReplicationConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type StorageTierConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type StorageProtocolsConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type StorageMonitoringConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type CanonicalSecurityConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type SecurityAuthenticationConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type SecurityAuthorizationConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type SecurityEncryptionConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type SecurityTlsConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type SecurityAuditConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type SecurityThreatDetectionConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
    pub type SecurityComplianceConfigCanonical =
        crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
}
pub use deprecated_canonical_aliases::*;
