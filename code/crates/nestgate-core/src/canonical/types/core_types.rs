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
/// **Canonical Health Check Function**
///
/// Type alias for health check functions.
pub type HealthCheckFunction = Arc<dyn Fn() -> bool + Send + Sync>;
// ==================== SECTION ====================

/// **Service Information Structure**
///
/// Canonical structure for storing service metadata in registries.
#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub service_id: String,
    pub service_type: String,
    pub capabilities: Vec<String>,
    pub endpoint: String,
    pub health_status: String,
    pub metadata: HashMap<String, String>,
}
/// **Event Handler Structure**
///
/// Canonical structure for event handling in the system.
#[derive(Debug, Clone)]
pub struct EventHandler {
    pub handler_id: String,
    pub event_type: String,
    pub handler_function: String, // Function name or identifier
}
/// **Alert Rule Structure**
///
/// Canonical structure for defining alert rules.
#[derive(Debug, Clone)]
pub struct AlertRule {
    pub rule_id: String,
    pub name: String,
    pub condition: String,
    pub severity: String,
    pub enabled: bool,
}
/// **Alert Structure**
///
/// Canonical structure for active alerts.
#[derive(Debug, Clone)]
pub struct Alert {
    pub alert_id: String,
    pub rule_id: String,
    pub message: String,
    pub severity: String,
    pub timestamp: std::time::SystemTime,
    pub resolved: bool,
}
/// **Alert Channel Structure**
///
/// Canonical structure for alert notification channels.
#[derive(Debug, Clone)]
pub struct AlertChannel {
    pub channel_id: String,
    pub channel_type: String,
    pub endpoint: String,
    pub enabled: bool,
}
/// **Suppression Rule Structure**
///
/// Canonical structure for alert suppression rules.
#[derive(Debug, Clone)]
pub struct SuppressionRule {
    pub rule_id: String,
    pub name: String,
    pub pattern: String,
    pub duration_seconds: u64,
    pub enabled: bool,
}
/// **Health Check Structure**
///
/// Canonical structure for health check configurations.
#[derive(Debug, Clone)]
pub struct HealthCheck {
    pub check_id: String,
    pub name: String,
    pub component: String,
    pub interval_seconds: u64,
    pub timeout_seconds: u64,
    pub enabled: bool,
}
// ==================== SECTION ====================

impl Default for ServiceInfo {
    fn default() -> Self {
        // Use centralized port configuration instead of direct env::var
        let discovery_config = crate::config::discovery_config::ServiceDiscoveryConfig::default();
        let api_port = crate::constants::port_defaults::get_api_port();
        Self {
            service_id: "unknown".to_string(),
            service_type: "generic".to_string(),
            capabilities: Vec::new(),
            endpoint: discovery_config.build_endpoint(api_port),
            health_status: "unknown".to_string(),
            metadata: HashMap::new(),
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self {
            handler_id: "default".to_string(),
            event_type: "generic".to_string(),
            handler_function: "default_handler".to_string(),
        }
    }
}

impl Default for AlertRule {
    fn default() -> Self {
        Self {
            rule_id: "default".to_string(),
            name: "Default Rule".to_string(),
            condition: "always".to_string(),
            severity: "info".to_string(),
            enabled: true,
        }
    }
}

impl Default for Alert {
    fn default() -> Self {
        Self {
            alert_id: "default".to_string(),
            rule_id: "default".to_string(),
            message: "Default alert".to_string(),
            severity: "info".to_string(),
            timestamp: std::time::SystemTime::now(),
            resolved: false,
        }
    }
}

impl Default for AlertChannel {
    fn default() -> Self {
        Self {
            channel_id: "default".to_string(),
            channel_type: "console".to_string(),
            endpoint: "stdout".to_string(),
            enabled: true,
        }
    }
}

impl Default for SuppressionRule {
    fn default() -> Self {
        Self {
            rule_id: "default".to_string(),
            name: "Default Suppression".to_string(),
            pattern: ".*".to_string(),
            duration_seconds: 300,
            enabled: false,
        }
    }
}

impl Default for HealthCheck {
    fn default() -> Self {
        Self {
            check_id: "default".to_string(),
            name: "Default Health Check".to_string(),
            component: "system".to_string(),
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
        let capabilities = vec!["storage".to_string(), "zfs".to_string()];

        let mut metadata = HashMap::new();
        metadata.insert("region".to_string(), "us-west".to_string());

        let info = ServiceInfo {
            service_id: "svc-123".to_string(),
            service_type: "storage".to_string(),
            capabilities,
            endpoint: "http://localhost:9000".to_string(),
            health_status: "healthy".to_string(),
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
            handler_id: "handler-001".to_string(),
            event_type: "storage.created".to_string(),
            handler_function: "handle_storage_created".to_string(),
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
            rule_id: "rule-001".to_string(),
            name: "High CPU Alert".to_string(),
            condition: "cpu_usage > 80".to_string(),
            severity: "warning".to_string(),
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
            alert_id: "alert-001".to_string(),
            rule_id: "rule-001".to_string(),
            message: "CPU usage exceeded threshold".to_string(),
            severity: "critical".to_string(),
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
        // Use environment variable or placeholder for sensitive webhook URLs
        let endpoint = std::env::var("SLACK_WEBHOOK_URL").unwrap_or_else(|_| {
            "https://hooks.slack.com/services/TEST123/TEST456/TestWebhookPlaceholder".to_string()
        });

        let channel = AlertChannel {
            channel_id: "slack-001".to_string(),
            channel_type: "slack".to_string(),
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
            rule_id: "suppress-001".to_string(),
            name: "Maintenance Window".to_string(),
            pattern: "maintenance.*".to_string(),
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
            check_id: "check-001".to_string(),
            name: "Database Health".to_string(),
            component: "database".to_string(),
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
        map.insert("storage".to_string(), "zfs-provider".to_string());

        assert_eq!(map.get("storage").unwrap(), "zfs-provider");
    }

    #[test]
    fn test_capability_index_map_creation() {
        let index: CapabilityIndexMap = Arc::new(RwLock::new(HashMap::new()));
        let mut map = index.write().expect("Failed to acquire write lock");

        let providers = vec!["provider1".to_string(), "provider2".to_string()];

        map.insert("compute".to_string(), providers);

        let compute_providers = map.get("compute").unwrap();
        assert_eq!(compute_providers.len(), 2);
        assert_eq!(compute_providers[0], "provider1");
    }

    #[test]
    fn test_service_registry_creation() {
        let registry: ServiceRegistry = Arc::new(RwLock::new(HashMap::new()));
        let mut map = registry.write().expect("Failed to acquire write lock");

        let service_info = ServiceInfo {
            service_id: "svc-001".to_string(),
            service_type: "api".to_string(),
            capabilities: vec!["rest".to_string()],
            endpoint: "http://localhost:8080".to_string(),
            health_status: "healthy".to_string(),
            metadata: HashMap::new(),
        };

        map.insert("api-service".to_string(), service_info);

        assert!(map.contains_key("api-service"));
        assert_eq!(map.get("api-service").unwrap().service_id, "svc-001");
    }

    #[test]
    fn test_event_handler_clone() {
        let handler = EventHandler {
            handler_id: "handler-001".to_string(),
            event_type: "test.event".to_string(),
            handler_function: "handle_test".to_string(),
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
