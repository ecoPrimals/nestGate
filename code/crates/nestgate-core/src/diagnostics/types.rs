/// Core Diagnostic Types and Enums
/// This module contains the fundamental types used throughout the diagnostics system.
use serde::{Deserialize, Serialize};
use std::fmt;

/// System diagnostic level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticLevel {
    /// Informational diagnostic
    Info,
    /// Warning diagnostic
    Warning,
    /// Error diagnostic
    Error,
    /// Critical error diagnostic
    Critical,
}

impl fmt::Display for DiagnosticLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticLevel::Info => write!(f, "INFO"),
            DiagnosticLevel::Warning => write!(f, "WARNING"),
            DiagnosticLevel::Error => write!(f, "ERROR"),
            DiagnosticLevel::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// System component type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentType {
    /// CPU
    Cpu,
    /// Memory
    Memory,
    /// Storage
    Storage,
    /// Network
    Network,
    /// System
    System,
    /// Application
    Application,
    /// Database
    Database,
    /// Cache
    Cache,
}

impl fmt::Display for ComponentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComponentType::Cpu => write!(f, "CPU"),
            ComponentType::Memory => write!(f, "Memory"),
            ComponentType::Storage => write!(f, "Storage"),
            ComponentType::Network => write!(f, "Network"),
            ComponentType::System => write!(f, "System"),
            ComponentType::Application => write!(f, "Application"),
            ComponentType::Database => write!(f, "Database"),
            ComponentType::Cache => write!(f, "Cache"),
        }
    }
}

/// Service information for diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub version: String,
    pub status: String,
    pub start_time: Option<std::time::SystemTime>,
    pub pid: Option<u32>,
    pub memory_bytes: Option<u64>,
    pub log_path: Option<String>,
    pub environment: Option<String>,
    pub description: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub cpu_percent: Option<f64>,
    pub config_path: Option<String>,
    pub command_line: Option<String>,
}

impl Default for ServiceInfo {
    fn default() -> Self {
        Self {
            name: "unknown".to_string(),
            version: "0.0.0".to_string(),
            status: "unknown".to_string(),
            start_time: None,
            pid: None,
            memory_bytes: None,
            log_path: None,
            environment: None,
            description: None,
            dependencies: None,
            cpu_percent: None,
            config_path: None,
            command_line: None,
        }
    }
}

// Use UnifiedHealthStatus and UnifiedServiceState from crate::unified_enums instead
// This cleanup eliminates fragmented status type definitions in favor of the unified system.
