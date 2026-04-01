// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Access to the shared [`ProductionZfsManager`] from application state.

use crate::dev_stubs::zfs::ProductionZfsManager;
use crate::routes::AppState;
use std::sync::Arc;

/// Get ZFS service instance using zero-cost operations
pub async fn get_zfs_service(state: &AppState) -> Result<Arc<ProductionZfsManager>, String> {
    state
        .get_zfs_manager()
        .ok_or_else(|| "ZFS service not available".to_string())
}
