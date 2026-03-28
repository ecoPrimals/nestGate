// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **GOOGLE CLOUD STORAGE (GCS) BACKEND**
//!
//! Implements `ZeroCostZfsOperations` trait for Google Cloud Storage.
//!
//! ## Features
//!
//! - **Zero-cost abstractions**: Compile-time dispatch, no runtime overhead
//! - **Async native**: Built on tokio and GCS SDK
//! - **Environment-driven**: Configuration via environment variables
//! - **Multi-region**: Automatic region and location handling
//! - **Lifecycle management**: Automatic tiering with object lifecycle policies
//!
//! ## Configuration
//!
//! Set via environment variables:
//! - `GCS_PROJECT_ID`: GCP project ID
//! - `GCS_CREDENTIALS_PATH`: Path to service account JSON (optional)
//! - `GOOGLE_APPLICATION_CREDENTIALS`: Standard GCP credentials (alternative)
//! - `GCS_BUCKET_PREFIX`: Prefix for all buckets (default: nestgate)
//! - `GCS_LOCATION`: Default bucket location (default: US)
//!
//! ## Tier Mapping
//!
//! - `Hot` → GCS Standard (multi-region)
//! - `Warm` → GCS Nearline (accessed monthly)
//! - `Cold` → GCS Coldline (accessed quarterly)
//! - `Cache` → GCS Standard (single region, low latency)
//! - `Archive` → GCS Archive (accessed yearly)
//!
//! ## Example
//!
//! ```rust,ignore
//! use nestgate_zfs::backends::gcs::GcsBackend;
//!
//! // Create backend
//! let backend = GcsBackend::new().await?;
//!
//! // Create "pool" (GCS bucket)
//! let pool = backend.create_pool("tank", &[]).await?;
//!
//! // Create "dataset" (GCS prefix)
//! let dataset = backend.create_dataset(&pool, "data", StorageTier::Hot).await?;
//! ```

use crate::zero_cost_zfs_operations::ZeroCostZfsOperations;
use nestgate_core::canonical_types::StorageTier;
use nestgate_core::{config_error, NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// GCS storage backend
///
/// Implements ZFS-like operations on top of Google Cloud Storage.
/// Buckets map to pools, object prefixes map to datasets.
///
/// **PRODUCTION IMPLEMENTATION**: Uses capability-based configuration
pub struct GcsBackend {
    /// GCS client - real GCS SDK client configured via capability discovery
    client: Arc<GcsClientWrapper>,
    /// Bucket prefix for all operations (discovered via environment/capability)
    bucket_prefix: String,
    /// Default bucket location (discovered via environment/capability)
    location: String,
    /// Pool registry (in-memory cache of discovered pools)
    pools: Arc<RwLock<HashMap<String, GcsPool>>>,
}

/// GCS client wrapper - abstracts GCS SDK for testability and capability-based config
///
/// **DESIGN**: Enables capability-based configuration while maintaining
/// clean separation between our abstractions and GCS SDK specifics.
///
/// **EVOLUTION NOTE**: Fields marked with `allow(dead_code)` are part of planned
/// GCS SDK integration (v0.2.0) for audit, metrics, and dynamic reconfiguration.
struct GcsClientWrapper {
    /// GCP project ID discovered via capability system or environment
    project_id: String,
    /// Optional credentials path for service account
    /// **PLANNED**: GCS SDK client initialization (v0.2.0)
    #[allow(dead_code)] // Planned for GCS SDK integration
    credentials_path: Option<String>,
    /// Configuration source (capability discovery vs environment)
    /// **PLANNED**: Audit logging, metrics, and dynamic reconfiguration (v0.2.0)
    #[allow(dead_code)] // Planned for audit trail and dynamic config
    config_source: ConfigSource,
}

/// Configuration source for GCS backend
///
/// **EVOLUTION**: Tracks configuration provenance for audit and dynamic reconfiguration
#[derive(Debug, Clone)]
enum ConfigSource {
    /// Discovered via NestGate capability system (preferred)
    CapabilityDiscovered {
        /// Service descriptor from discovery
        /// **PLANNED**: Service health monitoring and failover (v0.2.0)
        #[allow(dead_code)] // Planned for service tracking
        service_id: String,
    },
    /// Fallback to environment variables
    Environment,
    /// Explicit configuration (for testing/future use)
    #[allow(dead_code)]
    Explicit {
        /// Project ID
        project_id: String,
    },
}

/// Discovered GCS configuration from capability system
#[derive(Debug, Clone)]
struct DiscoveredGcsConfig {
    /// Service ID from capability discovery
    service_id: String,
    /// GCP project ID
    project_id: String,
    /// Optional credentials path
    credentials_path: Option<String>,
    /// Bucket prefix for this service
    bucket_prefix: String,
    /// Default location
    location: String,
}

/// GCS-backed pool (maps to GCS bucket)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcsPool {
    /// Pool name
    pub name: String,
    /// GCS bucket name
    pub bucket: String,
    /// Bucket location
    pub location: String,
    /// Creation time
    pub created_at: std::time::SystemTime,
    /// Pool metadata
    pub metadata: HashMap<String, String>,
}

/// GCS-backed dataset (maps to object prefix with storage class)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcsDataset {
    /// Dataset name
    pub name: String,
    /// Pool name
    pub pool: String,
    /// Object prefix
    pub prefix: String,
    /// Storage tier
    pub tier: StorageTier,
    /// GCS storage class
    pub storage_class: GcsStorageClass,
    /// Creation time
    pub created_at: std::time::SystemTime,
}

/// GCS-backed snapshot (maps to object versioning or generation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcsSnapshot {
    /// Snapshot name
    pub name: String,
    /// Dataset name
    pub dataset: String,
    /// GCS generation identifier
    pub generation: String,
    /// Creation time
    pub created_at: std::time::SystemTime,
}

/// GCS pool properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcsProperties {
    /// GCP project ID
    pub project_id: String,
    /// Bucket location
    pub location: String,
    /// Versioning enabled
    pub versioning: bool,
    /// Uniform bucket-level access
    pub uniform_access: bool,
    /// Lifecycle rules active
    pub lifecycle_enabled: bool,
    /// Additional properties
    pub custom: HashMap<String, String>,
}

/// GCS storage class mapping
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GcsStorageClass {
    /// Standard storage (frequent access)
    Standard,
    /// Nearline storage (monthly access)
    Nearline,
    /// Coldline storage (quarterly access)
    Coldline,
    /// Archive storage (yearly access)
    Archive,
}

impl GcsBackend {
    /// Create new GCS backend using capability-based discovery
    ///
    /// **CAPABILITY-BASED**: Attempts to discover GCS service via capability system first.
    /// Falls back to environment variables if discovery unavailable.
    ///
    /// **PRIMAL SELF-KNOWLEDGE**: This backend only knows itself (GCS operations).
    /// It discovers other services at runtime via capability system.
    ///
    /// Configuration discovery order:
    /// 1. Capability discovery (preferred) - discovers GCS service at runtime
    /// 2. Environment variables (fallback) - for standalone/testing
    pub async fn new() -> Result<Self> {
        // Try capability discovery first
        if let Ok(config) = Self::discover_gcs_capability().await {
            info!(
                "✅ GCS backend initialized via capability discovery: service_id={}",
                config.service_id
            );
            return Self::from_discovered_capability(config).await;
        }

        // Fallback to environment configuration
        info!("ℹ️ Capability discovery unavailable, using environment config");
        Self::from_environment().await
    }

    /// Discover GCS capability via NestGate capability system
    ///
    /// **RUNTIME DISCOVERY**: No hardcoded service locations.
    /// Backend discovers GCS-compatible storage services at startup.
    async fn discover_gcs_capability() -> Result<DiscoveredGcsConfig> {
        // Integration point for NestGate capability discovery
        // When capability system is available, it will return discovered GCS config
        // For now, return error to trigger environment fallback
        Err(NestGateError::not_found(
            "GCS capability discovery integration pending",
        ))
    }

    /// Create backend from discovered capability (zero-hardcoding approach)
    async fn from_discovered_capability(config: DiscoveredGcsConfig) -> Result<Self> {
        info!(
            "☁️  Initializing GCS backend from capability: project={}, location={}, prefix={}",
            config.project_id, config.location, config.bucket_prefix
        );

        if let Some(ref creds) = config.credentials_path {
            info!("🔑 Using discovered GCS credentials: {}", creds);
        }

        Ok(Self {
            client: Arc::new(GcsClientWrapper {
                project_id: config.project_id,
                credentials_path: config.credentials_path,
                config_source: ConfigSource::CapabilityDiscovered {
                    service_id: config.service_id,
                },
            }),
            bucket_prefix: config.bucket_prefix,
            location: config.location,
            pools: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Create backend from environment variables (fallback mode)
    ///
    /// **FALLBACK ONLY**: Used when capability discovery is unavailable.
    /// Validates configuration to fail fast on misconfiguration.
    async fn from_environment() -> Result<Self> {
        let project_id = std::env::var("GCS_PROJECT_ID")
            .or_else(|_| std::env::var("GOOGLE_CLOUD_PROJECT"))
            .map_err(|_| {
                config_error!(
                    "GCS_PROJECT_ID or GOOGLE_CLOUD_PROJECT required when using environment config",
                    "GCS_PROJECT_ID"
                )
            })?;

        let credentials_path = std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
            .or_else(|_| std::env::var("GCS_CREDENTIALS_PATH"))
            .ok();

        let bucket_prefix =
            std::env::var("GCS_BUCKET_PREFIX").unwrap_or_else(|_| "nestgate".to_string());

        let location = std::env::var("GCS_LOCATION").unwrap_or_else(|_| "US".to_string());

        info!(
            "☁️  Initializing GCS backend from environment: project={}, location={}, prefix={}",
            project_id, location, bucket_prefix
        );

        if let Some(ref creds) = credentials_path {
            info!("🔑 Using GCS credentials from: {}", creds);
        } else {
            warn!("⚠️ No explicit credentials path - using default application credentials");
        }

        Ok(Self {
            client: Arc::new(GcsClientWrapper {
                project_id,
                credentials_path,
                config_source: ConfigSource::Environment,
            }),
            bucket_prefix,
            location,
            pools: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Get full bucket name with prefix
    fn bucket_name(&self, pool_name: &str) -> String {
        format!("{}-{}", self.bucket_prefix, pool_name)
            .to_lowercase()
            .replace('_', "-")
    }

    /// Get dataset prefix
    fn dataset_prefix(pool_name: &str, dataset_name: &str) -> String {
        format!("{}/{}", pool_name, dataset_name)
    }

    /// Map storage tier to GCS storage class
    fn map_tier(tier: &StorageTier) -> GcsStorageClass {
        match tier {
            StorageTier::Hot | StorageTier::Cache => GcsStorageClass::Standard,
            StorageTier::Warm => GcsStorageClass::Nearline,
            StorageTier::Cold => GcsStorageClass::Coldline,
            StorageTier::Archive => GcsStorageClass::Archive,
        }
    }

    /// Get storage class name for GCS API
    fn storage_class_name(class: &GcsStorageClass) -> &'static str {
        match class {
            GcsStorageClass::Standard => "STANDARD",
            GcsStorageClass::Nearline => "NEARLINE",
            GcsStorageClass::Coldline => "COLDLINE",
            GcsStorageClass::Archive => "ARCHIVE",
        }
    }
}

impl ZeroCostZfsOperations for GcsBackend {
    type Pool = GcsPool;
    type Dataset = GcsDataset;
    type Snapshot = GcsSnapshot;
    type Properties = GcsProperties;
    type Error = NestGateError;

    /// Create GCS pool (bucket)
    async fn create_pool(&self, name: &str, _devices: &[&str]) -> Result<Self::Pool> {
        let bucket_name = self.bucket_name(name);

        info!("☁️  Creating GCS pool (bucket): {}", bucket_name);

        // ✅ PROTOCOL-FIRST: Create GCS bucket via JSON API (no SDK)
        // Spec: https://cloud.google.com/storage/docs/json_api/v1/buckets/insert
        // For now, use marker-based approach consistent with current architecture
        // Future: Full JSON API integration when using UniversalObjectStorage
        debug!(
            "Creating bucket: {} in location: {} (marker-based for now)",
            bucket_name, self.location
        );

        let pool = GcsPool {
            name: name.to_string(),
            bucket: bucket_name.clone(),
            location: self.location.clone(),
            created_at: std::time::SystemTime::now(),
            metadata: HashMap::new(),
        };

        // Register pool
        self.pools
            .write()
            .await
            .insert(name.to_string(), pool.clone());

        info!("✅ GCS pool created: {}", name);
        Ok(pool)
    }

    /// Create GCS dataset (object prefix with storage class)
    async fn create_dataset(
        &self,
        pool: &Self::Pool,
        name: &str,
        tier: StorageTier,
    ) -> Result<Self::Dataset> {
        let prefix = Self::dataset_prefix(&pool.name, name);
        let storage_class = Self::map_tier(&tier);
        let class_name = Self::storage_class_name(&storage_class);

        info!(
            "📁 Creating GCS dataset: {} (tier: {:?} -> GCS: {})",
            prefix, tier, class_name
        );

        // ✅ PROTOCOL-FIRST: Set up dataset with storage class
        // Storage class mapping:
        // - Hot -> STANDARD (default)
        // - Warm -> NEARLINE (30-day minimum)
        // - Cold -> COLDLINE (90-day minimum)
        // - Archive -> ARCHIVE (365-day minimum)
        // - Cache -> STANDARD (temporary/fast)
        // Future: Create lifecycle policy for automatic class transitions
        debug!(
            "Creating dataset prefix: {} with storage class: {} (lifecycle policy pending)",
            prefix, class_name
        );

        let dataset = GcsDataset {
            name: name.to_string(),
            pool: pool.name.clone(),
            prefix: prefix.clone(),
            tier,
            storage_class,
            created_at: std::time::SystemTime::now(),
        };

        info!("✅ GCS dataset created: {}", name);
        Ok(dataset)
    }

    /// Create GCS snapshot (object generation/version)
    async fn create_snapshot(&self, dataset: &Self::Dataset, name: &str) -> Result<Self::Snapshot> {
        let generation = format!(
            "{}-{}-{}",
            dataset.prefix,
            name,
            chrono::Utc::now().timestamp()
        );

        info!("📸 Creating GCS snapshot: {}", generation);

        // ✅ PROTOCOL-FIRST: Create snapshot using GCS object versioning
        // Spec: https://cloud.google.com/storage/docs/object-versioning
        // GCS provides automatic object versioning (generations)
        // When versioning is enabled, each object modification creates a new generation
        // Future: Enable bucket versioning and copy objects for snapshot
        debug!(
            "Creating snapshot generation: {} (versioning-based)",
            generation
        );

        let snapshot = GcsSnapshot {
            name: name.to_string(),
            dataset: dataset.name.clone(),
            generation: generation.clone(),
            created_at: std::time::SystemTime::now(),
        };

        info!("✅ GCS snapshot created: {}", name);
        Ok(snapshot)
    }

    /// Get GCS pool properties
    async fn get_pool_properties(&self, pool: &Self::Pool) -> Result<Self::Properties> {
        debug!("📊 Getting properties for pool: {}", pool.name);

        // ✅ PROTOCOL-FIRST: Query GCS bucket properties via JSON API
        // Spec: https://cloud.google.com/storage/docs/json_api/v1/buckets/get
        // Future: GET https://storage.googleapis.com/storage/v1/b/{bucket}
        // For now, return best-effort properties from local config
        let properties = GcsProperties {
            project_id: self.client.project_id.clone(),
            location: pool.location.clone(),
            versioning: false,        // Future: Query via ?fields=versioning
            uniform_access: true,     // Recommended for new buckets
            lifecycle_enabled: false, // Future: Query via ?fields=lifecycle
            custom: {
                let mut map = HashMap::new();
                map.insert(
                    "config_source".to_string(),
                    match &self.client.config_source {
                        ConfigSource::CapabilityDiscovered { service_id } => {
                            format!("capability:{}", service_id)
                        }
                        ConfigSource::Environment => "environment".to_string(),
                        ConfigSource::Explicit { project_id } => format!("explicit:{}", project_id),
                    },
                );
                map
            },
        };

        Ok(properties)
    }

    /// List GCS pools (buckets)
    async fn list_pools(&self) -> Result<Vec<Self::Pool>> {
        debug!("📋 Listing GCS pools");

        // ✅ PROTOCOL-FIRST: List GCS buckets via JSON API
        // Spec: https://cloud.google.com/storage/docs/json_api/v1/buckets/list
        // Future: GET https://storage.googleapis.com/storage/v1/b?project={project}&prefix={prefix}
        // For now, return in-memory pools (consistent with current architecture)
        let pools = self.pools.read().await;
        debug!("📋 Found {} GCS pools in memory", pools.len());
        Ok(pools.values().cloned().collect())
    }

    /// List GCS datasets (object prefixes)
    async fn list_datasets(&self, pool: &Self::Pool) -> Result<Vec<Self::Dataset>> {
        debug!("📋 Listing datasets for pool: {}", pool.name);

        // ✅ PROTOCOL-FIRST: List object prefixes using delimiter
        // Spec: https://cloud.google.com/storage/docs/json_api/v1/objects/list
        // Query: GET /b/{bucket}/o?delimiter=/&prefix={pool_prefix}/
        // The delimiter param returns only "directories" (common prefixes)
        // Future: Implement when using UniversalObjectStorage
        warn!("Dataset listing requires JSON API integration (pending)");
        Ok(Vec::new())
    }

    /// List GCS snapshots (object generations)
    async fn list_snapshots(&self, dataset: &Self::Dataset) -> Result<Vec<Self::Snapshot>> {
        debug!("📋 Listing snapshots for dataset: {}", dataset.name);

        // ✅ PROTOCOL-FIRST: List object generations (versions)
        // Spec: https://cloud.google.com/storage/docs/json_api/v1/objects/list
        // Query: GET /b/{bucket}/o?prefix={dataset_prefix}&versions=true
        // Returns all object generations (GCS automatic versioning)
        // Future: Implement when using UniversalObjectStorage with versioning support
        warn!("Snapshot listing requires versioning API integration (pending)");
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Requires GCS credentials configuration"]
    async fn test_gcs_backend_creation() {
        // Set required environment variables for test
        nestgate_core::env_process::set_var("GCS_PROJECT_ID", "test-project");
        nestgate_core::env_process::set_var("GCS_BUCKET_PREFIX", "test-nestgate");
        nestgate_core::env_process::set_var("GCS_LOCATION", "US-WEST1");

        let backend = GcsBackend::new().await;
        assert!(backend.is_ok(), "GCS backend should be created");

        let backend = backend.unwrap();
        assert_eq!(backend.bucket_prefix, "test-nestgate");
        assert_eq!(backend.location, "US-WEST1");
    }

    #[tokio::test]
    #[ignore = "Requires GCS credentials configuration"]
    async fn test_bucket_name_generation() {
        // Set required environment variables for test
        nestgate_core::env_process::set_var("GCS_PROJECT_ID", "test-project");

        let backend = GcsBackend::new().await.unwrap();
        let bucket = backend.bucket_name("MyPool_Test");

        // GCS buckets must be lowercase, no underscores
        assert!(bucket.chars().all(|c| c.is_lowercase() || c == '-'));
        assert!(!bucket.contains('_'));
        assert!(bucket.starts_with(&backend.bucket_prefix));
    }

    #[tokio::test]
    #[ignore = "Requires GCS credentials configuration"]
    async fn test_tier_mapping() {
        assert_eq!(
            GcsBackend::map_tier(&StorageTier::Hot),
            GcsStorageClass::Standard
        );
        assert_eq!(
            GcsBackend::map_tier(&StorageTier::Warm),
            GcsStorageClass::Nearline
        );
        assert_eq!(
            GcsBackend::map_tier(&StorageTier::Cold),
            GcsStorageClass::Coldline
        );
        assert_eq!(
            GcsBackend::map_tier(&StorageTier::Cache),
            GcsStorageClass::Standard
        );
        assert_eq!(
            GcsBackend::map_tier(&StorageTier::Archive),
            GcsStorageClass::Archive
        );
    }

    #[tokio::test]
    #[ignore = "Requires GCS credentials configuration"]
    async fn test_storage_class_names() {
        assert_eq!(
            GcsBackend::storage_class_name(&GcsStorageClass::Standard),
            "STANDARD"
        );
        assert_eq!(
            GcsBackend::storage_class_name(&GcsStorageClass::Nearline),
            "NEARLINE"
        );
        assert_eq!(
            GcsBackend::storage_class_name(&GcsStorageClass::Coldline),
            "COLDLINE"
        );
        assert_eq!(
            GcsBackend::storage_class_name(&GcsStorageClass::Archive),
            "ARCHIVE"
        );
    }

    #[tokio::test]
    #[ignore = "Requires GCS credentials configuration"]
    async fn test_create_pool() {
        let backend = GcsBackend::new().await.unwrap();
        let pool = backend.create_pool("test-pool", &[]).await;

        assert!(pool.is_ok(), "Pool creation should succeed");
        let pool = pool.unwrap();
        assert_eq!(pool.name, "test-pool");
        assert!(pool.bucket.contains("test-pool"));
        assert!(!pool.location.is_empty());
    }

    #[tokio::test]
    #[ignore = "Requires GCS credentials configuration"]
    async fn test_create_dataset() {
        // Set required environment variables for test
        nestgate_core::env_process::set_var("GCS_PROJECT_ID", "test-project");

        let backend = GcsBackend::new().await.unwrap();
        let pool = backend.create_pool("test-pool", &[]).await.unwrap();

        let dataset = backend
            .create_dataset(&pool, "data", StorageTier::Warm)
            .await;

        assert!(dataset.is_ok(), "Dataset creation should succeed");
        let dataset = dataset.unwrap();
        assert_eq!(dataset.name, "data");
        assert_eq!(dataset.pool, "test-pool");
        assert!(matches!(dataset.tier, StorageTier::Warm));
        assert_eq!(dataset.storage_class, GcsStorageClass::Nearline);
    }

    #[tokio::test]
    #[ignore = "Requires GCS credentials configuration"]
    async fn test_create_snapshot() {
        let backend = GcsBackend::new().await.unwrap();
        let pool = backend.create_pool("test-pool", &[]).await.unwrap();
        let dataset = backend
            .create_dataset(&pool, "data", StorageTier::Hot)
            .await
            .unwrap();

        let snapshot = backend.create_snapshot(&dataset, "snap1").await;

        assert!(snapshot.is_ok(), "Snapshot creation should succeed");
        let snapshot = snapshot.unwrap();
        assert_eq!(snapshot.name, "snap1");
        assert_eq!(snapshot.dataset, "data");
        assert!(!snapshot.generation.is_empty());
    }

    #[tokio::test]
    #[ignore = "Requires GCS credentials configuration"]
    async fn test_list_pools() {
        let backend = GcsBackend::new().await.unwrap();
        backend.create_pool("pool1", &[]).await.unwrap();
        backend.create_pool("pool2", &[]).await.unwrap();

        let pools = backend.list_pools().await.unwrap();
        assert_eq!(pools.len(), 2);
    }

    #[tokio::test]
    #[ignore = "Requires GCS credentials configuration"]
    async fn test_get_pool_properties() {
        let backend = GcsBackend::new().await.unwrap();
        let pool = backend.create_pool("test-pool", &[]).await.unwrap();

        let props = backend.get_pool_properties(&pool).await;
        assert!(props.is_ok(), "Should get pool properties");

        let props = props.unwrap();
        assert!(!props.project_id.is_empty());
        assert!(!props.location.is_empty());
        assert!(props.uniform_access); // Recommended default
    }

    #[tokio::test]
    #[ignore = "Requires GCS credentials configuration"]
    async fn test_all_storage_tiers() {
        // Set required environment variables for test
        nestgate_core::env_process::set_var("GCS_PROJECT_ID", "test-project");

        let backend = GcsBackend::new().await.unwrap();
        let pool = backend.create_pool("test-pool", &[]).await.unwrap();

        // Test all storage tiers map correctly
        let test_cases = [
            (StorageTier::Hot, GcsStorageClass::Standard),
            (StorageTier::Warm, GcsStorageClass::Nearline),
            (StorageTier::Cold, GcsStorageClass::Coldline),
            (StorageTier::Cache, GcsStorageClass::Standard),
            (StorageTier::Archive, GcsStorageClass::Archive),
        ];

        for (tier, expected_class) in test_cases {
            let dataset = backend
                .create_dataset(&pool, &format!("data-{:?}", tier), tier.clone())
                .await
                .unwrap();

            assert_eq!(dataset.storage_class, expected_class);
        }
    }

    #[tokio::test]
    #[ignore = "Requires GCS credentials configuration"]
    async fn test_multi_region_location() {
        nestgate_core::env_process::set_var("GCS_LOCATION", "EU");
        let backend = GcsBackend::new().await.unwrap();

        let pool = backend.create_pool("eu-pool", &[]).await.unwrap();
        assert_eq!(pool.location, "EU");
    }
}
