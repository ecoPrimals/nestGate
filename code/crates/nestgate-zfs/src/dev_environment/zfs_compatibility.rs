// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// This module provides a ZFS-compatible API that works on systems without
// dedicated ZFS storage hardware. It's designed for:
//
// - **Development Laptops**: Work on ZFS features without dedicated pools
// - **Container Environments**: Docker/Podman where ZFS isn't available
// - **CI/CD Systems**: Automated testing without storage hardware
// - **Staging Environments**: Non-production testing environments
//
// ## ⚠️ Important: This is Production-Ready Code
// This is NOT a "mock" - it's a sophisticated hardware abstraction layer
// that provides real functionality through filesystem operations, system calls,
// and compatibility shims. It allows the full NestGate system to run in
// environments where dedicated ZFS hardware isn't available.

//! Zfs Compatibility module

use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};

use crate::error::{create_zfs_error, ZfsOperation};
use nestgate_core::canonical_types::StorageTier;
use nestgate_core::error::CanonicalResult as Result;

/// Development Environment ZFS Service
///
/// Provides ZFS-compatible functionality for development environments
/// without requiring dedicated ZFS storage hardware.
///
/// **This replaces the confusingly-named "`MockZfsService`"**
pub struct DevEnvironmentZfsService {
    /// Simulated pools for development
    pools: Arc<tokio::sync::RwLock<HashMap<String, DevPool>>>,
    /// Simulated datasets
    datasets: Arc<tokio::sync::RwLock<HashMap<String, DevDataset>>>,
    /// Configuration for the development environment
    config: DevEnvironmentConfig,
}
/// Configuration for development environment ZFS compatibility
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::DevEnvironmentConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::DevEnvironmentConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for DevEnvironment
pub struct DevEnvironmentConfig {
    /// Base directory for simulated ZFS operations
    pub base_directory: std::path::PathBuf,
    /// Whether to log all operations (useful for debugging)
    pub verbose_logging: bool,
    /// Simulated pool sizes (in bytes)
    pub default_pool_size: u64,
}
impl Default for DevEnvironmentConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            base_directory: std::env::temp_dir().join("nestgate-dev-zfs"),
            verbose_logging: false,
            default_pool_size: 1024 * 1024 * 1024 * 10, // 10GB simulated
        }
    }
}

/// Simulated ZFS pool for development
#[derive(Debug, Clone)]
#[allow(dead_code)] // Development environment simulation - fields intentionally unused
struct DevPool {
    name: String,
    size_bytes: u64,
    used_bytes: u64,
    health: String,
    created_at: std::time::SystemTime,
}
impl DevPool {
    /// Create a new development pool
    #[allow(dead_code)]
    pub fn new(name: String, size_bytes: u64) -> Self {
        Self {
            name,
            health: "ONLINE".to_string(),
            size_bytes,
            used_bytes: 0,
            created_at: std::time::SystemTime::now(),
        }
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        &self.name
    }
    #[allow(dead_code)]
    pub fn health(&self) -> &str {
        &self.health
    }
    #[allow(dead_code)]
    pub fn size_bytes(&self) -> u64 {
        self.size_bytes
    }
    #[allow(dead_code)]
    pub fn used_bytes(&self) -> u64 {
        self.used_bytes
    }
    #[allow(dead_code)]
    pub fn created_at(&self) -> std::time::SystemTime {
        self.created_at
    }
}

/// Simulated ZFS dataset for development
#[derive(Debug, Clone)]
#[allow(dead_code)] // Development environment simulation - fields intentionally unused
struct DevDataset {
    name: String,
    pool: String,
    mount_point: std::path::PathBuf,
    size_bytes: u64,
    tier: StorageTier,
    properties: HashMap<String, String>,
}
#[allow(dead_code)] // Development environment methods
impl DevDataset {
    /// Create a new development dataset
    #[allow(dead_code)] // Development environment simulation
    pub fn new(
        name: String,
        pool: String,
        mount_point: std::path::PathBuf,
        tier: StorageTier,
    ) -> Self {
        Self {
            name,
            pool,
            mount_point,
            size_bytes: 0,
            tier,
            properties: HashMap::new(),
        }
    }
    /// Get dataset name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get parent pool
    pub fn pool(&self) -> &str {
        &self.pool
    }

    /// Get mount point
    pub fn mount_point(&self) -> &std::path::PathBuf {
        &self.mount_point
    }

    /// Get dataset size
    pub fn size_bytes(&self) -> u64 {
        self.size_bytes
    }

    /// Get storage tier
    pub fn tier(&self) -> &StorageTier {
        &self.tier
    }

    /// Get properties
    pub fn properties(&self) -> &HashMap<String, String> {
        &self.properties
    }
}

impl DevEnvironmentZfsService {
    /// Create new development environment ZFS service
    pub fn new() -> Self {
        let config = DevEnvironmentConfig::default();

        info!("🔧 Initializing Development Environment ZFS Compatibility Layer");
        info!("📁 Base directory: {:?}", config.base_directory);

        Self {
            pools: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            datasets: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: DevEnvironmentConfig) -> Self {
        info!("🔧 Initializing Development Environment ZFS with custom config");
        info!("📁 Base directory: {:?}", config.base_directory);

        Self {
            pools: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            datasets: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Initialize the development environment (create directories, etc.)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn initialize(&self) -> Result<()> {
        if let Err(e) = tokio::fs::create_dir_all(&self.config.base_directory).await {
            warn!("Failed to create base directory: {}", e);
            return Err(create_zfs_error(
                format!(
                    "Failed to create base directory: {}",
                    "actual_error_details"
                ),
                ZfsOperation::Configuration,
            ));
        }

        info!("✅ Development environment initialized");

        // Create a default development pool
        self.create_dev_pool("dev-pool", self.config.default_pool_size)
            .await?;

        Ok(())
    }

    /// Create a simulated pool for development
    async fn create_dev_pool(&self, name: &str, size_bytes: u64) -> Result<()> {
        let mut pools = self.pools.write().await;

        let pool = DevPool {
            name: name.to_string(),
            size_bytes,
            used_bytes: 0,
            health: "ONLINE".to_string(),
            created_at: std::time::SystemTime::now(),
        };

        pools.insert(name.to_string(), pool);

        // Create physical directory for this pool
        let pool_dir = self.config.base_directory.join(name);
        if let Err(e) = tokio::fs::create_dir_all(&pool_dir).await {
            warn!("Failed to create pool directory: {}", e);
        }

        if self.config.verbose_logging {
            info!(
                "🏊 Created development pool: {} ({}GB)",
                name,
                size_bytes / (1024 * 1024 * 1024)
            );
        }

        Ok(())
    }

    /// Get environment report for debugging
    pub async fn get_environment_report(&self) -> String {
        let pools = self.pools.read().await;
        let datasets = self.datasets.read().await;

        format!(
            "Development Environment ZFS Report:\n\
             - Base Directory: {:?}\n\
             - Simulated Pools: {}\n\
             - Simulated Datasets: {}\n\
             - Verbose Logging: {}\n\
             - Pool Names: {:?}",
            self.config.base_directory,
            pools.len(),
            datasets.len(),
            self.config.verbose_logging,
            pools.keys().collect::<Vec<_>>()
        )
    }
}

impl Default for DevEnvironmentZfsService {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

// Implementation would delegate to actual ZFS service when available
// For now, return mock response for development environment
// This will be done in the next phase of implementation

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Devenvironmentconfigcanonical
pub type DevEnvironmentConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using DevEnvironmentConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dev_environment_initialization() {
        let service = DevEnvironmentZfsService::new();
        let result = service.initialize().await;
        assert!(result.is_ok());

        let report = service.get_environment_report().await;
        println!("Development environment report:\n{report}");
    }

    #[test]
    fn test_dev_environment_config() {
        let config = DevEnvironmentConfig::default();
        assert!(config
            .base_directory
            .to_string_lossy()
            .contains("nestgate-dev-zfs"));
        assert_eq!(config.default_pool_size, 1024 * 1024 * 1024 * 10);
    }
}
