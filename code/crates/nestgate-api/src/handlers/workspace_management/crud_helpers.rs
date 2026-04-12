// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Shared ZFS helpers and size parsing for workspace CRUD.

use nestgate_types::{EnvSource, env_var_or_default};
use nestgate_zfs::numeric::f64_to_u64_saturating;
use serde_json::{Value, json};
use tokio::process::Command;

pub(super) fn zfs_executable(env: &(impl EnvSource + ?Sized)) -> String {
    env_var_or_default(env, "NESTGATE_ZFS_BINARY", "zfs")
}

pub(super) fn workspace_pool_name(env: &(impl EnvSource + ?Sized)) -> String {
    env_var_or_default(env, "NESTGATE_WORKSPACE_POOL", "zfspool")
}

/// Get additional workspace properties
pub(super) async fn get_workspace_properties(
    zfs_bin: &str,
    dataset_name: &str,
) -> (String, String, String) {
    let props_output = Command::new(zfs_bin)
        .args([
            "get",
            "-H",
            "-o",
            "value",
            "compression,quota,mounted",
            dataset_name,
        ])
        .output()
        .await;
    if let Ok(output) = props_output
        && output.status.success()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        if lines.len() >= 3 {
            let compression = lines[0].to_string();
            let quota = lines[1].to_string();
            let mounted = lines[2];
            let status = if mounted == "yes" {
                "active"
            } else {
                "inactive"
            };
            return (compression, quota, status.to_string());
        }
    }

    ("lz4".to_string(), "none".to_string(), "unknown".to_string())
}

/// Get workspace details for a specific workspace ID
pub(super) async fn get_workspace_details(
    zfs_bin: &str,
    env: &(impl EnvSource + ?Sized),
    workspace_id: &str,
) -> Value {
    let pool_name = workspace_pool_name(env);
    let dataset_name = format!("{pool_name}/workspaces/{workspace_id}");
    let props_output = Command::new(zfs_bin)
        .args([
            "get",
            "-H",
            "-o",
            "value",
            "used,available,quota,compression",
            &dataset_name,
        ])
        .output()
        .await;

    if let Ok(output) = props_output
        && output.status.success()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        if lines.len() >= 4 {
            return json!({
                "used": lines[0],
                "available": lines[1],
                "quota": lines[2],
                "compression": lines[3]
            });
        }
    }

    json!({
        "used": "unknown",
        "available": "unknown",
        "quota": "unknown",
        "compression": "unknown"
    })
}

/// Get snapshot count for a dataset
pub(super) async fn get_snapshot_count(zfs_bin: &str, dataset_name: &str) -> u32 {
    let snapshot_output = Command::new(zfs_bin)
        .args(["list", "-H", "-t", "snapshot", "-d", "1", dataset_name])
        .output()
        .await;
    if let Ok(output) = snapshot_output
        && output.status.success()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return stdout.lines().count() as u32;
    }

    0
}

/// Parse ZFS size strings (e.g., "1.5G", "512M") to bytes
pub fn parse_size(size_str: &str) -> u64 {
    if size_str == "none" || size_str == "-" {
        return 0;
    }
    let size_str = size_str.trim();
    if size_str.is_empty() {
        return 0;
    }

    // Handle numeric-only values (bytes)
    if let Ok(bytes) = size_str.parse::<u64>() {
        return bytes;
    }

    // Handle suffixed values
    let (number_part, suffix) = if size_str.len() > 1 {
        let split_pos = size_str.len() - 1;
        let (num, suf) = size_str.split_at(split_pos);
        (num, suf)
    } else {
        return 0;
    };

    if let Ok(number) = number_part.parse::<f64>() {
        let multiplier = match suffix.to_uppercase().as_str() {
            "K" => 1024,
            "M" => 1024 * 1024,
            "G" => 1024 * 1024 * 1024,
            "T" => 1024_u64 * 1024 * 1024 * 1024,
            "P" => 1024_u64 * 1024 * 1024 * 1024 * 1024,
            _ => 1,
        };

        f64_to_u64_saturating(number * multiplier as f64)
    } else {
        0
    }
}
