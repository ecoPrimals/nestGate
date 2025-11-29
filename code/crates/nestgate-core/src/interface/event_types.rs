/// Event Types Module
/// Event structures for system-wide event handling
/// **PROBLEM SOLVED**: Standardized event communication patterns
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
/// Unified event structure for system-wide event handling
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Unifiedevent
pub struct UnifiedEvent {
    /// Unique event identifier
    pub event_id: String,
    /// Event type/category
    pub event_type: String,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Source service/component
    pub source: String,
    /// Event data payload
    pub data: serde_json::Value,
    /// Event metadata
    pub metadata: HashMap<String, String>,
    /// Event priority level
    pub priority: EventPriority,
    /// Event correlation ID for tracking related events
    pub correlation_id: Option<String>,
}
/// Event priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Eventpriority
pub enum EventPriority {
    /// Low priority event
    Low,
    /// Normal priority event
    Normal,
    /// High priority event
    High,
    /// Critical event requiring immediate attention
    Critical,
}
/// Event subscription configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Eventsubscription
pub struct EventSubscription {
    /// Unique subscription identifier
    pub subscription_id: String,
    /// Event types to subscribe to
    pub event_types: Vec<String>,
    /// Subscriber identifier
    pub subscriber: String,
    /// Subscription filter criteria
    pub filters: HashMap<String, serde_json::Value>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
    /// Whether subscription is active
    pub is_active: bool,
}
/// Event handler configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Handler for Event requests
pub struct EventHandler {
    /// Handler identifier
    pub handler_id: String,
    /// Handler name
    pub name: String,
    /// Event types this handler processes
    pub handled_events: Vec<String>,
    /// Handler endpoint or callback
    pub endpoint: String,
    /// Handler configuration
    pub config: HashMap<String, serde_json::Value>,
    /// Whether handler is enabled
    pub enabled: bool,
}
impl Default for UnifiedEvent {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            event_id: Uuid::new_v4().to_string(),
            event_type: "unknown".to_string(),
            timestamp: Utc::now(),
            source: "system".to_string(),
            data: serde_json::Value::Null,
            metadata: HashMap::new(),
            priority: EventPriority::Normal,
            correlation_id: None,
        }
    }
}

impl Default for EventSubscription {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            subscription_id: Uuid::new_v4().to_string(),
            event_types: Vec::new(),
            subscriber: "unknown".to_string(),
            filters: HashMap::new(),
            created_at: Utc::now(),
            last_activity: Utc::now(),
            is_active: true,
        }
    }
}

impl Default for EventHandler {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            handler_id: Uuid::new_v4().to_string(),
            name: "Default Handler".to_string(),
            handled_events: Vec::new(),
            endpoint: "http://localhost:8080/events".to_string(),
            config: HashMap::new(),
            enabled: true,
        }
    }
}

impl UnifiedEvent {
    /// Create a new event
    pub fn new(event_type: &str, source: &str) -> Self {
        Self {
            event_type: event_type.to_string(),
            source: source.to_string(),
            ..Default::default()
        }
    }

    /// Create event with data
    #[must_use]
    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = data;
        self
    }

    /// Set event priority
    #[must_use]
    pub fn with_priority(mut self, priority: EventPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Add metadata
    #[must_use]
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    /// Set correlation ID for event tracing
    #[must_use]
    pub fn with_correlation_id(mut self, correlation_id: &str) -> Self {
        self.correlation_id = Some(correlation_id.to_string());
        self
    }

    /// Create a critical event
    pub fn critical(event_type: &str, source: &str, data: serde_json::Value) -> Self {
        Self::new(event_type, source)
            .with_data(data)
            .with_priority(EventPriority::Critical)
    }

    /// Create a high priority event
    pub fn high_priority(event_type: &str, source: &str, data: serde_json::Value) -> Self {
        Self::new(event_type, source)
            .with_data(data)
            .with_priority(EventPriority::High)
    }

    /// Check if event matches subscription criteria
    pub fn matches_subscription(&self, subscription: &EventSubscription) -> bool {
        // Check if event type matches
        if !subscription.event_types.contains(&self.event_type) {
            return false;
        }

        // Check filters
        for (filter_key, filtervalue) in &subscription.filters {
            match filter_key.as_str() {
                "source" => {
                    if let Some(expected_source) = filtervalue.as_str() {
                        if self.source != expected_source {
                            return false;
                        }
                    }
                }
                "priority" => {
                    if let Some(expected_priority) = filtervalue.as_str() {
                        let priority_str = match self.priority {
                            EventPriority::Low => "low",
                            EventPriority::Normal => "normal",
                            EventPriority::High => "high",
                            EventPriority::Critical => "critical",
                        };
                        if priority_str != expected_priority {
                            return false;
                        }
                    }
                }
                _ => {
                    // Check metadata filters
                    if let Some(metadatavalue) = self.metadata.get(filter_key) {
                        if let Some(expectedvalue) = filtervalue.as_str() {
                            if metadatavalue != expectedvalue {
                                return false;
                            }
                        }
                    }
                }
            }
        }

        true
    }
}
