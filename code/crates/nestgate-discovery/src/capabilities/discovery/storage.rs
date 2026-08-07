// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage capability self-knowledge registry.
//!
//! Provides the **static capability manifest** for nestGate's own storage domain.
//! These are not discovered at runtime from external primals — they are nestGate's
//! self-knowledge of what it can serve. External primals discover these via
//! `capabilities.list` / `primal.announce` over IPC.

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Storage capability types that can be advertised.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StorageCapabilityType {
    /// ZFS pool management capabilities.
    ZfsPool,
    /// Dataset creation and management.
    Dataset,
    /// Snapshot management and operations.
    Snapshot,
    /// Backup and restore capabilities.
    Backup,
    /// Data migration services.
    Migration,
    /// Performance monitoring and optimization.
    Performance,
    /// Storage health monitoring.
    Monitoring,
    /// Encryption at rest capabilities.
    Encryption,
}

/// Static metadata for an advertised storage capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCapabilityInfo {
    /// Type of storage capability provided.
    pub capability_type: StorageCapabilityType,
    /// Internal capability domain URI (not a network address).
    pub endpoint: String,
    /// API version string.
    pub version: String,
    /// List of supported operations for this capability.
    pub supported_operations: Vec<String>,
    /// Additional metadata key-value pairs.
    pub metadata: HashMap<String, String>,
}

/// Storage capability self-knowledge registry.
///
/// Returns nestGate's own capability manifest. These are internal domain URIs
/// for routing, not external endpoints discovered at runtime.
#[derive(Debug)]
pub struct StorageCapabilityDiscovery {
    discovered_capabilities:
        tokio::sync::RwLock<HashMap<StorageCapabilityType, StorageCapabilityInfo>>,
}

impl StorageCapabilityDiscovery {
    /// Create new storage capability registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            discovered_capabilities: tokio::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Return nestGate's static storage capability manifest.
    ///
    /// # Errors
    ///
    /// Returns an error if the internal cache lock is poisoned.
    pub async fn discover_capabilities(&self) -> Result<Vec<StorageCapabilityInfo>> {
        let capabilities = vec![
            Self::zfs_pool_capability(),
            Self::dataset_capability(),
        ];

        let mut cache = self.discovered_capabilities.write().await;
        for capability in &capabilities {
            cache.insert(capability.capability_type.clone(), capability.clone());
        }

        Ok(capabilities)
    }

    /// Get specific storage capability by type.
    ///
    /// # Errors
    ///
    /// Returns an error if the internal cache lock is poisoned.
    pub async fn get_capability(
        &self,
        capability_type: &StorageCapabilityType,
    ) -> Result<Option<StorageCapabilityInfo>> {
        let cache = self.discovered_capabilities.read().await;
        Ok(cache.get(capability_type).cloned())
    }

    fn zfs_pool_capability() -> StorageCapabilityInfo {
        StorageCapabilityInfo {
            capability_type: StorageCapabilityType::ZfsPool,
            endpoint: "local://zfs.pool".into(),
            version: "1.0.0".into(),
            supported_operations: vec![
                "create_pool".into(),
                "destroy_pool".into(),
                "list_pools".into(),
                "pool_status".into(),
            ],
            metadata: HashMap::from([("source".into(), "self-knowledge".into())]),
        }
    }

    fn dataset_capability() -> StorageCapabilityInfo {
        StorageCapabilityInfo {
            capability_type: StorageCapabilityType::Dataset,
            endpoint: "local://zfs.dataset".into(),
            version: "1.0.0".into(),
            supported_operations: vec![
                "create_dataset".into(),
                "destroy_dataset".into(),
                "list_datasets".into(),
                "dataset_properties".into(),
            ],
            metadata: HashMap::from([("source".into(), "self-knowledge".into())]),
        }
    }
}

impl Default for StorageCapabilityDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

/// Get ZFS endpoint from self-knowledge manifest.
pub async fn get_zfs_endpoint(
    #[expect(unused_variables, reason = "adapter reserved for future capability-provider injection")]
    _adapter: &(),
) -> Result<String> {
    let discovery = StorageCapabilityDiscovery::new();
    let capabilities = discovery.discover_capabilities().await?;
    for capability in capabilities {
        if matches!(capability.capability_type, StorageCapabilityType::ZfsPool) {
            return Ok(capability.endpoint);
        }
    }
    Ok("local://zfs.pool".into())
}
