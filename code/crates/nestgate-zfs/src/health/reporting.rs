// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Parse `zpool` / `zfs` command output into [`super::types::HealthStatus`] for reporting.

use super::types::HealthStatus;

/// Interpret `zpool status` stdout for health classification.
///
/// Used by `ZfsHealthMonitor::check_pool_health` so parsing logic can be tested without ZFS.
pub fn pool_health_from_zpool_status_text(stdout: &str) -> HealthStatus {
    if stdout.contains("ONLINE") && !stdout.contains("errors:") {
        HealthStatus::Healthy
    } else if stdout.contains("DEGRADED")
        || stdout.contains("FAULTED")
        || stdout.contains("UNAVAIL")
    {
        HealthStatus::Critical
    } else {
        HealthStatus::Warning
    }
}

/// Interpret `zfs list -H -o name,avail` stdout for dataset space health.
///
/// Used by `ZfsHealthMonitor::check_dataset_health` so threshold logic can be unit-tested.
pub fn dataset_health_from_zfs_list_text(stdout: &str) -> HealthStatus {
    const ONE_GIB: u64 = 1024 * 1024 * 1024;

    let mut total_datasets = 0_u32;
    let mut low_space_datasets = 0_u32;

    for line in stdout.lines() {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() >= 2 {
            total_datasets += 1;

            if let Ok(avail_bytes) = fields[1].parse::<u64>()
                && avail_bytes < ONE_GIB
            {
                low_space_datasets += 1;
            }
        }
    }

    if low_space_datasets == 0 {
        HealthStatus::Healthy
    } else if low_space_datasets < total_datasets / 2 {
        HealthStatus::Warning
    } else {
        HealthStatus::Critical
    }
}
