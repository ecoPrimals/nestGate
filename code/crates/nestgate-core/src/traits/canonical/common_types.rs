//! Common Types for Canonical Traits
//!
//! Shared types used across canonical trait definitions.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Service Capabilities Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCapabilities {
    pub supported_operations: Vec<String>,
    pub version: String,
}

/// Provider Health Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderHealth {
    pub is_healthy: bool,
    pub last_check: SystemTime,
}

impl Default for ProviderHealth {
    fn default() -> Self {
        Self {
            is_healthy: true,
            last_check: SystemTime::now(),
        }
    }
}

/// Provider Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapabilities {
    pub features: Vec<String>,
}

/// Storage Usage Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageUsageStats {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percentage: f64,
}

/// Connection Handle (opaque identifier)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionHandle(pub u64);

/// Connection Status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Connecting,
    Error,
}

/// Health Status Enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Security Credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCredentials {
    pub username: String,
    pub token: String,
}

/// Cron Schedule Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronSchedule {
    pub expression: String,
}

/// Schedule Identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScheduleId {
    pub id: String,
}

/// Schedule Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleInfo {
    pub id: ScheduleId,
    pub schedule: CronSchedule,
    pub enabled: bool,
}

/// Storage Backend Type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageBackendType {
    Local,
    Zfs,
    Network,
    S3,
    Azure,
    Gcs,
    Custom(String),
}

/// Storage Capability
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageCapability {
    Snapshots,
    Compression,
    Encryption,
    Deduplication,
    Replication,
    Quotas,
}

/// Snapshot Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    pub id: String,
    pub name: String,
    pub created_at: SystemTime,
    pub size_bytes: u64,
}

/// Data Stream (for reading)
#[derive(Debug)]
pub struct DataStream {
    pub stream_id: u64,
}

/// Write Stream (for writing)
#[derive(Debug)]
pub struct WriteStream {
    pub stream_id: u64,
}

