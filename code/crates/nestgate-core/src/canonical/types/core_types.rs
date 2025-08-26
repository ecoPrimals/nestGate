use std::collections::HashMap;
//
// This module provides the fundamental type definitions used throughout
// the canonical modernization system.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// ==================== CANONICAL TYPE ALIASES ====================

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

// ==================== SUPPORTING TYPES ====================

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

// ==================== DEFAULT IMPLEMENTATIONS ====================

impl Default for ServiceInfo {
    fn default() -> Self {
        Self {
            service_id: "unknown".to_string(),
            service_type: "generic".to_string(),
            capabilities: Vec::new(),
            endpoint: "http://localhost:8080".to_string(),
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