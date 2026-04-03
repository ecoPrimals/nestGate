// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Production auth handler and shared [`AuthManager`] access.

use super::auth_manager::AuthManager;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// **PRODUCTION AUTH HANDLER**
///
/// Manages real authentication using the security module.
#[derive(Debug, Clone)]
/// Handler for ProductionAuth requests
pub struct ProductionAuthHandler {
    manager: Arc<RwLock<AuthManager>>,
}

impl Default for ProductionAuthHandler {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl ProductionAuthHandler {
    /// Create a new production auth handler
    #[must_use]
    pub fn new() -> Self {
        info!("Initializing production auth handler");
        Self {
            manager: Arc::new(RwLock::new(AuthManager::new())),
        }
    }

    /// Get auth manager
    pub(crate) async fn get_manager(&self) -> tokio::sync::RwLockReadGuard<'_, AuthManager> {
        self.manager.read().await
    }

    /// Get mutable auth manager
    pub(crate) async fn get_manager_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, AuthManager> {
        self.manager.write().await
    }
}
