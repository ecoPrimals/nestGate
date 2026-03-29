// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// Message and Event Classification Enums
/// This module contains enums related to messaging, events, operations,
/// and communication patterns.
use serde::{Deserialize, Serialize};
use std::fmt;
// ==================== SECTION ====================

/// **THE** `MessageType` - unified across all modules
/// Replaces 3+ fragmented `MessageType` definitions across MCP, network, and communication modules
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of `UnifiedMessage`
pub enum UnifiedMessageType {
    /// Request message
    Request,
    /// Response message
    Response,
    /// Event notification
    Event,
    /// Status update
    Status,
    /// Error message
    Error,
    /// Heartbeat/keepalive message
    Heartbeat,
    /// Acknowledgment message
    Acknowledgment,
    /// Command message
    Command,
    /// Query message
    Query,
    /// Broadcast message
    Broadcast,
    /// Custom message type
    Custom(String),
}
impl Default for UnifiedMessageType {
    /// Returns the default instance
    fn default() -> Self {
        Self::Request
    }
}

impl fmt::Display for UnifiedMessageType {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request => write!(f, "request"),
            Self::Response => write!(f, "response"),
            Self::Event => write!(f, "event"),
            Self::Status => write!(f, "status"),
            Self::Error => write!(f, "error"),
            Self::Heartbeat => write!(f, "heartbeat"),
            Self::Acknowledgment => write!(f, "acknowledgment"),
            Self::Command => write!(f, "command"),
            Self::Query => write!(f, "query"),
            Self::Broadcast => write!(f, "broadcast"),
            Self::Custom(msg_type) => write!(f, "{msg_type}"),
        }
    }
}

// ==================== SECTION ====================

/// **THE** `EventType` - unified across all modules
/// Replaces 5+ fragmented `EventType` definitions across various event systems
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of `UnifiedEvent`
pub enum UnifiedEventType {
    /// System startup/initialization event
    SystemStart,
    /// System shutdown event
    SystemStop,
    /// Service lifecycle event
    ServiceLifecycle,
    /// Configuration change event
    ConfigurationChange,
    /// Network connectivity event
    NetworkEvent,
    /// Storage operation event
    StorageEvent,
    /// Security event (auth, access control, etc.)
    SecurityEvent,
    /// Performance/metrics event
    PerformanceEvent,
    /// User interaction event
    UserEvent,
    /// API operation event
    ApiEvent,
    /// File system event
    FileSystemEvent,
    /// Database event
    DatabaseEvent,
    /// Error event
    ErrorEvent,
    /// Custom event type
    Custom(String),
}
impl Default for UnifiedEventType {
    /// Returns the default instance
    fn default() -> Self {
        Self::SystemStart
    }
}

impl fmt::Display for UnifiedEventType {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SystemStart => write!(f, "system_start"),
            Self::SystemStop => write!(f, "system_stop"),
            Self::ServiceLifecycle => write!(f, "service_lifecycle"),
            Self::ConfigurationChange => write!(f, "configuration_change"),
            Self::NetworkEvent => write!(f, "network_event"),
            Self::StorageEvent => write!(f, "storage_event"),
            Self::SecurityEvent => write!(f, "security_event"),
            Self::PerformanceEvent => write!(f, "performance_event"),
            Self::UserEvent => write!(f, "user_event"),
            Self::ApiEvent => write!(f, "api_event"),
            Self::FileSystemEvent => write!(f, "filesystem_event"),
            Self::DatabaseEvent => write!(f, "database_event"),
            Self::ErrorEvent => write!(f, "error_event"),
            Self::Custom(event_type) => write!(f, "{event_type}"),
        }
    }
}

// ==================== SECTION ====================

/// **THE** `OperationType` - unified across all modules
/// Replaces `OperationType` definitions across service operations and handlers
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of `UnifiedOperation`
pub enum UnifiedOperationType {
    /// Create operation
    Create,
    /// Read operation
    Read,
    /// Update operation
    Update,
    /// Delete operation
    Delete,
    /// List operation
    List,
    /// Search operation
    Search,
    /// Backup operation
    Backup,
    /// Restore operation
    Restore,
    /// Sync operation
    Sync,
    /// Monitor operation
    Monitor,
    /// Health check operation
    HealthCheck,
    /// Configuration operation
    Configure,
    /// Security operation
    Security,
    /// Custom operation type
    Custom(String),
}
impl Default for UnifiedOperationType {
    /// Returns the default instance
    fn default() -> Self {
        Self::Read
    }
}

impl fmt::Display for UnifiedOperationType {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Create => write!(f, "create"),
            Self::Read => write!(f, "read"),
            Self::Update => write!(f, "update"),
            Self::Delete => write!(f, "delete"),
            Self::List => write!(f, "list"),
            Self::Search => write!(f, "search"),
            Self::Backup => write!(f, "backup"),
            Self::Restore => write!(f, "restore"),
            Self::Sync => write!(f, "sync"),
            Self::Monitor => write!(f, "monitor"),
            Self::HealthCheck => write!(f, "health_check"),
            Self::Configure => write!(f, "configure"),
            Self::Security => write!(f, "security"),
            Self::Custom(op_type) => write!(f, "{op_type}"),
        }
    }
}

// ==================== SECTION ====================

/// **THE** `AlertType` - unified across all modules
/// Replaces `AlertType` definitions in monitoring and notification systems
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of `UnifiedAlert`
pub enum UnifiedAlertType {
    /// System performance alert
    Performance,
    /// Security-related alert
    Security,
    /// Storage capacity alert
    Storage,
    /// Network connectivity alert
    Network,
    /// Service availability alert
    Service,
    /// Configuration issue alert
    Configuration,
    /// Resource utilization alert
    Resource,
    /// Error condition alert
    Error,
    /// Custom alert type
    Custom(String),
}
impl Default for UnifiedAlertType {
    /// Returns the default instance
    fn default() -> Self {
        Self::Performance
    }
}

/// **THE** `AlertSeverity` - unified across all modules
/// Replaces `AlertSeverity` definitions in monitoring and notification systems
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Unifiedalertseverity
pub enum UnifiedAlertSeverity {
    /// Critical severity - immediate attention required
    Critical,
    /// High severity - urgent attention needed
    High,
    /// Medium severity - attention needed soon
    Medium,
    /// Low severity - informational
    Low,
    /// Info severity - general information
    Info,
    /// Custom severity level
    Custom(String),
}
impl Default for UnifiedAlertSeverity {
    /// Returns the default instance
    fn default() -> Self {
        Self::Info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_message_type_default() {
        assert!(matches!(
            UnifiedMessageType::default(),
            UnifiedMessageType::Request
        ));
    }

    #[test]
    fn test_unified_event_type_display() {
        assert_eq!(UnifiedEventType::SystemStart.to_string(), "system_start");
        assert_eq!(UnifiedEventType::StorageEvent.to_string(), "storage_event");
    }

    #[test]
    fn test_unified_operation_type_serialization() {
        let op = UnifiedOperationType::Create;
        let json = serde_json::to_string(&op).unwrap();
        let parsed: UnifiedOperationType = serde_json::from_str(&json).unwrap();
        assert_eq!(op, parsed);
    }

    #[test]
    fn test_unified_alert_type_default() {
        assert!(matches!(
            UnifiedAlertType::default(),
            UnifiedAlertType::Performance
        ));
    }

    #[test]
    fn test_unified_alert_severity_variants() {
        let severity = UnifiedAlertSeverity::Critical;
        assert!(matches!(severity, UnifiedAlertSeverity::Critical));
        assert_eq!(UnifiedAlertSeverity::default(), UnifiedAlertSeverity::Info);
    }
}
