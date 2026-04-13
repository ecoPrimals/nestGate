// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Apply individual ZFS user properties during workspace update.

use tokio::process::Command;
use tracing::info;

pub(super) async fn workspace_apply_quota(
    zfs_bin: &str,
    dataset_name: &str,
    quota: &str,
    updated_properties: &mut Vec<String>,
    errors: &mut Vec<String>,
) {
    let quota_result = Command::new(zfs_bin)
        .args(["set", &format!("quota={quota}"), dataset_name])
        .output()
        .await;

    match quota_result {
        Ok(output) if output.status.success() => {
            updated_properties.push(format!("quota: {quota}"));
            info!("Updated quota to: {}", quota);
        }
        Ok(output) => {
            errors.push(format!(
                "Failed to update quota: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Err(e) => {
            errors.push(format!("Quota update command failed: {e}"));
        }
    }
}

pub(super) async fn workspace_apply_compression(
    zfs_bin: &str,
    dataset_name: &str,
    compression: &str,
    updated_properties: &mut Vec<String>,
    errors: &mut Vec<String>,
) {
    let compression_result = Command::new(zfs_bin)
        .args(["set", &format!("compression={compression}"), dataset_name])
        .output()
        .await;

    match compression_result {
        Ok(output) if output.status.success() => {
            updated_properties.push(format!("compression: {compression}"));
            info!("Updated compression to: {}", compression);
        }
        Ok(output) => {
            errors.push(format!(
                "Failed to update compression: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Err(_e) => {
            errors.push("Compression update command failed".to_string());
        }
    }
}

pub(super) async fn workspace_apply_name(
    zfs_bin: &str,
    dataset_name: &str,
    name: &str,
    updated_properties: &mut Vec<String>,
    errors: &mut Vec<String>,
) {
    let prop = format!("org.nestgate:workspace_name={name}");
    let name_result = Command::new(zfs_bin)
        .args(["set", &prop, dataset_name])
        .output()
        .await;

    match name_result {
        Ok(output) if output.status.success() => {
            updated_properties.push(format!("name: {name}"));
            info!("Updated workspace name to: {}", name);
        }
        Ok(output) => {
            errors.push(format!(
                "Failed to update name: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Err(_e) => {
            errors.push("Name update command failed".to_string());
        }
    }
}
