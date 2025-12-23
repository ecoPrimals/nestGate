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

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Networkinterfaceconfigcanonical
pub type NetworkInterfaceConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using NetworkInterfaceConfig (the deprecated struct) for now.
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
/// Type alias for Networkprotocolsconfigcanonical
pub type NetworkProtocolsConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using NetworkProtocolsConfig (the deprecated struct) for now.
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
/// Type alias for Networkconnectionconfigcanonical
pub type NetworkConnectionConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using NetworkConnectionConfig (the deprecated struct) for now.
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
/// Type alias for Networksecurityconfigcanonical
pub type NetworkSecurityConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using NetworkSecurityConfig (the deprecated struct) for now.
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
/// Type alias for Networkperformanceconfigcanonical
pub type NetworkPerformanceConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using NetworkPerformanceConfig (the deprecated struct) for now.
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
/// Type alias for Networkbufferconfigcanonical
pub type NetworkBufferConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using NetworkBufferConfig (the deprecated struct) for now.
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
/// Type alias for Networkloadbalancingconfigcanonical
pub type NetworkLoadBalancingConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using NetworkLoadBalancingConfig (the deprecated struct) for now.
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
/// Type alias for Networkservicediscoveryconfigcanonical
pub type NetworkServiceDiscoveryConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using NetworkServiceDiscoveryConfig (the deprecated struct) for now.
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
/// Type alias for Networkmonitoringconfigcanonical
pub type NetworkMonitoringConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using NetworkMonitoringConfig (the deprecated struct) for now.
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
/// Type alias for Storageconnectionconfigcanonical
pub type StorageConnectionConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StorageConnectionConfig (the deprecated struct) for now.
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
/// Type alias for Storageperformanceconfigcanonical
pub type StoragePerformanceConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StoragePerformanceConfig (the deprecated struct) for now.
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
/// Type alias for Storagesecurityconfigcanonical
pub type StorageSecurityConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StorageSecurityConfig (the deprecated struct) for now.
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
/// Type alias for Storagereplicationconfigcanonical
pub type StorageReplicationConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StorageReplicationConfig (the deprecated struct) for now.
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
/// Type alias for Storagetierconfigcanonical
pub type StorageTierConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StorageTierConfig (the deprecated struct) for now.
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
/// Type alias for Storageprotocolsconfigcanonical
pub type StorageProtocolsConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StorageProtocolsConfig (the deprecated struct) for now.
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
/// Type alias for Storagemonitoringconfigcanonical
pub type StorageMonitoringConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StorageMonitoringConfig (the deprecated struct) for now.
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
/// Type alias for Canonicalsecurityconfigcanonical
pub type CanonicalSecurityConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using CanonicalSecurityConfig (the deprecated struct) for now.
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
/// Type alias for Securityauthenticationconfigcanonical
pub type SecurityAuthenticationConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using SecurityAuthenticationConfig (the deprecated struct) for now.
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
/// Type alias for Securityauthorizationconfigcanonical
pub type SecurityAuthorizationConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using SecurityAuthorizationConfig (the deprecated struct) for now.
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
/// Type alias for Securityencryptionconfigcanonical
pub type SecurityEncryptionConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using SecurityEncryptionConfig (the deprecated struct) for now.
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
/// Type alias for Securitytlsconfigcanonical
pub type SecurityTlsConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using SecurityTlsConfig (the deprecated struct) for now.
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
/// Type alias for Securityauditconfigcanonical
pub type SecurityAuditConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using SecurityAuditConfig (the deprecated struct) for now.
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
/// Type alias for Securitythreatdetectionconfigcanonical
pub type SecurityThreatDetectionConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using SecurityThreatDetectionConfig (the deprecated struct) for now.
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
/// Type alias for Securitycomplianceconfigcanonical
pub type SecurityComplianceConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using SecurityComplianceConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.
