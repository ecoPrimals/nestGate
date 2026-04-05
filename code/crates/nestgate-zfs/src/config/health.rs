// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Configuration for ZFS health monitoring, alerting, and failure thresholds.

use nestgate_core::constants::LOCALHOST_NAME;
use serde::{Deserialize, Serialize};

/// Health monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
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
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for `HealthMonitoring`
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
    /// Returns the default instance
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
                // ✅ SOVEREIGNTY: Environment-driven alert configuration
                // Alert endpoints should be explicitly configured in production
                // For development, this can be optional

                std::env::var("NESTGATE_ALERT_ENDPOINTS").ok().map_or_else(
                    || {
                        // Development-only fallback - logs warning
                        tracing::warn!(
                            "NESTGATE_ALERT_ENDPOINTS not set. Using development defaults. \
                             Set NESTGATE_ALERT_ENDPOINTS in production."
                        );

                        // Development convenience: local endpoints
                        // In production, this code path should never execute
                        let dev_host = std::env::var("NESTGATE_DEV_HOST")
                            .unwrap_or_else(|_| LOCALHOST_NAME.to_string());
                        let dev_port = std::env::var("NESTGATE_DEV_PORT")
                            .ok()
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(8080);

                        vec![
                            format!("email:dev@{}", dev_host),
                            format!("webhook:http://{}:{}/alerts", dev_host, dev_port),
                        ]
                    },
                    |endpoints_str| {
                        // Parse comma-separated endpoints
                        endpoints_str
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect::<Vec<_>>()
                    },
                )
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
/// Type alias for Healthmonitoringconfigcanonical
pub type HealthMonitoringConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using HealthMonitoringConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[expect(deprecated)]
    #[test]
    fn health_monitoring_config_default_roundtrip_json() {
        let c = HealthMonitoringConfig::default();
        let json = serde_json::to_string(&c).expect("serialize");
        let back: HealthMonitoringConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.enabled, c.enabled);
        assert_eq!(back.failure_threshold, c.failure_threshold);
        assert_eq!(back.alert_endpoints, c.alert_endpoints);
    }

    #[expect(deprecated)]
    #[test]
    fn health_monitoring_config_production_sets_alerting() {
        let p = HealthMonitoringConfig::production();
        assert!(p.enabled);
        assert!(p.alerting_enabled);
        assert!(!p.alert_endpoints.is_empty());
    }
}
