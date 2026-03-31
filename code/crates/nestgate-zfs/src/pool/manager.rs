// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! ZFS Pool Manager — core type and constructors.
//!
//! Pool discovery, status queries, and mutating operations live in sibling
//! modules (`discovery`, `status`, `operations`).

use tracing::{info, warn};

use crate::config::ZfsConfig;
use crate::error::Result;
use crate::pool::types::PoolInfo;

/// ZFS Pool Manager - handles pool operations and management
#[derive(Debug, Clone)]
/// Manager for `ZfsPool` operations
pub struct ZfsPoolManager {
    pub(crate) config: ZfsConfig,
    /// In-memory cache of discovered pools with automatic persistence
    pub(crate) discovered_pools:
        std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, PoolInfo>>>,
}

impl ZfsPoolManager {
    /// Create a new ZFS pool manager (async)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn new(config: &ZfsConfig) -> Result<Self> {
        info!("Initializing ZFS pool manager");

        let manager = Self {
            config: config.clone(),
            discovered_pools: std::sync::Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::new(),
            )),
        };

        // Test ZFS availability
        if !crate::native::is_zfs_available().await {
            warn!(
                "ZFS not available; pool operations will fail until ZFS kernel and userland tools are present"
            );
        }
        Ok(manager)
    }

    /// Create a new ZFS pool manager with owned config (zero-copy)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn with_owned_config(config: ZfsConfig) -> Result<Self> {
        info!("Initializing ZFS pool manager with owned config");

        let manager = Self {
            config,
            discovered_pools: std::sync::Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::new(),
            )),
        };

        // Test ZFS availability
        if !crate::native::is_zfs_available().await {
            warn!(
                "ZFS not available; pool operations will fail until ZFS kernel and userland tools are present"
            );
        }
        Ok(manager)
    }

    /// Create instance for real production use
    ///
    /// This is the primary constructor for production code. Use this instead of
    /// `new()` when you have a synchronous context and need immediate initialization.
    #[must_use]
    pub fn new_production(config: ZfsConfig) -> Self {
        Self {
            config,
            discovered_pools: std::sync::Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::new(),
            )),
        }
    }
}

// ========== TEST-ONLY CONSTRUCTORS ==========
// Isolated from production code to maintain clear boundaries

#[cfg(test)]
impl ZfsPoolManager {
    /// Create instance for testing with default configuration
    ///
    /// **TEST-ONLY**: This constructor is only available in test builds.
    /// Production code must use `ZfsPoolManager::new()` or `new_production()`.
    pub fn new_for_testing() -> Self {
        Self {
            config: ZfsConfig::default(),
            discovered_pools: std::sync::Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::new(),
            )),
        }
    }

    /// Insert a pool into the in-memory cache (exercises `list_pools`, `get_pool_info`, `get_overall_status`).
    pub async fn insert_pool_for_testing(&self, info: PoolInfo) {
        let mut pools = self.discovered_pools.write().await;
        pools.insert(info.name.clone(), info);
    }
}

#[cfg(test)]
mod tests {
    use crate::config::ZfsConfig;

    use super::*;

    #[test]
    fn new_production_uses_config_and_starts_empty() {
        let cfg = ZfsConfig::default();
        let m = ZfsPoolManager::new_production(cfg);
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("test runtime");
        rt.block_on(async {
            let pools = m.list_pools().await.expect("test: list_pools");
            assert!(pools.is_empty());
        });
    }

    #[tokio::test]
    async fn new_and_with_owned_config_initialize() {
        let cfg = ZfsConfig::default();
        let a = ZfsPoolManager::new(&cfg).await.expect("test: new");
        let _ = a.list_pools().await.expect("test: list after new");

        let b = ZfsPoolManager::with_owned_config(cfg)
            .await
            .expect("test: with_owned");
        let _ = b.list_pools().await.expect("test: list after with_owned");
    }
}
