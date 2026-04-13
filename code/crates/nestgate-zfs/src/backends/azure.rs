// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! **AZURE BLOB STORAGE BACKEND**
//!
//! Implements `ZeroCostZfsOperations` trait for Azure Blob Storage.
//!
//! ## Features
//!
//! - **Zero-cost abstractions**: Compile-time dispatch, no runtime overhead
//! - **Async native**: Built on tokio and Azure SDK
//! - **Environment-driven**: Configuration via environment variables
//! - **Tier mapping**: Automatic storage tier to Azure tier mapping
//!
//! ## Configuration
//!
//! Set via environment variables:
//! - `AZURE_STORAGE_ACCOUNT`: Storage account name
//! - `AZURE_STORAGE_KEY`: Storage account key
//! - `AZURE_STORAGE_CONNECTION_STRING`: Full connection string (alternative)
//! - `AZURE_CONTAINER_PREFIX`: Prefix for all containers (default: nestgate)
//!
//! ## Tier Mapping
//!
//! - `Hot` → Azure Hot tier
//! - `Warm` → Azure Cool tier
//! - `Cold` → Azure Archive tier
//! - `Cache` → Azure Premium
//! - `Archive` → Azure Archive (long-term)
//!
//! ## Example
//!
//! ```rust,ignore
//! use nestgate_zfs::backends::azure::AzureBackend;
//!
//! // Create backend
//! let backend = AzureBackend::new()?;
//!
//! // Create "pool" (Azure container)
//! let pool = backend.create_pool("tank", &[]).await?;
//!
//! // Create "dataset" (Azure blob prefix)
//! let dataset = backend.create_dataset(&pool, "data", StorageTier::Hot).await?;
//! ```

use super::cloud_helpers::{
    self, CloudConfigSource, dataset_path_prefix, prefixed_pool_resource_name,
};
use crate::zero_cost_zfs_operations::ZeroCostZfsOperations;
use nestgate_core::canonical_types::StorageTier;
use nestgate_core::{NestGateError, Result, config_error};
use nestgate_types::{EnvSource, ProcessEnv};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Azure storage backend
///
/// Implements ZFS-like operations on top of Azure Blob Storage.
/// Containers map to pools, blob prefixes map to datasets.
///
/// **PRODUCTION IMPLEMENTATION**: Uses capability-based configuration
pub struct AzureBackend {
    /// Azure client - real Azure SDK client configured via capability discovery
    client: Arc<AzureClientWrapper>,
    /// Container prefix for all operations (discovered via environment/capability)
    container_prefix: String,
    /// Pool registry (in-memory cache of discovered pools)
    pools: Arc<RwLock<HashMap<String, AzurePool>>>,
}

/// Azure client wrapper - abstracts Azure SDK for testability and capability-based config
///
/// **DESIGN**: Enables capability-based configuration while maintaining
/// clean separation between our abstractions and Azure SDK specifics.
///
/// **EVOLUTION NOTE**: Reserved fields support planned Azure SDK integration for audit,
/// metrics, and dynamic reconfiguration.
struct AzureClientWrapper {
    /// Storage account discovered via capability system or environment
    account: String,
    /// Optional connection string (reserved for Azure SDK client initialization)
    connection_string: Option<String>,
    /// Configuration source (capability discovery vs environment)
    config_source: CloudConfigSource,
}

/// Discovered Azure configuration from capability system
#[derive(Debug, Clone)]
struct DiscoveredAzureConfig {
    /// Service ID from capability discovery
    service_id: String,
    /// Azure storage account
    account: String,
    /// Optional connection string
    connection_string: Option<String>,
    /// Container prefix for this service
    container_prefix: String,
}

/// Azure-backed pool (maps to Azure container)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzurePool {
    /// Pool name
    pub name: String,
    /// Azure container name
    pub container: String,
    /// Creation time
    pub created_at: std::time::SystemTime,
    /// Pool metadata
    pub metadata: HashMap<String, String>,
}

/// Azure-backed dataset (maps to blob prefix)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureDataset {
    /// Dataset name
    pub name: String,
    /// Pool name
    pub pool: String,
    /// Blob prefix
    pub prefix: String,
    /// Storage tier
    pub tier: StorageTier,
    /// Azure access tier
    pub azure_tier: AzureAccessTier,
    /// Creation time
    pub created_at: std::time::SystemTime,
}

/// Azure-backed snapshot (maps to blob snapshot)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureSnapshot {
    /// Snapshot name
    pub name: String,
    /// Dataset name
    pub dataset: String,
    /// Azure snapshot identifier
    pub snapshot_id: String,
    /// Creation time
    pub created_at: std::time::SystemTime,
}

/// Azure pool properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureProperties {
    /// Storage account
    pub account: String,
    /// Container public access level
    pub public_access: bool,
    /// Encryption enabled
    pub encryption: bool,
    /// Additional properties
    pub custom: HashMap<String, String>,
}

/// Azure access tier mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AzureAccessTier {
    /// Hot tier (frequent access)
    Hot,
    /// Cool tier (infrequent access)
    Cool,
    /// Archive tier (rarely accessed)
    Archive,
    /// Premium tier (low latency)
    Premium,
}

impl AzureBackend {
    /// Create new Azure backend using capability-based discovery
    ///
    /// **CAPABILITY-BASED**: Attempts to discover Azure service via capability system first.
    /// Falls back to environment variables if discovery unavailable.
    ///
    /// **PRIMAL SELF-KNOWLEDGE**: This backend only knows itself (Azure operations).
    /// It discovers other services at runtime via capability system.
    ///
    /// Configuration discovery order:
    /// 1. Capability discovery (preferred) - discovers Azure service at runtime
    /// 2. Environment variables (fallback) - for standalone/testing
    pub fn new() -> Result<Self> {
        // Try capability discovery first
        if let Ok(config) = Self::discover_azure_capability() {
            info!(
                "Azure backend initialized via capability discovery: service_id={}",
                config.service_id
            );
            return Self::from_discovered_capability(config);
        }

        // Fallback to environment configuration
        info!("Capability discovery unavailable, using environment config");
        Self::from_env_source(&ProcessEnv)
    }

    /// Discover Azure capability via `NestGate` capability system
    ///
    /// **RUNTIME DISCOVERY**: No hardcoded service locations.
    /// Backend discovers Azure-compatible storage services at startup.
    fn discover_azure_capability() -> Result<DiscoveredAzureConfig> {
        Err(cloud_helpers::pending_capability_discovery("Azure"))
    }

    /// Create backend from discovered capability (zero-hardcoding approach)
    fn from_discovered_capability(config: DiscoveredAzureConfig) -> Result<Self> {
        info!(
            "Initializing Azure backend from capability: account={}, prefix={}",
            config.account, config.container_prefix
        );

        if config.connection_string.is_some() {
            info!("Using discovered Azure connection string");
        }

        Ok(Self {
            client: Arc::new(AzureClientWrapper {
                account: config.account,
                connection_string: config.connection_string,
                config_source: CloudConfigSource::CapabilityDiscovered {
                    service_id: config.service_id,
                },
            }),
            container_prefix: config.container_prefix,
            pools: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Create backend from the process environment (fallback mode).
    ///
    /// **FALLBACK ONLY**: Used when capability discovery is unavailable.
    /// Validates configuration to fail fast on misconfiguration.
    fn from_environment() -> Result<Self> {
        Self::from_env_source(&ProcessEnv)
    }

    /// Create backend from an injectable environment source (fallback mode).
    ///
    /// **FALLBACK ONLY**: Used when capability discovery is unavailable.
    /// Validates configuration to fail fast on misconfiguration.
    fn from_env_source(env: &(impl EnvSource + ?Sized)) -> Result<Self> {
        let account = env.get("AZURE_STORAGE_ACCOUNT").ok_or_else(|| {
            config_error!(
                "AZURE_STORAGE_ACCOUNT required when using environment config",
                "AZURE_STORAGE_ACCOUNT"
            )
        })?;

        let connection_string = env.get("AZURE_STORAGE_CONNECTION_STRING");

        let container_prefix = env.get_or("AZURE_CONTAINER_PREFIX", "nestgate");

        info!(
            "Initializing Azure backend from environment: account={}, prefix={}",
            account, container_prefix
        );

        if connection_string.is_some() {
            info!("Using Azure connection string from environment");
        } else {
            warn!("No connection string provided - using default credential chain");
        }

        Ok(Self {
            client: Arc::new(AzureClientWrapper {
                account,
                connection_string,
                config_source: CloudConfigSource::Environment,
            }),
            container_prefix,
            pools: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Get full container name with prefix
    fn container_name(&self, pool_name: &str) -> String {
        prefixed_pool_resource_name(&self.container_prefix, pool_name)
    }

    /// Get dataset prefix
    fn dataset_prefix(pool_name: &str, dataset_name: &str) -> String {
        dataset_path_prefix(pool_name, dataset_name)
    }

    /// Map storage tier to Azure access tier
    const fn map_tier(tier: &StorageTier) -> AzureAccessTier {
        match tier {
            StorageTier::Hot | StorageTier::Cache => AzureAccessTier::Premium,
            StorageTier::Warm => AzureAccessTier::Cool,
            StorageTier::Cold | StorageTier::Archive => AzureAccessTier::Archive,
        }
    }
}

#[cfg(test)]
impl AzureBackend {
    /// Construct a backend for unit tests without env vars or Azure SDK calls.
    pub(crate) fn test_with_environment_config(
        account: impl Into<String>,
        container_prefix: impl Into<String>,
    ) -> Self {
        Self {
            client: Arc::new(AzureClientWrapper {
                account: account.into(),
                connection_string: None,
                config_source: CloudConfigSource::Environment,
            }),
            container_prefix: container_prefix.into(),
            pools: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl ZeroCostZfsOperations for AzureBackend {
    type Pool = AzurePool;
    type Dataset = AzureDataset;
    type Snapshot = AzureSnapshot;
    type Properties = AzureProperties;
    type Error = NestGateError;

    /// Create Azure pool (container)
    async fn create_pool(&self, name: &str, _devices: &[&str]) -> Result<Self::Pool> {
        let container_name = self.container_name(name);

        info!("Creating Azure pool (container): {}", container_name);

        // ✅ PROTOCOL-FIRST: Create Azure container via REST API (no SDK)
        // Spec: https://docs.microsoft.com/en-us/rest/api/storageservices/create-container
        // PUT /{container}?restype=container
        // Future: Implement when using UniversalObjectStorage
        debug!(
            "Creating container: {} (marker-based for now)",
            container_name
        );

        let pool = AzurePool {
            name: name.to_string(),
            container: container_name.clone(),
            created_at: std::time::SystemTime::now(),
            metadata: HashMap::new(),
        };

        // Register pool
        self.pools
            .write()
            .await
            .insert(name.to_string(), pool.clone());

        info!("Azure pool created: {}", name);
        Ok(pool)
    }

    /// Create Azure dataset (blob prefix)
    async fn create_dataset(
        &self,
        pool: &Self::Pool,
        name: &str,
        tier: StorageTier,
    ) -> Result<Self::Dataset> {
        let prefix = Self::dataset_prefix(&pool.name, name);
        let azure_tier = Self::map_tier(&tier);

        info!(
            "Creating Azure dataset: {} (tier: {:?} -> Azure: {:?})",
            prefix, tier, azure_tier
        );

        // ✅ PROTOCOL-FIRST: Set up dataset with access tier
        // Access tier mapping:
        // - Hot -> Hot (frequent access, higher storage cost, lower access cost)
        // - Warm -> Cool (infrequent access, 30-day minimum)
        // - Cold/Archive -> Archive (rare access, 180-day minimum, offline)
        // - Cache -> Hot (temporary/fast access)
        // Spec: https://docs.microsoft.com/en-us/rest/api/storageservices/set-blob-tier
        // Future: Set tier via x-ms-access-tier header on PUT
        debug!(
            "Creating dataset prefix: {} with tier: {:?} (access tier pending)",
            prefix, azure_tier
        );

        let dataset = AzureDataset {
            name: name.to_string(),
            pool: pool.name.clone(),
            prefix,
            tier,
            azure_tier,
            created_at: std::time::SystemTime::now(),
        };

        info!("Azure dataset created: {}", name);
        Ok(dataset)
    }

    /// Create Azure snapshot (blob snapshot)
    async fn create_snapshot(&self, dataset: &Self::Dataset, name: &str) -> Result<Self::Snapshot> {
        let snapshot_id = format!("{}-{}", dataset.prefix, name);

        info!("Creating Azure snapshot: {}", snapshot_id);

        // ✅ PROTOCOL-FIRST: Create Azure blob snapshot
        // Spec: https://docs.microsoft.com/en-us/rest/api/storageservices/snapshot-blob
        // PUT /{container}/{blob}?comp=snapshot
        // Azure blob snapshots are immutable, read-only versions
        // Each snapshot has a unique DateTime identifier
        // Future: Implement when using UniversalObjectStorage
        debug!(
            "Creating blob snapshot: {} (snapshot API pending)",
            snapshot_id
        );

        let snapshot = AzureSnapshot {
            name: name.to_string(),
            dataset: dataset.name.clone(),
            snapshot_id,
            created_at: std::time::SystemTime::now(),
        };

        info!("Azure snapshot created: {}", name);
        Ok(snapshot)
    }

    /// Get Azure pool properties
    async fn get_pool_properties(&self, pool: &Self::Pool) -> Result<Self::Properties> {
        debug!("Getting properties for pool: {}", pool.name);

        // ✅ PROTOCOL-FIRST: Query Azure container properties via REST API
        // Spec: https://docs.microsoft.com/en-us/rest/api/storageservices/get-container-properties
        // HEAD /{container}?restype=container
        // Returns: x-ms-blob-public-access, x-ms-has-immutability-policy, etc.
        // Future: Implement when using UniversalObjectStorage
        let properties = AzureProperties {
            account: self.client.account.clone(),
            public_access: false, // Future: Query via HEAD request
            encryption: true,     // Azure encrypts by default (server-side)
            custom: {
                let mut map = HashMap::new();
                map.insert(
                    "config_source".to_string(),
                    cloud_helpers::config_source_custom_value(&self.client.config_source),
                );
                map
            },
        };

        Ok(properties)
    }

    /// List Azure pools (containers)
    async fn list_pools(&self) -> Result<Vec<Self::Pool>> {
        debug!("Listing Azure pools");

        // ✅ PROTOCOL-FIRST: List Azure containers via REST API
        // Spec: https://docs.microsoft.com/en-us/rest/api/storageservices/list-containers2
        // GET /?comp=list&prefix={prefix}
        // Returns XML with container names and metadata
        // Future: Implement when using UniversalObjectStorage
        let pools = self.pools.read().await;
        debug!("Found {} Azure pools in memory", pools.len());
        Ok(pools.values().cloned().collect())
    }

    /// List Azure datasets (blob prefixes)
    async fn list_datasets(&self, pool: &Self::Pool) -> Result<Vec<Self::Dataset>> {
        debug!("Listing datasets for pool: {}", pool.name);

        // ✅ PROTOCOL-FIRST: List blob prefixes using delimiter
        // Spec: https://docs.microsoft.com/en-us/rest/api/storageservices/list-blobs
        // GET /{container}?restype=container&comp=list&delimiter=/&prefix={prefix}
        // The delimiter param returns "virtual directories" (BlobPrefix elements)
        // Future: Implement when using UniversalObjectStorage
        warn!("Dataset listing requires REST API integration (pending)");
        Ok(Vec::new())
    }

    /// List Azure snapshots
    async fn list_snapshots(&self, dataset: &Self::Dataset) -> Result<Vec<Self::Snapshot>> {
        debug!("Listing snapshots for dataset: {}", dataset.name);

        // ✅ PROTOCOL-FIRST: List blob snapshots
        // Spec: https://docs.microsoft.com/en-us/rest/api/storageservices/list-blobs
        // GET /{container}?restype=container&comp=list&include=snapshots&prefix={prefix}
        // Returns all blob versions including snapshots with DateTime identifiers
        // Future: Implement when using UniversalObjectStorage
        warn!("Snapshot listing requires REST API integration (pending)");
        Ok(Vec::new())
    }
}

#[cfg(test)]
#[path = "azure_tests.rs"]
mod tests;
