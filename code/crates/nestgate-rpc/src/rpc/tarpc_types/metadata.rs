// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Capability discovery, health, and protocol metadata for [`super::NestGateRpc`].

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Capability registration for discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRegistration {
    /// Service ID (UUID)
    pub service_id: Arc<str>,

    /// Service name (`NestGate` primal identity: `"nestgate"`, not the compiling crate name)
    pub service_name: Arc<str>,

    /// Primary capability ("storage")
    pub capability: Arc<str>,

    /// All capabilities provided
    pub capabilities: Vec<Arc<str>>,

    /// tarpc endpoint
    pub tarpc_endpoint: Arc<str>,

    /// JSON-RPC endpoint
    pub jsonrpc_endpoint: Option<Arc<str>>,

    /// HTTP endpoint (if enabled)
    pub http_endpoint: Option<Arc<str>>,

    /// Service metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// Discovered service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Service ID
    pub id: Arc<str>,

    /// Service capability
    pub capability: Arc<str>,

    /// Available endpoints by protocol
    pub endpoints: HashMap<String, String>,

    /// Service status
    pub status: Arc<str>,

    /// Service metadata
    pub metadata: Option<serde_json::Value>,
}

/// Registration result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationResult {
    /// Success flag
    pub success: bool,

    /// Result message
    pub message: String,
}

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Status string ("healthy", "degraded", "unhealthy")
    pub status: String,

    /// Service version
    pub version: String,

    /// Uptime in seconds
    pub uptime_seconds: u64,

    /// Total datasets
    pub total_datasets: usize,

    /// Total objects
    pub total_objects: u64,

    /// Storage used in bytes
    pub storage_used_bytes: u64,

    /// Additional health metrics
    #[serde(default)]
    pub metrics: HashMap<String, serde_json::Value>,
}

/// Storage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    /// Total capacity in bytes
    pub total_capacity_bytes: u64,

    /// Used space in bytes
    pub used_space_bytes: u64,

    /// Available space in bytes
    pub available_space_bytes: u64,

    /// Number of datasets
    pub dataset_count: usize,

    /// Total number of objects
    pub object_count: u64,

    /// Average compression ratio
    pub avg_compression_ratio: f64,

    /// Deduplication ratio
    pub dedup_ratio: f64,

    /// Read operations per second
    pub read_ops_per_sec: f64,

    /// Write operations per second
    pub write_ops_per_sec: f64,

    /// Average read latency in milliseconds
    pub avg_read_latency_ms: f64,

    /// Average write latency in milliseconds
    pub avg_write_latency_ms: f64,
}

/// Version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    /// Service version
    pub version: String,

    /// API version
    pub api_version: String,

    /// Supported protocol versions
    pub protocol_versions: Vec<String>,

    /// Build information
    pub build_info: Option<String>,
}

/// Protocol information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolInfo {
    /// Protocol name ("tarpc", "jsonrpc", "http")
    pub protocol: String,

    /// Protocol version
    pub version: String,

    /// Connection endpoint
    pub endpoint: String,

    /// Priority (1 = highest)
    pub priority: u8,

    /// Enabled flag
    pub enabled: bool,
}
