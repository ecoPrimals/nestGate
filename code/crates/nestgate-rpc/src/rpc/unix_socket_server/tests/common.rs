// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use nestgate_config::config::storage_paths::get_storage_base_path;

/// Cleanup a test family's dataset directory after a test.
pub async fn cleanup_family(family_id: &str) {
    let _ =
        tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(family_id)).await;
}
