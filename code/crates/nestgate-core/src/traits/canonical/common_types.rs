//! Common Types for Canonical Traits
//!
//! Shared types used across canonical trait definitions.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Service Capabilities Information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicecapabilities
pub struct ServiceCapabilities {
    /// Supported Operations
    pub supported_operations: Vec<String>,
    /// Version
    pub version: String,
}

/// Provider Health Status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Providerhealth
pub struct ProviderHealth {
    /// Whether healthy
    pub is_healthy: bool,
    /// Last Check
    pub last_check: SystemTime,
}

impl Default for ProviderHealth {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            is_healthy: true,
            last_check: SystemTime::now(),
        }
    }
}

/// Provider Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Providercapabilities
pub struct ProviderCapabilities {
    /// Features
    pub features: Vec<String>,
}

/// Storage Usage Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storageusagestats
pub struct StorageUsageStats {
    /// Total Bytes
    pub total_bytes: u64,
    /// Used Bytes
    pub used_bytes: u64,
    /// Available Bytes
    pub available_bytes: u64,
    /// Usage Percentage
    pub usage_percentage: f64,
}

/// Connection Handle (opaque identifier)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Connectionhandle
pub struct ConnectionHandle(pub u64);

/// Connection Status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Status values for Connection
pub enum ConnectionStatus {
    /// Connected
    Connected,
    /// Disconnected
    Disconnected,
    /// Connecting
    Connecting,
    /// Error
    Error,
}

/// Health Status Enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Status values for Health
pub enum HealthStatus {
    /// Healthy
    Healthy,
    /// Degraded
    Degraded,
    /// Unhealthy
    Unhealthy,
    /// Unknown
    Unknown,
}

/// Security Credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Securitycredentials
pub struct SecurityCredentials {
    /// Username
    pub username: String,
    /// Token
    pub token: String,
}

/// Cron Schedule Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cronschedule
pub struct CronSchedule {
    /// Expression
    pub expression: String,
}

/// Schedule Identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Scheduleid
pub struct ScheduleId {
    /// Unique identifier
    pub id: String,
}

/// Schedule Information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Scheduleinfo
pub struct ScheduleInfo {
    /// Unique identifier
    pub id: ScheduleId,
    /// Schedule
    pub schedule: CronSchedule,
    /// Whether this feature is enabled
    pub enabled: bool,
}

/// Storage Backend Type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of StorageBackend
pub enum StorageBackendType {
    /// Local
    Local,
    /// Zfs
    Zfs,
    /// Network
    Network,
    /// S3
    S3,
    /// Azure
    Azure,
    /// Gcs
    Gcs,
    Custom(String),
}

/// Storage Capability
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Storagecapability
pub enum StorageCapability {
    /// Snapshots
    Snapshots,
    /// Compression
    Compression,
    /// Encryption
    Encryption,
    /// Deduplication
    Deduplication,
    /// Replication
    Replication,
    /// Quotas
    Quotas,
}

/// Snapshot Information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Snapshotinfo
pub struct SnapshotInfo {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Size Bytes
    pub size_bytes: u64,
}

/// Data Stream (for reading)
#[derive(Debug)]
/// Datastream
pub struct DataStream {
    /// Stream identifier
    pub stream_id: u64,
}

/// Write Stream (for writing)
#[derive(Debug)]
/// Writestream
pub struct WriteStream {
    /// Stream identifier
    pub stream_id: u64,
}

