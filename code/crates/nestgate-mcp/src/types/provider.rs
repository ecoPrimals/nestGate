// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::auth::AuthConfig;

/// Provider configuration (Enhanced with proven patterns)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Provider
pub struct ProviderConfig {
    /// Provider Type
    pub provider_type: String,
    /// Endpoint
    pub endpoint: String,
    /// Region
    pub region: Option<String>,
    /// Credentials
    pub credentials: Option<AuthConfig>,
    /// Configuration for custom
    pub custom_config: HashMap<String, serde_json::Value>,
}
/// Capabilities supported by a storage provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Providercapabilities
pub struct ProviderCapabilities {
    /// Supported storage protocols
    pub protocols: Vec<String>,
    /// Maximum capacity in bytes
    pub max_capacity: u64,
    /// IOPS capabilities
    pub iops: IopsCapabilities,
    /// Throughput capabilities
    pub throughput: ThroughputCapabilities,
    /// Availability zones
    pub availability_zones: Vec<String>,
}
impl Default for ProviderCapabilities {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            protocols: vec!["nfs".to_string(), "smb".to_string()],
            max_capacity: 1_000_000_000_000, // 1TB default
            iops: IopsCapabilities::default(),
            throughput: ThroughputCapabilities::default(),
            availability_zones: vec!["local".to_string()],
        }
    }
}

/// IOPS capabilities of a storage provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Iopscapabilities
pub struct IopsCapabilities {
    /// Minimum guaranteed IOPS.
    pub min_iops: u32,
    /// Maximum supported IOPS.
    pub max_iops: u32,
    /// Burst IOPS limit.
    pub burst_iops: Option<u32>,
}
impl Default for IopsCapabilities {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            min_iops: 1000,
            max_iops: 50000,
            burst_iops: Some(10_000),
        }
    }
}

/// Throughput capabilities of a storage provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Throughputcapabilities
pub struct ThroughputCapabilities {
    /// Minimum guaranteed throughput in MB/s.
    pub min_throughput_mbs: u32,
    /// Maximum supported throughput in MB/s.
    pub max_throughput_mbs: u32,
    /// Burst throughput limit in MB/s.
    pub burst_throughput_mbs: Option<u32>,
}
impl Default for ThroughputCapabilities {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            min_throughput_mbs: 100,          // 100MB/s
            max_throughput_mbs: 1000,         // 1GB/s
            burst_throughput_mbs: Some(2000), // 2GB/s burst
        }
    }
}

/// Status of a storage provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Status values for Provider
pub enum ProviderStatus {
    /// Provider is online and ready.
    Online,
    /// Provider is offline or unreachable.
    Offline,
    /// Provider is in maintenance mode.
    Maintenance,
    /// Provider is in an error state.
    Error(String),
}
/// Information about a storage provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Providerinfo
pub struct ProviderInfo {
    /// Unique identifier for the provider.
    pub id: String,
    /// Name of the provider.
    pub name: String,
    /// Current status of the provider.
    pub status: ProviderStatus,
    /// Total storage capacity in bytes.
    pub total_capacity: u64,
    /// Available storage capacity in bytes.
    pub available_capacity: u64,
}
/// Filter criteria for listing providers.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Providerfilter
pub struct ProviderFilter {
    /// Filter by provider status.
    pub status: Option<ProviderStatus>,
    /// Filter by minimum available capacity.
    pub min_capacity: Option<u64>,
}
