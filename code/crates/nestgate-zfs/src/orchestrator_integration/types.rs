// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Type definitions for orchestrator integration
//!
//! This module contains all type definitions used for orchestrator integration,
//! including service registration, configuration, and health status types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Service registration information for orchestrator integration
///
/// This structure contains all the information needed to register a ZFS service
/// with an orchestrator (capability-based orchestration provider or Kubernetes service discovery).
///
/// # Fields
///
/// * `service_id` - Unique identifier for this service instance
/// * `service_type` - Type of service (e.g., "zfs-storage", "zfs-compute")
/// * `capabilities` - List of capabilities this service provides
/// * `endpoints` - Network endpoints where this service is accessible
/// * `metadata` - Additional key-value metadata for service discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistration {
    /// Unique identifier for this service instance
    pub service_id: String,
    /// Type of service being registered
    pub service_type: String,
    /// List of capabilities this service provides
    pub capabilities: Vec<String>,
    /// Network endpoints where this service is accessible
    pub endpoints: Vec<String>,
    /// Additional metadata for service discovery
    pub metadata: HashMap<String, String>,
}

/// Configuration for ZFS service
///
/// # Migration Note
///
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::ZfsServiceConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsServiceConfig {
    /// Service name
    pub service_name: String,
    /// Bind Address
    pub bind_address: String,
    /// Port
    pub port: u16,
    /// Orchestrator Endpoints
    pub orchestrator_endpoints: Vec<String>,
    /// Health Check Interval (seconds)
    pub health_check_interval: u64,
    /// Capabilities
    pub capabilities: Vec<String>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

impl Default for ZfsServiceConfig {
    fn default() -> Self {
        // ✅ MIGRATED: Now uses centralized runtime configuration
        use nestgate_core::config::runtime::get_config;
        let config = get_config();

        Self {
            service_name: "nestgate-zfs".to_string(),
            bind_address: if config.network.bind_all {
                nestgate_core::constants::hardcoding::addresses::BIND_ALL_IPV4.to_string()
            } else {
                config.network.api_host.to_string()
            },
            port: config.network.api_port,
            orchestrator_endpoints: vec![],
            health_check_interval: 30,
            capabilities: vec![
                "zfs-pool-management".to_string(),
                "zfs-dataset-management".to_string(),
                "zfs-snapshot-management".to_string(),
                "tier-management".to_string(),
            ],
            metadata: HashMap::new(),
        }
    }
}

/// Health status for ZFS service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsHealthStatus {
    /// Node identifier
    pub node_id: String,
    /// Status (healthy, degraded, unhealthy)
    pub status: String,
    /// Pools Healthy
    pub pools_healthy: bool,
    /// Datasets Healthy
    pub datasets_healthy: bool,
    /// System Healthy
    pub system_healthy: bool,
    /// Total Capacity (bytes)
    pub total_capacity: u64,
    /// Available Capacity (bytes)
    pub available_capacity: u64,
    /// Last Check (Unix timestamp)
    pub last_check: u64,
}

/// Service information for discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Service identifier
    pub service_id: String,
    /// Service type
    pub service_type: String,
    /// Service endpoints
    pub endpoints: Vec<String>,
    /// Service capabilities
    pub capabilities: Vec<String>,
    /// Service metadata
    pub metadata: HashMap<String, String>,
    /// Last heartbeat timestamp
    pub last_heartbeat: Option<u64>,
}

impl ServiceInfo {
    /// Create a new `ServiceInfo` instance
    pub fn new(service_id: impl Into<String>, service_type: impl Into<String>) -> Self {
        Self {
            service_id: service_id.into(),
            service_type: service_type.into(),
            endpoints: Vec::new(),
            capabilities: Vec::new(),
            metadata: HashMap::new(),
            last_heartbeat: None,
        }
    }

    /// Generate a unique service ID
    #[must_use]
    pub fn generate_id() -> String {
        Uuid::new_v4().to_string()
    }
}
