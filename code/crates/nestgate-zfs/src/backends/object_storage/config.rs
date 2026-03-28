// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **CONFIGURATION STRUCTURES**
//!
//! Configuration discovery and storage capability types.

/// Configuration source for object storage
#[derive(Debug, Clone)]
#[allow(dead_code)] // Variants/fields used for future telemetry features
pub enum ConfigSource {
    /// Discovered via capability system (preferred)
    CapabilityDiscovered {
        /// Service ID from discovery
        service_id: String,
        /// Capability type
        capability: StorageCapability,
    },
    /// Environment variables (fallback)
    Environment,
}

/// Storage capability types
#[derive(Debug, Clone)]
#[allow(dead_code)] // Variants/fields used for capability discovery system
pub enum StorageCapability {
    /// S3-compatible API
    S3Compatible {
        /// API version
        version: String,
    },
    /// Native provider API (not used in this backend)
    Native {
        /// Provider name
        provider: String,
    },
}

/// Discovered object storage configuration
#[derive(Debug, Clone)]
pub struct DiscoveredStorageConfig {
    /// Service ID from discovery
    pub service_id: String,
    /// Endpoint URL
    pub endpoint: String,
    /// Region
    pub region: String,
    /// Bucket prefix
    pub bucket_prefix: String,
    /// Storage capability
    pub capability: StorageCapability,
    /// Path-style requests
    pub path_style: bool,
}
