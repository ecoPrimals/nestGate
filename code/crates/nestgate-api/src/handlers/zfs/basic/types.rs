// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Request/response types and ZFS health evaluation.

use crate::dev_stubs::zfs::ZeroCostPoolInfo;
use std::collections::HashMap;

/// **ZFS API REQUEST - CREATE POOL**
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// Request parameters for `CreatePool` operation
pub struct CreatePoolRequest {
    /// Name of the ZFS pool to create
    pub name: String,
    /// List of device paths to use for the pool
    pub _devices: Vec<String>,
}

/// **ZFS API REQUEST - CREATE DATASET**
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// Request parameters for `CreateDataset` operation
pub struct CreateDatasetRequest {
    /// Name of the ZFS dataset to create
    pub name: String,
    /// Optional ZFS properties to set on the dataset
    pub properties: Option<HashMap<String, String>>,
}

/// **ZFS API REQUEST - CREATE SNAPSHOT**
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// Request parameters for `CreateSnapshot` operation
pub struct CreateSnapshotRequest {
    /// Dataset to snapshot
    pub dataset: String,
    /// Name of the snapshot to create
    pub name: String,
}

/// **ZFS HEALTH RESPONSE**
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// Response data for `ZfsHealth` operation
pub struct ZfsHealthResponse {
    /// Overall ZFS system health status
    pub healthy: bool,
    /// List of ZFS pools with their information
    pub pools: Vec<ZeroCostPoolInfo>,
    /// List of any health issues detected
    pub issues: Vec<String>,
}

/// Build [`ZfsHealthResponse`] from a pool listing (health endpoint and unit tests).
pub(crate) fn evaluate_zfs_health(pools: Vec<ZeroCostPoolInfo>) -> ZfsHealthResponse {
    let mut issues = Vec::new();
    let mut healthy = true;

    for pool in &pools {
        match pool.health.as_str() {
            "CRITICAL" | "FAULTED" | "UNAVAIL" => {
                healthy = false;
                issues.push(format!("Pool '{}' is in critical state", pool.name));
            }
            "DEGRADED" => {
                issues.push(format!("Pool '{}' has warnings", pool.name));
            }
            "UNKNOWN" => {
                issues.push(format!("Pool '{}' status unknown", pool.name));
            }
            _ => {}
        }
    }

    ZfsHealthResponse {
        healthy,
        pools,
        issues,
    }
}
