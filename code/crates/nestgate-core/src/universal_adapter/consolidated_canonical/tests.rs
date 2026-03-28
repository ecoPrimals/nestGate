// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Tests for the consolidated canonical adapter

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_adapter_config_defaults() {
        #[allow(deprecated)]
        let config = CanonicalAdapterConfig::default();
        assert_eq!(config.service_name, "nestgate");
        assert!(config.discovery.auto_discovery);
        assert_eq!(config.discovery.retry_attempts, 3);
    }

    #[test]
    fn test_discovery_config_defaults() {
        let config = DiscoveryConfig::default();
        assert!(config.auto_discovery);
        assert_eq!(config.retry_attempts, 3);
        assert_eq!(config.discovery_methods.len(), 2);
    }

    #[test]
    fn test_alert_thresholds_defaults() {
        let thresholds = AlertThresholds::default();
        assert_eq!(thresholds.response_time_ms, 1000);
        assert_eq!(thresholds.error_rate_percent, 5.0);
        assert_eq!(thresholds.resource_usage_percent, 80.0);
    }

    #[test]
    fn test_resource_requirements_defaults() {
        let reqs = ResourceRequirements::default();
        assert!(reqs.cpu_cores.is_none());
        assert!(reqs.memory_mb.is_none());
        assert!(reqs.storage_gb.is_none());
    }

    #[test]
    fn test_adapter_health_defaults() {
        let health = AdapterHealthStatus::default();
        assert!(health.healthy);
        assert_eq!(health.successful_operations, 0);
        assert_eq!(health.failed_operations, 0);
    }

    #[test]
    fn test_adapter_stats_defaults() {
        let stats = AdapterStats::default();
        assert_eq!(stats.active_connections, 0);
        assert_eq!(stats.total_requests, 0);
        assert_eq!(stats.successful_requests, 0);
        assert_eq!(stats.failed_requests, 0);
    }
}
