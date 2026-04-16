// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// **COMMON TYPES - CANONICAL MODERNIZED**
//! Type definitions for universal traits
// Common types and structures shared across universal traits modules.
// Organized for clarity and reusability across the ecosystem.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export types from other modules for convenience
pub use super::compute::{PerformanceMetrics, ResourceAllocation, ResourceSpec};
pub use super::ecosystem::{PrimalContext, PrimalHealth, PrimalInfo, PrimalType};
pub use super::orchestration::{ServiceHealth, ServiceInfo, ServiceRequest, ServiceResponse};
pub use super::security::{AuthToken, Credentials, Signature};

/// Network location information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networklocation
pub struct NetworkLocation {
    /// Host
    pub host: String,
    /// Port
    pub port: u16,
    /// Protocol
    pub protocol: String,
    /// Secure
    pub secure: bool,
}
/// Security level enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Securitylevel
pub enum SecurityLevel {
    /// Public
    Public,
    /// Internal
    Internal,
    /// Confidential
    Confidential,
    /// Restricted
    Restricted,
    /// Topsecret
    TopSecret,
}
/// Primal capability enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Primalcapability
pub enum PrimalCapability {
    /// Storage
    Storage,
    /// Compute
    Compute,
    /// Security
    Security,
    /// Orchestration
    Orchestration,
    /// Monitoring
    Monitoring,
    /// Analytics
    Analytics,
    /// Machinelearning
    MachineLearning,
    /// Dataprocessing
    DataProcessing,
    /// Networkmanagement
    NetworkManagement,
    /// A custom capability type defined by the user
    Custom(String),
}
/// Primal dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Primaldependency
pub enum PrimalDependency {
    /// A required dependency that must be present
    Required(String),
    /// An optional dependency that enhances functionality but isn't required
    Optional(String),
    /// A preferred dependency that should be used if available
    Preferred(String),
}
/// Primal endpoints configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Primalendpoints
pub struct PrimalEndpoints {
    /// Primary
    pub primary: NetworkLocation,
    /// Backup
    pub backup: Option<NetworkLocation>,
    /// Health Check
    pub health_check: Option<NetworkLocation>,
    /// Metrics
    pub metrics: Option<NetworkLocation>,
}
/// System metrics collection
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Systemmetrics
pub struct SystemMetrics {
    /// Cpu Usage Percent
    pub cpu_usage_percent: f64,
    /// Memory Usage Percent
    pub memory_usage_percent: f64,
    /// Disk Usage Percent
    pub disk_usage_percent: f64,
    /// Network Throughput Mbps
    pub network_throughput_mbps: f64,
    /// Active Connections
    pub active_connections: u32,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
}
/// Configuration structure for universal traits
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Universal
pub struct UniversalConfig {
    /// Primal identifier
    pub primal_id: String,
    /// Capabilities
    pub capabilities: Vec<PrimalCapability>,
    /// Endpoints
    pub endpoints: PrimalEndpoints,
    /// Security Level
    pub security_level: SecurityLevel,
    /// Dependencies
    pub dependencies: Vec<PrimalDependency>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
/// Ecosystem integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for EcosystemIntegration
pub struct EcosystemIntegrationConfig {
    /// Discovery Endpoint
    pub discovery_endpoint: String,
    /// Heartbeat Interval Seconds
    pub heartbeat_interval_seconds: u64,
    /// Timeout Seconds
    pub timeout_seconds: u64,
    /// Retry Attempts
    pub retry_attempts: u32,
    /// Configuration for security (canonical unified network configuration surface)
    pub security_config: SecurityConfigCanonical,
}

/// Type alias to canonical network configuration for ecosystem security settings.
pub type SecurityConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
