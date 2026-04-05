// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! Dataset module — ZFS dataset operations split by concern.

mod create;
mod list;
mod properties;
mod types;
mod validation;

pub use types::{DatasetInfo, ZfsDatasetManager};

#[cfg(test)]
impl ZfsDatasetManager {
    /// Create dataset manager for testing
    ///
    /// **TEST-ONLY**: This constructor is only available in test builds.
    /// Production code must use `ZfsDatasetManager::new()` with proper configuration.
    pub fn new_for_testing() -> Self {
        use crate::config::ZfsConfig;
        use crate::pool::ZfsPoolManager;
        use std::sync::Arc;

        Self {
            config: Arc::new(ZfsConfig::default()),
            pool_manager: Arc::new(ZfsPoolManager::new_for_testing()),
        }
    }
}

#[cfg(test)]
mod tests;
