/// Error Context and Metadata
/// This module provides structured context information and metadata
/// for errors throughout the NestGate system.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use crate::unified_types::error_types::UnifiedErrorSeverity;

// ==================== ERROR CONTEXT AND METADATA ====================

/// **Rich Error Context**
/// Provides structured context information for all errors including request IDs,
/// user context, system state, and debugging information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Unique request/operation identifier for tracing
    pub request_id: Option<String>,
    /// User context when error occurred
    pub user_context: Option<UserContext>,
    /// System context and resource utilization
    pub system_context: Option<SystemContext>,
    /// Debugging and troubleshooting information
    pub debug_info: Option<DebugInfo>,
    /// Error occurrence timestamp
    pub timestamp: SystemTime,
    /// Error severity level
    pub severity: UnifiedErrorSeverity,
    /// Additional structured metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// User context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContext {
    /// User identifier
    pub user_id: Option<String>,
    /// User session ID
    pub session_id: Option<String>,
    /// IP address or client identifier
    pub client_ip: Option<String>,
    /// User agent or client information
    pub user_agent: Option<String>,
    /// User's current permissions
    pub permissions: Vec<String>,
    /// User's roles
    pub roles: Vec<String>,
}

/// System context at time of error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemContext {
    /// Server/node identifier
    pub node_id: Option<String>,
    /// Service version
    pub version: Option<String>,
    /// System load and resource utilization
    pub load_average: Option<f64>,
    /// Memory usage percentage
    pub memory_usage: Option<f64>,
    /// Disk usage percentage
    pub disk_usage: Option<f64>,
    /// Active connection count
    pub active_connections: Option<u32>,
    /// Uptime in seconds
    pub uptime: Option<u64>,
}

/// Debugging information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugInfo {
    /// Source code location (file:line)
    pub location: Option<String>,
    /// Function call stack
    pub stack_trace: Option<Vec<String>>,
    /// Variable values and state
    pub variables: Option<HashMap<String, String>>,
    /// Related log entries
    pub log_entries: Option<Vec<String>>,
    /// Performance metrics
    pub performance: Option<PerformanceContext>,
}

/// Performance context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceContext {
    /// Operation duration
    pub duration: Option<Duration>,
    /// Memory allocated during operation
    pub memory_allocated: Option<u64>,
    /// I/O operations performed
    pub io_operations: Option<u32>,
    /// Network bytes transferred
    pub network_bytes: Option<u64>,
}

/// Security context for operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    /// User ID
    pub user_id: Option<String>,
    /// Session ID
    pub session_id: Option<String>,
    /// Source IP address
    pub source_ip: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
    /// User permissions
    pub permissions: Vec<String>,
    /// User roles
    pub roles: Vec<String>,
}

// ErrorSeverity enum removed - now using UnifiedErrorSeverity from unified_types

/// Security severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    /// Informational security event
    Info,
    /// Low security risk
    Low,
    /// Medium security risk
    Medium,
    /// High security risk
    High,
    /// Critical security breach
    Critical,
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self {
            request_id: None,
            user_context: None,
            system_context: None,
            debug_info: None,
            timestamp: SystemTime::now(),
            severity: UnifiedErrorSeverity::Warning,
            metadata: HashMap::new(),
        }
    }
}

impl ErrorContext {
    /// Create a new error context with basic information
    pub fn new() -> Self {
        Self::default()
    }

    /// Set request ID for tracing
    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }

    /// Set user context
    pub fn with_user_context(mut self, user_context: UserContext) -> Self {
        self.user_context = Some(user_context);
        self
    }

    /// Set error severity
    pub fn with_severity(mut self, severity: UnifiedErrorSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}
