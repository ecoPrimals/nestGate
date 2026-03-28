// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Storage configuration types
//!
//! Configuration structures for storage resources and performance requirements.

use super::{providers::UniversalStorageType, resources::StorageResourceType};
use nestgate_types::unified_enums::UnifiedTierType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Storage resource configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_config::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct StorageResourceConfig {
    /// Resource name
    pub name: String,
    /// Storage type
    pub storage_type: UniversalStorageType,
    /// Resource type
    pub resource_type: StorageResourceType,
    /// Storage tier
    pub tier: UnifiedTierType,
    /// Initial size in bytes
    pub size_bytes: Option<u64>,
    /// Configuration metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Storage performance requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePerformanceRequirements {
    /// Minimum read IOPS
    pub min_read_iops: Option<u64>,
    /// Minimum write IOPS
    pub min_write_iops: Option<u64>,
    /// Minimum read throughput (bytes/sec)
    pub min_read_throughput: Option<u64>,
    /// Minimum write throughput (bytes/sec)
    pub min_write_throughput: Option<u64>,
    /// Maximum latency (milliseconds)
    pub max_latency_ms: Option<f64>,
}

// NOTE: This module is deprecated - use canonical_primary::domains::network instead
// All network configuration has been unified in the canonical config system
