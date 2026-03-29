// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// Core Diagnostic Types and Enums
/// This module contains the fundamental types used throughout the diagnostics system.
use serde::{Deserialize, Serialize};
use std::fmt;
/// System diagnostic level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Diagnosticlevel
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
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => write!(f, "INFO"),
            Self::Warning => write!(f, "WARNING"),
            Self::Error => write!(f, "ERROR"),
            Self::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// System component type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of Component
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
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cpu => write!(f, "CPU"),
            Self::Memory => write!(f, "Memory"),
            Self::Storage => write!(f, "Storage"),
            Self::Network => write!(f, "Network"),
            Self::System => write!(f, "System"),
            Self::Application => write!(f, "Application"),
            Self::Database => write!(f, "Database"),
            Self::Cache => write!(f, "Cache"),
        }
    }
}

/// Service information for diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serviceinfo
pub struct ServiceInfo {
    /// Name
    pub name: String,
    /// Version
    pub version: String,
    /// Status
    pub status: String,
    /// Start Time
    pub start_time: Option<std::time::SystemTime>,
    /// Pid
    pub pid: Option<u32>,
    /// Memory Bytes
    pub memory_bytes: Option<u64>,
    /// Human-readable description
    pub description: Option<String>,
    /// Dependencies
    pub dependencies: Option<Vec<String>>,
    /// Cpu Percent
    pub cpu_percent: Option<f64>,
    /// Command Line
    pub command_line: Option<String>,
}
impl Default for ServiceInfo {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            name: "unknown".to_string(),
            version: "0.0.0".to_string(),
            status: "unknown".to_string(),
            start_time: None,
            pid: None,
            memory_bytes: None,
            description: None,
            dependencies: None,
            cpu_percent: None,
            command_line: None,
        }
    }
}

// Use UnifiedHealthStatus and UnifiedServiceState from nestgate_types::unified_enums instead
