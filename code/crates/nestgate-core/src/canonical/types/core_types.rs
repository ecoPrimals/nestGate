// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::collections::HashMap;
//
// This module provides the fundamental type definitions used throughout
// the canonical modernization system.

use std::sync::{Arc, RwLock};

// ==================== SECTION ====================

/// **Canonical Provider Registry**
///
/// Registry for tracking capability providers in the universal adapter system.
/// This provides zero-cost type aliasing for complex provider storage.
pub type ProviderRegistry = Arc<RwLock<HashMap<String, String>>>;
/// **Canonical Capability Index Map**
///
/// Index mapping capabilities to their providers for fast lookup.
/// Enables O(1) capability discovery in the universal adapter.
pub type CapabilityIndexMap = Arc<RwLock<HashMap<String, Vec<String>>>>;
/// **Canonical Health Monitor Registry**
///
/// Registry for tracking health status of various system components.
/// Provides centralized health monitoring across all capabilities.
pub type HealthMonitorRegistry = Arc<RwLock<HashMap<String, String>>>;
/// **Canonical Service Registry**
///
/// Registry for tracking discovered services and their metadata.
pub type ServiceRegistry = Arc<RwLock<HashMap<String, ServiceInfo>>>;
/// **Canonical Configuration Registry**
///
/// Registry for dynamic configuration management across the system.
pub type ConfigurationRegistry = Arc<RwLock<HashMap<String, serde_json::Value>>>;
/// **Canonical Event Registry**
///
/// Registry for tracking system events and their handlers.
pub type EventRegistry = Arc<RwLock<HashMap<String, Vec<EventHandler>>>>;
/// **Canonical Timestamp Changes Map**
///
/// Map for tracking timestamp changes in storage operations.
pub type TimestampChangesMap = HashMap<String, std::time::SystemTime>;
/// **Canonical Attribute Changes Map**
///
/// Map for tracking attribute changes in storage operations.
pub type AttributeChangesMap = HashMap<String, serde_json::Value>;
/// **Canonical Memory Pool**
///
/// Generic memory pool type for efficient memory management.
pub type MemoryPool<T> = Arc<RwLock<Vec<T>>>;
/// **Canonical Pool Statistics Tuple**
///
/// Tuple type for memory pool statistics (used, available, total).
pub type PoolStatisticsTuple = (usize, usize, usize);
/// **Canonical Alert Registry**
///
/// Registry for tracking alert rules and configurations.
pub type AlertRegistry = Arc<RwLock<HashMap<String, AlertRule>>>;
/// **Canonical Alert Map**
///
/// Map for tracking active alerts and their states.
pub type AlertMap = Arc<RwLock<HashMap<String, Alert>>>;
/// **Canonical Alert Channel Map**
///
/// Map for tracking alert notification channels.
pub type AlertChannelMap = Arc<RwLock<HashMap<String, AlertChannel>>>;
/// **Canonical Suppression Rule Map**
///
/// Map for tracking alert suppression rules.
pub type SuppressionRuleMap = Arc<RwLock<HashMap<String, SuppressionRule>>>;
/// **Canonical Health Check Map**
///
/// Map for tracking health check configurations.
pub type HealthCheckMap = Arc<RwLock<HashMap<String, HealthCheck>>>;
// ==================== SECTION ====================

/// **Service Information Structure**
///
/// Canonical structure for storing service metadata in registries.
#[derive(Debug, Clone)]
/// Serviceinfo
pub struct ServiceInfo {
    /// Service identifier
    pub service_id: String,
    /// Service Type
    pub service_type: String,
    /// Capabilities
    pub capabilities: Vec<String>,
    /// Endpoint
    pub endpoint: String,
    /// Health Status
    pub health_status: String,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
/// **Event Handler Structure**
///
/// Canonical structure for event handling in the system.
#[derive(Debug, Clone)]
/// Handler for Event requests
pub struct EventHandler {
    /// Handler identifier
    pub handler_id: String,
    /// Event Type
    pub event_type: String,
    /// Handler Function
    pub handler_function: String, // Function name or identifier
}
/// **Alert Rule Structure**
///
/// Canonical structure for defining alert rules.
#[derive(Debug, Clone)]
/// Alertrule
pub struct AlertRule {
    /// Rule identifier
    pub rule_id: String,
    /// Name
    pub name: String,
    /// Condition
    pub condition: String,
    /// Severity
    pub severity: String,
    /// Whether this feature is enabled
    pub enabled: bool,
}
/// **Alert Structure**
///
/// Canonical structure for active alerts.
#[derive(Debug, Clone)]
/// Alert
pub struct Alert {
    /// Alert identifier
    pub alert_id: String,
    /// Rule identifier
    pub rule_id: String,
    /// Message
    pub message: String,
    /// Severity
    pub severity: String,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
    /// Resolved
    pub resolved: bool,
}
/// **Alert Channel Structure**
///
/// Canonical structure for alert notification channels.
#[derive(Debug, Clone)]
/// Alertchannel
pub struct AlertChannel {
    /// Channel identifier
    pub channel_id: String,
    /// Channel Type
    pub channel_type: String,
    /// Endpoint
    pub endpoint: String,
    /// Whether this feature is enabled
    pub enabled: bool,
}
/// **Suppression Rule Structure**
///
/// Canonical structure for alert suppression rules.
#[derive(Debug, Clone)]
/// Suppressionrule
pub struct SuppressionRule {
    /// Rule identifier
    pub rule_id: String,
    /// Name
    pub name: String,
    /// Pattern
    pub pattern: String,
    /// Duration Seconds
    pub duration_seconds: u64,
    /// Whether this feature is enabled
    pub enabled: bool,
}
/// **Health Check Structure**
///
/// Canonical structure for health check configurations.
#[derive(Debug, Clone)]
/// Healthcheck
pub struct HealthCheck {
    /// Check identifier
    pub check_id: String,
    /// Name
    pub name: String,
    /// Component
    pub component: String,
    /// Interval Seconds
    pub interval_seconds: u64,
    /// Timeout Seconds
    pub timeout_seconds: u64,
    /// Whether this feature is enabled
    pub enabled: bool,
}
// ==================== SECTION ====================

impl Default for ServiceInfo {
    /// Returns the default instance
    fn default() -> Self {
        // Use centralized port configuration instead of direct env::var
        let discovery_config = crate::config::discovery_config::ServiceDiscoveryConfig::default();
        let api_port = crate::constants::port_defaults::get_api_port();
        Self {
            service_id: "unknown".into(),
            service_type: "generic".into(),
            capabilities: Vec::new(),
            endpoint: discovery_config.build_endpoint(api_port),
            health_status: "unknown".into(),
            metadata: HashMap::new(),
        }
    }
}

impl Default for EventHandler {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            handler_id: "default".into(),
            event_type: "generic".into(),
            handler_function: "default_handler".into(),
        }
    }
}

impl Default for AlertRule {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            rule_id: "default".into(),
            name: "Default Rule".into(),
            condition: "always".into(),
            severity: "info".into(),
            enabled: true,
        }
    }
}

impl Default for Alert {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            alert_id: String::from("default"),
            rule_id: String::from("default"),
            message: String::from("Default alert"),
            severity: String::from("info"),
            timestamp: std::time::SystemTime::now(),
            resolved: false,
        }
    }
}

impl Default for AlertChannel {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            channel_id: "default".into(),
            channel_type: "console".into(),
            endpoint: "stdout".into(),
            enabled: true,
        }
    }
}

impl Default for SuppressionRule {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            rule_id: String::from("default"),
            name: String::from("Default Suppression"),
            pattern: String::from(".*"),
            duration_seconds: 300,
            enabled: false,
        }
    }
}

impl Default for HealthCheck {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            check_id: "default".into(),
            name: "Default Health Check".into(),
            component: "system".into(),
            interval_seconds: 30,
            timeout_seconds: 5,
            enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_info_default() {
        let info = ServiceInfo::default();
        assert_eq!(info.service_id, "unknown");
        assert_eq!(info.service_type, "generic");
        assert!(info.capabilities.is_empty());
        assert_eq!(info.health_status, "unknown");
        assert!(info.metadata.is_empty());
    }

    #[test]
    fn test_service_info_custom() {
        let capabilities = vec![String::from("storage"), String::from("zfs")];

        let mut metadata = HashMap::new();
        metadata.insert(String::from("region"), String::from("us-west"));

        let info = ServiceInfo {
            service_id: String::from("svc-123"),
            service_type: String::from("storage"),
            capabilities,
            endpoint: String::from("http://localhost:9000"),
            health_status: String::from("healthy"),
            metadata,
        };

        assert_eq!(info.service_id, "svc-123");
        assert_eq!(info.service_type, "storage");
        assert_eq!(info.capabilities.len(), 2);
        assert_eq!(info.endpoint, "http://localhost:9000");
        assert_eq!(info.health_status, "healthy");
        assert_eq!(info.metadata.get("region").unwrap(), "us-west");
    }

    #[test]
    fn test_event_handler_default() {
        let handler = EventHandler::default();
        assert_eq!(handler.handler_id, "default");
        assert_eq!(handler.event_type, "generic");
        assert_eq!(handler.handler_function, "default_handler");
    }

    #[test]
    fn test_event_handler_custom() {
        let handler = EventHandler {
            handler_id: String::from("handler-001"),
            event_type: String::from("storage.created"),
            handler_function: String::from("handle_storage_created"),
        };

        assert_eq!(handler.handler_id, "handler-001");
        assert_eq!(handler.event_type, "storage.created");
        assert_eq!(handler.handler_function, "handle_storage_created");
    }

    #[test]
    fn test_alert_rule_default() {
        let rule = AlertRule::default();
        assert_eq!(rule.rule_id, "default");
        assert_eq!(rule.name, "Default Rule");
        assert_eq!(rule.condition, "always");
        assert_eq!(rule.severity, "info");
        assert!(rule.enabled);
    }

    #[test]
    fn test_alert_rule_custom() {
        let rule = AlertRule {
            rule_id: String::from("rule-001"),
            name: String::from("High CPU Alert"),
            condition: String::from("cpu_usage > 80"),
            severity: String::from("warning"),
            enabled: true,
        };

        assert_eq!(rule.rule_id, "rule-001");
        assert_eq!(rule.name, "High CPU Alert");
        assert_eq!(rule.condition, "cpu_usage > 80");
        assert_eq!(rule.severity, "warning");
        assert!(rule.enabled);
    }

    #[test]
    fn test_alert_default() {
        let alert = Alert::default();
        assert_eq!(alert.alert_id, "default");
        assert_eq!(alert.rule_id, "default");
        assert_eq!(alert.message, "Default alert");
        assert_eq!(alert.severity, "info");
        assert!(!alert.resolved);
    }

    #[test]
    fn test_alert_custom() {
        let alert = Alert {
            alert_id: String::from("alert-001"),
            rule_id: String::from("rule-001"),
            message: String::from("CPU usage exceeded threshold"),
            severity: String::from("critical"),
            timestamp: std::time::SystemTime::now(),
            resolved: false,
        };

        assert_eq!(alert.alert_id, "alert-001");
        assert_eq!(alert.rule_id, "rule-001");
        assert_eq!(alert.message, "CPU usage exceeded threshold");
        assert_eq!(alert.severity, "critical");
        assert!(!alert.resolved);
    }

    #[test]
    fn test_alert_resolution() {
        let mut alert = Alert::default();
        assert!(!alert.resolved);

        alert.resolved = true;
        assert!(alert.resolved);
    }

    #[test]
    fn test_alert_channel_default() {
        let channel = AlertChannel::default();
        assert_eq!(channel.channel_id, "default");
        assert_eq!(channel.channel_type, "console");
        assert_eq!(channel.endpoint, "stdout");
        assert!(channel.enabled);
    }

    #[test]
    fn test_alert_channel_custom() {
        let Some(endpoint) = std::env::var("SLACK_WEBHOOK_URL")
            .ok()
            .filter(|s| !s.is_empty())
        else {
            return;
        };

        let channel = AlertChannel {
            channel_id: String::from("slack-001"),
            channel_type: String::from("slack"),
            endpoint,
            enabled: true,
        };

        assert_eq!(channel.channel_id, "slack-001");
        assert_eq!(channel.channel_type, "slack");
        assert!(channel.endpoint.starts_with("https://"));
        assert!(channel.enabled);
    }

    #[test]
    fn test_suppression_rule_default() {
        let rule = SuppressionRule::default();
        assert_eq!(rule.rule_id, "default");
        assert_eq!(rule.name, "Default Suppression");
        assert_eq!(rule.pattern, ".*");
        assert_eq!(rule.duration_seconds, 300);
        assert!(!rule.enabled);
    }

    #[test]
    fn test_suppression_rule_custom() {
        let rule = SuppressionRule {
            rule_id: String::from("suppress-001"),
            name: String::from("Maintenance Window"),
            pattern: String::from("maintenance.*"),
            duration_seconds: 3600,
            enabled: true,
        };

        assert_eq!(rule.rule_id, "suppress-001");
        assert_eq!(rule.name, "Maintenance Window");
        assert_eq!(rule.pattern, "maintenance.*");
        assert_eq!(rule.duration_seconds, 3600);
        assert!(rule.enabled);
    }

    #[test]
    fn test_health_check_default() {
        let check = HealthCheck::default();
        assert_eq!(check.check_id, "default");
        assert_eq!(check.name, "Default Health Check");
        assert_eq!(check.component, "system");
        assert_eq!(check.interval_seconds, 30);
        assert_eq!(check.timeout_seconds, 5);
        assert!(check.enabled);
    }

    #[test]
    fn test_health_check_custom() {
        let check = HealthCheck {
            check_id: String::from("check-001"),
            name: String::from("Database Health"),
            component: String::from("database"),
            interval_seconds: 60,
            timeout_seconds: 10,
            enabled: true,
        };

        assert_eq!(check.check_id, "check-001");
        assert_eq!(check.name, "Database Health");
        assert_eq!(check.component, "database");
        assert_eq!(check.interval_seconds, 60);
        assert_eq!(check.timeout_seconds, 10);
        assert!(check.enabled);
    }

    #[test]
    fn test_provider_registry_creation() {
        let registry: ProviderRegistry = Arc::new(RwLock::new(HashMap::new()));
        let mut map = registry.write().expect("Failed to acquire write lock");
        map.insert(String::from("storage"), String::from("zfs-provider"));

        assert_eq!(map.get("storage").unwrap(), "zfs-provider");
    }

    #[test]
    fn test_capability_index_map_creation() {
        let index: CapabilityIndexMap = Arc::new(RwLock::new(HashMap::new()));
        let mut map = index.write().expect("Failed to acquire write lock");

        let providers = vec![String::from("provider1"), String::from("provider2")];

        map.insert(String::from("compute"), providers);

        let compute_providers = map.get("compute").unwrap();
        assert_eq!(compute_providers.len(), 2);
        assert_eq!(compute_providers[0], "provider1");
    }

    #[test]
    fn test_service_registry_creation() {
        let registry: ServiceRegistry = Arc::new(RwLock::new(HashMap::new()));
        let mut map = registry.write().expect("Failed to acquire write lock");

        let service_info = ServiceInfo {
            service_id: String::from("svc-001"),
            service_type: String::from("api"),
            capabilities: vec![String::from("rest")],
            endpoint: String::from("http://localhost:18080"),
            health_status: String::from("healthy"),
            metadata: HashMap::new(),
        };

        map.insert(String::from("api-service"), service_info);

        assert!(map.contains_key("api-service"));
        assert_eq!(map.get("api-service").unwrap().service_id, "svc-001");
    }

    #[test]
    fn test_event_handler_clone() {
        let handler = EventHandler {
            handler_id: String::from("handler-001"),
            event_type: String::from("test.event"),
            handler_function: String::from("handle_test"),
        };

        let cloned = handler.clone();
        assert_eq!(handler.handler_id, cloned.handler_id);
        assert_eq!(handler.event_type, cloned.event_type);
        assert_eq!(handler.handler_function, cloned.handler_function);
    }

    #[test]
    fn test_pool_statistics_tuple() {
        let stats: PoolStatisticsTuple = (100, 400, 500);
        let (used, available, total) = stats;

        assert_eq!(used, 100);
        assert_eq!(available, 400);
        assert_eq!(total, 500);
        assert_eq!(used + available, total);
    }

    #[test]
    fn test_memory_pool_creation() {
        let pool: MemoryPool<u64> = Arc::new(RwLock::new(Vec::new()));
        let mut vec = pool.write().expect("Failed to acquire write lock");

        vec.push(100);
        vec.push(200);
        vec.push(300);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], 100);
        assert_eq!(vec[1], 200);
        assert_eq!(vec[2], 300);
    }
}
