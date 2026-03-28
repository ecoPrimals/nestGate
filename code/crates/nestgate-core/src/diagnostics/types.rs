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
            DiagnosticLevel::Info => write!(f, "INFO"),
            DiagnosticLevel::Warning => write!(f, "WARNING"),
            DiagnosticLevel::Error => write!(f, "ERROR"),
            DiagnosticLevel::Critical => write!(f, "CRITICAL"),
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

// Use UnifiedHealthStatus and UnifiedServiceState from crate::unified_enums instead
// This cleanup eliminates fragmented status type definitions in favor of the unified system.

#[cfg(test)]
#[path = "types_tests.rs"]
mod tests;
