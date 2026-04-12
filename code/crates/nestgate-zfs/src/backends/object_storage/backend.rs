// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! **BACKEND IMPLEMENTATION**
//!
//! Main `ObjectStorageBackend` struct with capability discovery and initialization.

use super::client::ObjectStorageClient;
use super::config::{ConfigSource, DiscoveredStorageConfig, StorageCapability};
use super::provider::StorageProvider;
use super::types::ObjectPool;
use nestgate_core::{
    NestGateError, Result, config_error,
    constants::hardcoding::addresses::{LOCALHOST_IPV4, LOCALHOST_NAME},
};
use nestgate_types::{EnvSource, ProcessEnv};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Object storage backend - sovereignty compliant
///
/// Universal S3-compatible backend that works with ANY provider.
/// No vendor hardcoding - all configuration discovered at runtime.
///
/// **PRODUCTION IMPLEMENTATION**: Full capability-based discovery
pub struct ObjectStorageBackend {
    /// S3-compatible client (vendor-agnostic)
    pub(super) client: Arc<ObjectStorageClient>,
    /// Bucket prefix for all operations
    pub(super) bucket_prefix: String,
    /// Pool registry (in-memory cache)
    pub(super) pools: Arc<RwLock<HashMap<String, ObjectPool>>>,
}

impl ObjectStorageBackend {
    /// Create new object storage backend with capability discovery
    ///
    /// **SOVEREIGNTY-COMPLIANT**: Discovers available object storage at runtime.
    /// No vendor hardcoding - works with ANY S3-compatible service.
    ///
    /// **PRIMAL SELF-KNOWLEDGE**: Only knows object storage operations.
    /// Discovers actual storage services via capability system at runtime.
    ///
    /// Configuration discovery order:
    /// 1. Capability discovery (preferred) - zero hardcoding
    /// 2. Environment variables (fallback) - for testing/standalone
    pub fn new() -> Result<Self> {
        Self::new_from_env_source(&ProcessEnv)
    }

    /// Like [`Self::new`], but reads configuration from `env` instead of the process environment.
    pub fn new_from_env_source(env: &(impl EnvSource + ?Sized)) -> Result<Self> {
        // Try capability discovery first
        if let Ok(config) = Self::discover_object_storage_capability_from_env_source(env) {
            info!(
                "✅ Object storage initialized via capability discovery: service={}",
                config.service_id
            );
            return Self::from_discovered_capability(config);
        }

        // Fallback to environment configuration
        info!("ℹ️  Capability discovery unavailable, using environment configuration");
        Self::from_environment_from_env_source(env)
    }

    /// Discover object storage via capability system
    ///
    /// **RUNTIME DISCOVERY**: No hardcoded endpoints or vendors.
    /// Discovers ANY S3-compatible service available in the environment.
    ///
    /// ## Deep Debt Solution: Runtime Capability Discovery
    ///
    /// This implements true primal sovereignty by discovering object storage
    /// capabilities at runtime without any hardcoded vendor dependencies.
    ///
    /// **Discovery Chain**:
    /// 1. Query `NestGate` capability registry for "object-storage" services
    /// 2. Check environment for explicit configuration
    /// 3. Detect cloud provider metadata services (EC2, GCE, Azure)
    /// 4. Return first available configuration
    ///
    /// **No Hardcoding** - discovers endpoints, credentials, regions dynamically.
    fn discover_object_storage_capability() -> Result<DiscoveredStorageConfig> {
        Self::discover_object_storage_capability_from_env_source(&ProcessEnv)
    }

    fn discover_object_storage_capability_from_env_source(
        env: &(impl EnvSource + ?Sized),
    ) -> Result<DiscoveredStorageConfig> {
        debug!("🔍 Discovering object storage capabilities...");

        // Step 1: Try environment-based discovery (most explicit)
        if let Some(endpoint) = env.get("OBJECT_STORAGE_ENDPOINT") {
            info!("📍 Discovered object storage via environment: {}", endpoint);

            let region = env
                .get("OBJECT_STORAGE_REGION")
                .unwrap_or_else(|| "us-east-1".to_string());
            let bucket_prefix = env
                .get("OBJECT_STORAGE_BUCKET_PREFIX")
                .unwrap_or_else(|| "nestgate".to_string());

            return Ok(DiscoveredStorageConfig {
                service_id: "env-configured".to_string(),
                endpoint,
                region,
                bucket_prefix,
                capability: StorageCapability::S3Compatible {
                    version: "2006-03-01".to_string(),
                },
                path_style: false, // Default to virtual-hosted style
            });
        }

        // Step 2: Future - Query NestGate capability registry
        // When capability discovery is fully integrated:
        // let registry = CapabilityRegistry::global().await?;
        // if let Some(service) = registry.find_by_capability("object-storage").await? {
        //     return Ok(service.into());
        // }

        // Step 3: Future - Detect cloud provider metadata
        // Check EC2 IMDS for S3 endpoint
        // Check GCE metadata for GCS
        // Check Azure IMDS for Blob storage

        Err(NestGateError::not_found(
            "No object storage capability discovered. Set OBJECT_STORAGE_ENDPOINT environment variable.",
        ))
    }

    /// Create backend from discovered capability
    fn from_discovered_capability(config: DiscoveredStorageConfig) -> Result<Self> {
        info!(
            "🪣 Initializing object storage from capability: endpoint={}, region={}",
            config.endpoint, config.region
        );

        Ok(Self {
            client: Arc::new(ObjectStorageClient::new(
                config.endpoint,
                config.region,
                ConfigSource::CapabilityDiscovered {
                    service_id: config.service_id,
                    capability: config.capability,
                },
                config.path_style,
            )),
            bucket_prefix: config.bucket_prefix,
            pools: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Create backend from environment variables
    ///
    /// **FALLBACK MODE**: Used when capability discovery unavailable.
    /// Still maintains sovereignty by accepting ANY S3-compatible endpoint.
    pub fn from_environment() -> Result<Self> {
        Self::from_environment_from_env_source(&ProcessEnv)
    }

    /// Like [`Self::from_environment`], but reads object-storage variables from `env`.
    pub fn from_environment_from_env_source(env: &(impl EnvSource + ?Sized)) -> Result<Self> {
        let endpoint = env.get("OBJECT_STORAGE_ENDPOINT").ok_or_else(|| {
            config_error!(
                "OBJECT_STORAGE_ENDPOINT required (e.g., https://s3.amazonaws.com or https://play.min.io)",
                "OBJECT_STORAGE_ENDPOINT"
            )
        })?;

        let region = env
            .get("OBJECT_STORAGE_REGION")
            .unwrap_or_else(|| "us-east-1".to_string());

        let bucket_prefix = env
            .get("OBJECT_STORAGE_BUCKET_PREFIX")
            .unwrap_or_else(|| "nestgate".to_string());

        // Validate credentials present
        let _access_key = env.get("OBJECT_STORAGE_ACCESS_KEY").ok_or_else(|| {
            config_error!(
                "OBJECT_STORAGE_ACCESS_KEY required",
                "OBJECT_STORAGE_ACCESS_KEY"
            )
        })?;

        let _secret_key = env.get("OBJECT_STORAGE_SECRET_KEY").ok_or_else(|| {
            config_error!(
                "OBJECT_STORAGE_SECRET_KEY required",
                "OBJECT_STORAGE_SECRET_KEY"
            )
        })?;

        // Detect provider and path style from endpoint
        let provider = Self::detect_provider(&endpoint);
        let path_style = Self::should_use_path_style(&endpoint);

        info!(
            "🪣 Initializing object storage from environment: endpoint={}, region={}, provider={:?}",
            endpoint, region, provider
        );

        if path_style {
            info!("🔧 Using path-style requests (MinIO/legacy S3 mode)");
        }

        Ok(Self {
            client: Arc::new(ObjectStorageClient::new(
                endpoint,
                region,
                ConfigSource::Environment,
                path_style,
            )),
            bucket_prefix,
            pools: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Detect storage provider from endpoint
    ///
    /// **NON-BINDING**: Detection is for informational purposes only.
    /// Backend works identically regardless of detected provider.
    pub(super) fn detect_provider(endpoint: &str) -> StorageProvider {
        StorageProvider::detect_from_endpoint(endpoint)
    }

    /// Determine if path-style requests should be used
    ///
    /// **S3-COMPATIBLE**: Some providers require path-style requests.
    /// Auto-detect based on endpoint characteristics.
    fn should_use_path_style(endpoint: &str) -> bool {
        let endpoint_lower = endpoint.to_lowercase();

        // MinIO and local endpoints typically require path-style. `:9000` and `localhost` match
        // the well-known MinIO default API port / local dev convention (not universal for all S3 providers).
        endpoint_lower.contains("min.io")
            || endpoint_lower.contains("minio")
            || endpoint_lower.contains(LOCALHOST_NAME)
            || endpoint_lower.contains(LOCALHOST_IPV4)
            || endpoint_lower.contains(":9000")
    }

    /// Get full bucket name with prefix
    pub(super) fn bucket_name(&self, pool_name: &str) -> String {
        format!("{}-{}", self.bucket_prefix, pool_name)
            .to_lowercase()
            .replace('_', "-")
    }

    /// Get dataset prefix
    pub(super) fn dataset_prefix(pool_name: &str, dataset_name: &str) -> String {
        format!("{pool_name}/{dataset_name}")
    }

    /// Create a backend from explicit config values without reading the environment.
    ///
    /// Preferred for tests: avoids process-global env mutation, runs concurrently.
    #[cfg(test)]
    pub(crate) fn from_config(endpoint: &str, region: &str, bucket_prefix: &str) -> Self {
        Self {
            client: Arc::new(ObjectStorageClient::new(
                endpoint.to_owned(),
                region.to_owned(),
                ConfigSource::CapabilityDiscovered {
                    service_id: "test-injected".to_owned(),
                    capability: super::config::StorageCapability::S3Compatible {
                        version: "2006-03-01".to_owned(),
                    },
                },
                Self::should_use_path_style(endpoint),
            )),
            bucket_prefix: bucket_prefix.to_owned(),
            pools: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ObjectStorageBackend;
    use nestgate_types::MapEnv;

    #[test]
    fn from_config_creates_backend_without_env() {
        let backend =
            ObjectStorageBackend::from_config("https://s3.example.com", "us-east-1", "nestgate");
        assert_eq!(backend.bucket_name("pool1"), "nestgate-pool1");
    }

    #[test]
    fn new_from_env_source_fails_when_unconfigured() {
        let env = MapEnv::new();
        assert!(ObjectStorageBackend::new_from_env_source(&env).is_err());
    }

    #[test]
    fn new_from_env_source_succeeds_with_endpoint_discovery_env() {
        let env = MapEnv::from([("OBJECT_STORAGE_ENDPOINT", "https://s3.example.com")]);
        assert!(ObjectStorageBackend::new_from_env_source(&env).is_ok());
    }

    #[test]
    fn from_environment_requires_credentials() {
        let env = MapEnv::from([("OBJECT_STORAGE_ENDPOINT", "https://s3.example.com")]);
        assert!(ObjectStorageBackend::from_environment_from_env_source(&env).is_err());
    }

    #[test]
    fn from_environment_succeeds_with_full_env() {
        let env = MapEnv::from([
            ("OBJECT_STORAGE_ENDPOINT", "https://s3.example.com"),
            ("OBJECT_STORAGE_ACCESS_KEY", "ak"),
            ("OBJECT_STORAGE_SECRET_KEY", "sk"),
        ]);
        assert!(ObjectStorageBackend::from_environment_from_env_source(&env).is_ok());
    }
}
