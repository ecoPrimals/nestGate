//
// Configuration for ZFS health monitoring, alerting, and failure thresholds.

use serde::{Deserialize, Serialize};

/// Health monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::network::config::HealthMonitoringConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::HealthMonitoringConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct HealthMonitoringConfig {
    /// Enable health monitoring
    pub enabled: bool,
    /// Check interval in seconds
    pub check_interval_seconds: u64,
    /// Failure threshold before marking unhealthy
    pub failure_threshold: u32,
    /// Recovery threshold before marking healthy
    pub recovery_threshold: u32,
    /// Enable alerting
    pub alerting_enabled: bool,
    /// Alert endpoints
    pub alert_endpoints: Vec<String>,
}
impl Default for HealthMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval_seconds: 60,
            failure_threshold: 3,
            recovery_threshold: 2,
            alerting_enabled: false,
            alert_endpoints: vec![],
        }
    }
}

impl HealthMonitoringConfig {
    /// Create production-optimized health monitoring configuration
    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            check_interval_seconds: 60,
            failure_threshold: 3,
            recovery_threshold: 1,
            alerting_enabled: true,
            alert_endpoints: {
                use nestgate_core::constants::hardcoding::{addresses, ports};
                vec![
                    format!("email:admin@{}", addresses::LOCALHOST_NAME),
                    format!(
                        "webhook:http://{}:{}/alerts",
                        addresses::LOCALHOST_NAME,
                        ports::HTTP_DEFAULT
                    ),
                ]
            },
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
pub type HealthMonitoringConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using HealthMonitoringConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

