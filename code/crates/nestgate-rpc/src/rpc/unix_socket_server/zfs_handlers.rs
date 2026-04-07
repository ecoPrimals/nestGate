// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS domain JSON-RPC handlers (`zfs.*`).
//!
//! Exposes ZFS pool, dataset, and snapshot operations over the ecosystem's
//! uniform JSON-RPC/UDS interface, resolving GAP-MATRIX-04 (REST-only ZFS
//! surface). Handlers call `zpool`/`zfs` CLI directly via
//! [`tokio::process::Command`] to avoid a cyclic crate dependency on
//! `nestgate-zfs`.
//!
//! Method naming follows `SEMANTIC_METHOD_NAMING_STANDARD.md`:
//! `{domain}.{operation}[.{variant}]`.

use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use tracing::debug;

/// Run a ZFS CLI command and return `(stdout, stderr, success)`.
async fn run_zfs_cmd(program: &str, args: &[&str]) -> Result<(String, String, bool)> {
    let output = tokio::process::Command::new(program)
        .args(args)
        .output()
        .await
        .map_err(|e| NestGateError::internal(format!("{program} execution failed: {e}")))?;

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    Ok((stdout, stderr, output.status.success()))
}

/// Check whether `zpool` is available on the host.
async fn is_zpool_available() -> bool {
    tokio::process::Command::new("zpool")
        .arg("version")
        .output()
        .await
        .is_ok_and(|o| o.status.success())
}

/// Check whether `zfs` is available on the host.
async fn is_zfs_available() -> bool {
    tokio::process::Command::new("zfs")
        .arg("version")
        .output()
        .await
        .is_ok_and(|o| o.status.success())
}

fn zfs_unavailable(tool: &str) -> NestGateError {
    NestGateError::service_unavailable(format!(
        "{tool} is not available on this system; install ZFS userland tools"
    ))
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// `zfs.pool.list` — list all ZFS pools with size, allocation, free space, and
/// health status.
///
/// # Errors
///
/// Returns an error if `zpool` is unavailable or the command fails.
pub(super) async fn zfs_pool_list(_params: Option<&Value>) -> Result<Value> {
    debug!("zfs.pool.list");
    if !is_zpool_available().await {
        return Err(zfs_unavailable("zpool"));
    }

    let (stdout, stderr, ok) = run_zfs_cmd(
        "zpool",
        &["list", "-H", "-o", "name,size,alloc,free,health"],
    )
    .await?;
    if !ok {
        return Err(NestGateError::internal(format!(
            "zpool list failed: {stderr}"
        )));
    }

    let pools: Vec<Value> = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('\t').collect();
            (parts.len() >= 5).then(|| {
                json!({
                    "name": parts[0],
                    "size": parts[1],
                    "allocated": parts[2],
                    "free": parts[3],
                    "health": parts[4],
                })
            })
        })
        .collect();

    Ok(json!({ "status": "success", "pools": pools }))
}

/// `zfs.pool.get` — get status for a single pool.
///
/// Params: `{ "pool": "<name>" }`
///
/// # Errors
///
/// Returns an error if `zpool` is unavailable, the pool name is missing, or the
/// command fails.
pub(super) async fn zfs_pool_get(params: Option<&Value>) -> Result<Value> {
    let pool = params
        .and_then(|p| p.get("pool"))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field("pool", "required string parameter")
        })?;
    debug!("zfs.pool.get pool={pool}");

    if !is_zpool_available().await {
        return Err(zfs_unavailable("zpool"));
    }

    let (stdout, stderr, ok) = run_zfs_cmd("zpool", &["status", pool]).await?;
    if !ok {
        return Err(NestGateError::not_found(format!("pool {pool:?}: {stderr}")));
    }

    let healthy = stdout.contains("state: ONLINE") || stdout.contains("HEALTHY");
    let errors = stdout.contains("errors:") && !stdout.contains("errors: No known data errors");
    let scan = stdout
        .lines()
        .find(|l| l.trim().starts_with("scan:"))
        .map_or("none requested", |l| {
            l.trim().strip_prefix("scan:").unwrap_or("").trim()
        });

    Ok(json!({
        "status": "success",
        "pool": pool,
        "state": if healthy { "ONLINE" } else { "DEGRADED" },
        "scan": scan,
        "errors": if errors { "Yes" } else { "No" },
    }))
}

/// `zfs.pool.health` — summarize health across all pools, flagging unhealthy ones.
///
/// # Errors
///
/// Returns an error if `zpool` is unavailable or the command fails.
pub(super) async fn zfs_pool_health(_params: Option<&Value>) -> Result<Value> {
    debug!("zfs.pool.health");
    if !is_zpool_available().await {
        return Err(zfs_unavailable("zpool"));
    }

    let (stdout, stderr, ok) = run_zfs_cmd("zpool", &["list", "-H", "-o", "name,health"]).await?;
    if !ok {
        return Err(NestGateError::internal(format!(
            "zpool list failed: {stderr}"
        )));
    }

    let mut total = 0u32;
    let mut unhealthy: Vec<String> = Vec::new();
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 2 {
            total += 1;
            let h = parts[1].to_ascii_lowercase();
            if !(h.contains("online") || h == "ok" || h == "healthy") {
                unhealthy.push(parts[0].to_string());
            }
        }
    }

    Ok(json!({
        "status": "success",
        "pool_count": total,
        "pools_unhealthy": unhealthy,
    }))
}

/// `zfs.dataset.list` — list all ZFS datasets (filesystems).
///
/// Optional params: `{ "pool": "<name>" }` to scope to a single pool.
///
/// # Errors
///
/// Returns an error if `zfs` is unavailable or the command fails.
pub(super) async fn zfs_dataset_list(params: Option<&Value>) -> Result<Value> {
    let pool = params.and_then(|p| p.get("pool")).and_then(Value::as_str);
    debug!("zfs.dataset.list pool={pool:?}");

    if !is_zfs_available().await {
        return Err(zfs_unavailable("zfs"));
    }

    let mut args = vec!["list", "-H", "-o", "name,used,avail,refer,mountpoint"];
    if let Some(p) = pool {
        args.push(p);
    }

    let (stdout, stderr, ok) = run_zfs_cmd("zfs", &args).await?;
    if !ok {
        return Err(NestGateError::internal(format!(
            "zfs list failed: {stderr}"
        )));
    }

    let datasets: Vec<Value> = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('\t').collect();
            (parts.len() >= 5).then(|| {
                json!({
                    "name": parts[0],
                    "used": parts[1],
                    "available": parts[2],
                    "referenced": parts[3],
                    "mountpoint": parts[4],
                })
            })
        })
        .collect();

    Ok(json!({ "status": "success", "datasets": datasets }))
}

/// `zfs.dataset.get` — get a single dataset by name.
///
/// Params: `{ "dataset": "<name>" }`
///
/// # Errors
///
/// Returns an error if `zfs` is unavailable, the dataset name is missing, or the
/// dataset is not found.
pub(super) async fn zfs_dataset_get(params: Option<&Value>) -> Result<Value> {
    let dataset = params
        .and_then(|p| p.get("dataset"))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field("dataset", "required string parameter")
        })?;
    debug!("zfs.dataset.get dataset={dataset}");

    if !is_zfs_available().await {
        return Err(zfs_unavailable("zfs"));
    }

    let (stdout, stderr, ok) = run_zfs_cmd(
        "zfs",
        &[
            "list",
            "-H",
            "-o",
            "name,used,avail,refer,mountpoint",
            dataset,
        ],
    )
    .await?;
    if !ok {
        return Err(NestGateError::not_found(format!(
            "dataset {dataset:?}: {stderr}"
        )));
    }

    let line = stdout.lines().next().unwrap_or_default();
    let parts: Vec<&str> = line.split('\t').collect();
    if parts.len() < 5 {
        return Err(NestGateError::not_found(format!(
            "dataset {dataset:?} not found"
        )));
    }

    Ok(json!({
        "status": "success",
        "dataset": {
            "name": parts[0],
            "used": parts[1],
            "available": parts[2],
            "referenced": parts[3],
            "mountpoint": parts[4],
        },
    }))
}

/// `zfs.snapshot.list` — list ZFS snapshots.
///
/// Optional params: `{ "dataset": "<name>" }` to scope to a single dataset.
///
/// # Errors
///
/// Returns an error if `zfs` is unavailable or the command fails.
pub(super) async fn zfs_snapshot_list(params: Option<&Value>) -> Result<Value> {
    let dataset = params
        .and_then(|p| p.get("dataset"))
        .and_then(Value::as_str);
    debug!("zfs.snapshot.list dataset={dataset:?}");

    if !is_zfs_available().await {
        return Err(zfs_unavailable("zfs"));
    }

    let mut args = vec!["list", "-H", "-t", "snapshot", "-o", "name,used,creation"];
    if let Some(ds) = dataset {
        args.push(ds);
    }

    let (stdout, stderr, ok) = run_zfs_cmd("zfs", &args).await?;
    if !ok {
        return Err(NestGateError::internal(format!(
            "zfs list snapshots failed: {stderr}"
        )));
    }

    let snapshots: Vec<Value> = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('\t').collect();
            (parts.len() >= 3).then(|| {
                json!({
                    "name": parts[0],
                    "used": parts[1],
                    "creation": parts[2],
                })
            })
        })
        .collect();

    Ok(json!({ "status": "success", "snapshots": snapshots }))
}

/// `zfs.health` — report ZFS/zpool userland availability and version.
///
/// # Errors
///
/// Returns an error if neither `zpool` nor `zfs` commands can execute.
pub(super) async fn zfs_health(_params: Option<&Value>) -> Result<Value> {
    debug!("zfs.health");

    let zpool_ok = is_zpool_available().await;
    let zfs_ok = is_zfs_available().await;

    if !zpool_ok && !zfs_ok {
        return Err(NestGateError::service_unavailable(
            "neither zpool nor zfs userland tools are available",
        ));
    }

    let version = if zfs_ok {
        let (stdout, _, ok) = run_zfs_cmd("zfs", &["version"]).await?;
        if ok {
            stdout
                .lines()
                .next()
                .unwrap_or("unknown")
                .trim()
                .to_string()
        } else {
            "unknown".to_string()
        }
    } else {
        "unknown".to_string()
    };

    Ok(json!({
        "status": "success",
        "zpool_available": zpool_ok,
        "zfs_available": zfs_ok,
        "zfs_version": version,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn zfs_pool_list_returns_result() {
        let result = zfs_pool_list(None).await;
        if is_zpool_available().await {
            let val = result.expect("should succeed when zpool available");
            assert_eq!(val["status"], "success");
            assert!(val.get("pools").is_some());
        } else {
            assert!(result.is_err());
        }
    }

    #[tokio::test]
    async fn zfs_pool_get_requires_pool_param() {
        let result = zfs_pool_get(None).await;
        assert!(result.is_err());

        let result = zfs_pool_get(Some(&json!({}))).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn zfs_pool_get_nonexistent() {
        if !is_zpool_available().await {
            return;
        }
        let result = zfs_pool_get(Some(&json!({"pool": "__nonexistent_pool_xyz__"}))).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn zfs_pool_health_returns_result() {
        let result = zfs_pool_health(None).await;
        if is_zpool_available().await {
            let val = result.expect("should succeed when zpool available");
            assert_eq!(val["status"], "success");
            assert!(val.get("pool_count").is_some());
        } else {
            assert!(result.is_err());
        }
    }

    #[tokio::test]
    async fn zfs_dataset_list_returns_result() {
        let result = zfs_dataset_list(None).await;
        if is_zfs_available().await {
            let val = result.expect("should succeed when zfs available");
            assert_eq!(val["status"], "success");
            assert!(val.get("datasets").is_some());
        } else {
            assert!(result.is_err());
        }
    }

    #[tokio::test]
    async fn zfs_dataset_get_requires_param() {
        let result = zfs_dataset_get(None).await;
        assert!(result.is_err());

        let result = zfs_dataset_get(Some(&json!({}))).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn zfs_snapshot_list_returns_result() {
        let result = zfs_snapshot_list(None).await;
        if is_zfs_available().await {
            let val = result.expect("should succeed when zfs available");
            assert_eq!(val["status"], "success");
            assert!(val.get("snapshots").is_some());
        } else {
            assert!(result.is_err());
        }
    }

    #[tokio::test]
    async fn zfs_health_returns_availability() {
        let result = zfs_health(None).await;
        if is_zpool_available().await || is_zfs_available().await {
            let val = result.expect("should succeed when some zfs tool available");
            assert_eq!(val["status"], "success");
            assert!(val.get("zpool_available").is_some());
            assert!(val.get("zfs_available").is_some());
        } else {
            assert!(result.is_err());
        }
    }
}
