// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS configuration and optimization stubs.
//!
//! These functions return explicit errors until the corresponding ZFS
//! tuning/analytics features are wired to real `zfs set`/`zfs get` calls.

use super::core::NativeZfsService;
use crate::handlers::zfs::universal_zfs_types::{UniversalZfsError, UniversalZfsResult};
use std::collections::HashMap;

/// Optimize ZFS configuration for better performance.
///
/// Returns an error until real ZFS tuning is wired.
pub fn optimize(
    _service: &NativeZfsService,
    _optimization_type: String,
) -> UniversalZfsResult<String> {
    Err(UniversalZfsError::ServiceUnavailable {
        message: "ZFS optimization not yet wired to real zfs set/get calls".into(),
    })
}

/// Get analytics data for ZFS optimization.
///
/// Returns an error until real ZFS analytics is wired.
pub fn get_optimization_analytics(
    _service: &NativeZfsService,
) -> UniversalZfsResult<HashMap<String, serde_json::Value>> {
    Err(UniversalZfsError::ServiceUnavailable {
        message: "ZFS optimization analytics not yet wired".into(),
    })
}

/// Predict optimal storage tier for a dataset.
///
/// Returns an error until real tiering logic is wired.
pub fn predict_tier(
    _service: &NativeZfsService,
    _dataset_name: &str,
) -> UniversalZfsResult<String> {
    Err(UniversalZfsError::ServiceUnavailable {
        message: "ZFS tier prediction not yet wired".into(),
    })
}

/// Get the current configuration of the native ZFS service.
pub fn get_configuration(
    _service: &NativeZfsService,
) -> UniversalZfsResult<HashMap<String, serde_json::Value>> {
    let mut config = HashMap::new();
    config.insert(
        "service_name".into(),
        serde_json::Value::String("native-zfs".into()),
    );
    config.insert("version".into(), serde_json::Value::String("1.0.0".into()));
    Ok(config)
}

/// Update the configuration of the native ZFS service.
///
/// Returns an error until real configuration updates are wired.
pub fn update_configuration(
    _service: &NativeZfsService,
    _config: HashMap<String, serde_json::Value>,
) -> UniversalZfsResult<()> {
    Err(UniversalZfsError::ServiceUnavailable {
        message: "ZFS configuration updates not yet wired".into(),
    })
}
