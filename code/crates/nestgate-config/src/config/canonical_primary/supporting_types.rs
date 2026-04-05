// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

/// **SUPPORTING TYPES**
///
/// Common types and enums used across all configuration modules.
/// This module contains shared types that don't belong to a specific domain.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ==================== SECTION ====================

/// Monitoring and observability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Monitoring
pub struct MonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,
    /// Metrics collection interval
    pub metrics_interval: Duration,
    /// Log retention period
    pub log_retention_days: u32,
    /// Enable distributed tracing
    pub tracing_enabled: bool,
    /// Monitoring endpoints
    pub endpoints: Vec<String>,
    /// Custom monitoring settings
    pub custom_settings: HashMap<String, serde_json::Value>,
}
impl Default for MonitoringConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_interval: Duration::from_secs(60),
            log_retention_days: 30,
            tracing_enabled: false,
            endpoints: vec!["http://localhost:9090".to_string()],
            custom_settings: HashMap::new(),
        }
    }
}

/// MCP (Model Context Protocol) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Mcp
pub struct McpConfig {
    /// Enable MCP protocol
    pub enabled: bool,
    /// MCP server port
    pub port: u16,
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Request timeout
    pub request_timeout: Duration,
    /// Protocol-specific settings
    pub protocol_settings: HashMap<String, serde_json::Value>,
}
impl Default for McpConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            port: 8082,
            max_connections: 100,
            request_timeout: Duration::from_secs(30),
            protocol_settings: HashMap::new(),
        }
    }
}

/// File system monitor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `FsMonitor`
pub struct FsMonitorConfig {
    /// Enable file system monitoring
    pub enabled: bool,
    /// Watched directories
    /// File patterns to ignore
    pub ignore_patterns: Vec<String>,
    /// Event processing settings
    pub event_settings: HashMap<String, serde_json::Value>,
}
impl Default for FsMonitorConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            ignore_patterns: vec!["*.tmp".to_string(), "*.log".to_string()],
            event_settings: HashMap::new(),
        }
    }
}

/// NAS configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for Nas
pub struct NasConfig {
    /// Enable NAS functionality
    pub enabled: bool,
    /// NAS mount points
    pub mount_points: Vec<String>,
    /// Share configurations
    pub shares: Vec<NasShare>,
    /// NAS-specific settings
    pub nas_settings: HashMap<String, serde_json::Value>,
}
/// NAS share configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Nasshare
pub struct NasShare {
    /// Share name
    pub name: String,
    /// Share path
    /// Read-only flag
    pub read_only: bool,
    /// Access permissions
    pub permissions: HashMap<String, String>,
}

/// Middleware configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Middleware
pub struct MiddlewareConfig {
    /// Enable middleware
    pub enabled: bool,
    /// Middleware chain
    pub middleware_chain: Vec<String>,
    /// Request processing settings
    pub request_settings: HashMap<String, serde_json::Value>,
    /// Response processing settings
    pub response_settings: HashMap<String, serde_json::Value>,
}
impl Default for MiddlewareConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            middleware_chain: vec![
                "cors".to_string(),
                "auth".to_string(),
                "logging".to_string(),
            ],
            request_settings: HashMap::new(),
            response_settings: HashMap::new(),
        }
    }
}
