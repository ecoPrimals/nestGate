// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

///
/// This module provides unified event type definitions to replace the numerous
/// scattered EventType enums throughout the codebase, eliminating duplication
/// and providing a single source of truth for event categorization.
///
/// **REPLACES**:
/// - WebSocketEventType (nestgate-api/src/websocket.rs)
/// - CoordinatedEventType (nestgate-api/src/event_coordination.rs)
/// - AuditEventType (nestgate-api/src/handlers/compliance.rs)
/// - SseEventType (nestgate-api/src/sse.rs)
/// - StreamEventType (nestgate-api/src/mcp_streaming.rs)
/// - ManagementEventType (nestgate-api/src/ecosystem/management_integration.rs)
use serde::{Deserialize, Serialize};
/// **CANONICAL**: Unified event type for all system events
/// This replaces multiple scattered EventType enums with a single, comprehensive classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Types of UnifiedEvent
pub enum UnifiedEventType {
    // === CONNECTION EVENTS ===
    /// Connection established (WebSocket, RPC, etc.)
    ConnectionEstablished,
    /// Connection closed or lost
    ConnectionClosed,
    /// Connection error occurred
    ConnectionError,
    /// Ping/keepalive event
    Ping,
    /// Pong response
    Pong,
    // === DATA EVENTS ===
    /// Data access event
    DataAccess,
    /// Data modification event
    DataModification,
    /// Data deletion event
    DataDeletion,
    /// Data synchronization event
    DataSync,
    /// Data backup event
    DataBackup,

    // === SERVICE EVENTS ===
    /// Service started
    ServiceStarted,
    /// Service stopped
    ServiceStopped,
    /// Service health check
    ServiceHealthCheck,
    /// Service discovery event
    ServiceDiscovery,
    /// Service registration
    ServiceRegistration,

    // === SECURITY EVENTS ===
    /// Authentication event
    Authentication,
    /// Authorization check
    Authorization,
    /// Security violation detected
    SecurityViolation,
    /// Policy change
    PolicyChange,
    /// Compliance check
    ComplianceCheck,

    // === SYSTEM EVENTS ===
    /// System configuration change
    SystemConfiguration,
    /// System monitoring event
    SystemMonitoring,
    /// System error
    SystemError,
    /// System maintenance
    SystemMaintenance,

    // === STORAGE EVENTS ===
    /// Storage operation (read/write)
    StorageOperation,
    /// Storage replication
    StorageReplication,
    /// Storage snapshot
    StorageSnapshot,
    /// Storage cleanup
    StorageCleanup,

    // === STREAMING EVENTS ===
    /// Stream started
    StreamStarted,
    /// Stream data
    StreamData,
    /// Stream ended
    StreamEnded,
    /// Stream error
    StreamError,

    // === NOTIFICATION EVENTS ===
    /// Alert generated
    Alert,
    /// Notification sent
    Notification,
    /// Message received
    Message,
    /// Broadcast event
    Broadcast,

    // === WORKFLOW EVENTS ===
    /// Workflow started
    WorkflowStarted,
    /// Workflow step completed
    WorkflowStep,
    /// Workflow completed
    WorkflowCompleted,
    /// Workflow failed
    WorkflowFailed,

    // === CUSTOM EVENTS ===
    /// Custom event with string identifier
    Custom(String),
}

impl Default for UnifiedEventType {
    /// Returns the default instance
    fn default() -> Self {
        Self::SystemMonitoring
    }
}

impl UnifiedEventType {
    /// Get the event category for grouping and filtering
    pub fn category(&self) -> EventCategory {
        match self {
            Self::ConnectionEstablished
            | Self::ConnectionClosed
            | Self::ConnectionError
            | Self::Ping
            | Self::Pong => EventCategory::Connection,

            Self::DataAccess
            | Self::DataModification
            | Self::DataDeletion
            | Self::DataSync
            | Self::DataBackup => EventCategory::Data,

            Self::ServiceStarted
            | Self::ServiceStopped
            | Self::ServiceHealthCheck
            | Self::ServiceDiscovery
            | Self::ServiceRegistration => EventCategory::Service,

            Self::Authentication
            | Self::Authorization
            | Self::SecurityViolation
            | Self::PolicyChange
            | Self::ComplianceCheck => EventCategory::Security,

            Self::SystemConfiguration
            | Self::SystemMonitoring
            | Self::SystemError
            | Self::SystemMaintenance => EventCategory::System,

            Self::StorageOperation
            | Self::StorageReplication
            | Self::StorageSnapshot
            | Self::StorageCleanup => EventCategory::Storage,

            Self::StreamStarted | Self::StreamData | Self::StreamEnded | Self::StreamError => {
                EventCategory::Streaming
            }

            Self::Alert | Self::Notification | Self::Message | Self::Broadcast => {
                EventCategory::Notification
            }

            Self::WorkflowStarted
            | Self::WorkflowStep
            | Self::WorkflowCompleted
            | Self::WorkflowFailed => EventCategory::Workflow,

            Self::Custom(_) => EventCategory::Custom,
        }
    }

    /// Check if this event type requires authentication
    pub fn requires_auth(&self) -> bool {
        matches!(
            self,
            Self::DataModification
                | Self::DataDeletion
                | Self::SystemConfiguration
                | Self::PolicyChange
                | Self::ServiceRegistration
                | Self::StorageOperation
        )
    }

    /// Check if this event should be audited
    pub fn should_audit(&self) -> bool {
        matches!(
            self,
            Self::DataAccess
                | Self::DataModification
                | Self::DataDeletion
                | Self::Authentication
                | Self::Authorization
                | Self::SecurityViolation
                | Self::PolicyChange
                | Self::SystemConfiguration
        )
    }

    /// Get the priority level for this event type
    pub fn priority(&self) -> EventPriority {
        match self {
            Self::SecurityViolation | Self::SystemError => EventPriority::Critical,
            Self::ConnectionError | Self::StreamError | Self::WorkflowFailed => EventPriority::High,
            Self::Alert | Self::SystemMaintenance | Self::ServiceHealthCheck => {
                EventPriority::Normal
            }
            _ => EventPriority::Low,
        }
    }
}

/// Event categories for grouping and filtering
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Eventcategory
pub enum EventCategory {
    /// Connection
    Connection,
    /// Data
    Data,
    /// Service
    Service,
    /// Security
    Security,
    /// System
    System,
    /// Storage
    Storage,
    /// Streaming
    Streaming,
    /// Notification
    Notification,
    /// Workflow
    Workflow,
    /// Custom
    Custom,
}
/// Event priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
/// Eventpriority
pub enum EventPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}
/// **MIGRATION HELPERS**
/// These functions help migrate from old EventType enums to the unified system
impl From<&str> for UnifiedEventType {
    /// From
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "connectionestablished" | "connection_established" => Self::ConnectionEstablished,
            "connectionclosed" | "connection_closed" | "disconnection" => Self::ConnectionClosed,
            "connectionerror" | "connection_error" | "error" => Self::ConnectionError,
            "ping" => Self::Ping,
            "pong" => Self::Pong,
            "message" => Self::Message,
            "websocket" => Self::ConnectionEstablished,
            "internalservice" | "internal_service" => Self::ServiceHealthCheck,
            "mcpstream" | "mcp_stream" => Self::StreamData,
            "storageoperation" | "storage_operation" => Self::StorageOperation,
            "configurationchange" | "configuration_change" => Self::SystemConfiguration,
            "healthmonitoring" | "health_monitoring" => Self::ServiceHealthCheck,
            "dataaccess" | "data_access" => Self::DataAccess,
            "datamodification" | "data_modification" => Self::DataModification,
            "datadeletion" | "data_deletion" => Self::DataDeletion,
            "authentication" => Self::Authentication,
            "authorization" => Self::Authorization,
            "policychange" | "policy_change" => Self::PolicyChange,
            "systemconfiguration" | "system_configuration" => Self::SystemConfiguration,
            "complianceviolation" | "compliance_violation" => Self::SecurityViolation,
            _ => Self::Custom(s.to_string()),
        }
    }
}

impl std::fmt::Display for UnifiedEventType {
    /// Fmt
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Custom(s) => write!(f, "Custom({s})"),
            _ => write!(f, "{self:?}"),
        }
    }
}

/// **BACKWARD COMPATIBILITY**
/// Type aliases for existing code during migration
/// WebSocket event type compatibility
pub type WebSocketEventType = UnifiedEventType;
/// Coordinated event type compatibility  
pub type CoordinatedEventType = UnifiedEventType;
/// Audit event type compatibility
pub type AuditEventType = UnifiedEventType;
/// SSE event type compatibility
pub type SseEventType = UnifiedEventType;
/// Stream event type compatibility
pub type StreamEventType = UnifiedEventType;
/// Management event type compatibility
pub type ManagementEventType = UnifiedEventType;
