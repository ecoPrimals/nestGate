// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]
//
// Contains configuration and utility operations for the native ZFS backend.

// Removed unused tracing import

//! Configuration module

use crate::handlers::zfs::universal_zfs_types::{UniversalZfsError, UniversalZfsResult};

use super::core::NativeZfsService;
use tracing::info;
use tracing::warn;

/// Get service configuration
pub async fn get_configuration(
    service: &NativeZfsService,
) -> UniversalZfsResult<serde_json::Value> {
    Ok(serde_json::json!({
        "service_name": service.service_name,
        "service_version": service.service_version,
        "backend": "native",
        "zfs_available": NativeZfsService::is_available().await
    }))
}
/// Update service configuration
pub fn update_configuration(
    _service: &NativeZfsService,
    config: serde_json::Value,
) -> UniversalZfsResult<()> {
    info!("Updating native ZFS configuration");
    // Parse configuration updates
    if let Ok(config_map) =
        serde_json::from_value::<std::collections::HashMap<String, serde_json::Value>>(config)
    {
        for (key, value) in config_map {
            match key.as_str() {
                "compression" => {
                    if let Some(compression) = value.as_str() {
                        info!("Setting compression to: {}", compression);
                        // Would update ZFS compression settings via zfs set command
                    }
                }
                "deduplication" => {
                    if let Some(dedup) = value.as_str() {
                        info!("Setting deduplication to: {}", dedup);
                        // Would update ZFS deduplication settings
                    }
                }
                "recordsize" => {
                    if let Some(recordsize) = value.as_str() {
                        info!("Setting recordsize to: {}", recordsize);
                        // Would update ZFS recordsize property
                    }
                }
                _ => {
                    warn!("Unknown configuration key: {}", key);
                }
            }
        }
        Ok(())
    } else {
        Err(UniversalZfsError::invalid_input(
            "Invalid configuration format - expected JSON object",
        ))
    }
}

/// Shutdown the service
pub fn shutdown(_service: &NativeZfsService) -> UniversalZfsResult<()> {
    info!("Shutting down native ZFS service");
    Ok(())
}
