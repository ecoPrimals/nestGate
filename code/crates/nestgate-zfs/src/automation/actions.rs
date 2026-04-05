// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]
//
// This module implements lifecycle actions and automatic stage rules
// for dataset management and tier optimization.

//! Actions module

use super::types::{DatasetLifecycle, LifecycleStage};
use crate::types::StorageTier;
use nestgate_core::NestGateError;
use nestgate_core::error::CanonicalResult as Result;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, info, warn};

/// Message returned for lifecycle actions that are not yet connected to ZFS operations.
pub const NOT_WIRED_TO_ZFS_MSG: &str = "Not yet wired to ZFS operations";

/// Result of executing a lifecycle action
#[derive(Debug, Clone)]
/// Actionresult
pub struct ActionResult {
    /// Action
    pub action: String,
    /// Success
    pub success: bool,
    /// Message
    pub message: String,
    /// Timestamp
    pub timestamp: SystemTime,
}
/// Execute a lifecycle action on a dataset
pub fn execute_lifecycle_action(
    dataset_name: &str,
    lifecycle: &DatasetLifecycle,
    action: &str,
) -> Result<ActionResult> {
    debug!(
        "Executing lifecycle action '{}' on dataset '{}' in stage {:?}",
        action, dataset_name, lifecycle.lifecycle_stage
    );
    let timestamp = SystemTime::now();

    match action {
        "compress" => execute_compression_action(dataset_name),
        "migrate_to_cold" => execute_migration_action(dataset_name, StorageTier::Cold),
        "migrate_to_warm" => execute_migration_action(dataset_name, StorageTier::Warm),
        "migrate_to_hot" => execute_migration_action(dataset_name, StorageTier::Hot),
        "create_snapshot" => execute_snapshot_action(dataset_name),
        "optimize_properties" => execute_optimization_action(dataset_name, lifecycle),
        "update_access_time" => execute_access_time_update(dataset_name),
        "archive" => execute_archive_action(dataset_name),
        "cleanup_temp_files" => execute_cleanup_action(dataset_name),
        _ => {
            warn!("Unknown lifecycle action: {}", action);
            Ok(ActionResult {
                action: action.to_string(),
                success: false,
                message: format!("Unknown action: {action}"),
                timestamp,
            })
        }
    }
}

/// Apply automatic stage rules based on current lifecycle stage
pub async fn apply_automatic_stage_rules(
    dataset_name: &str,
    lifecycle: &DatasetLifecycle,
) -> Result<()> {
    debug!(
        "Applying automatic stage rules for dataset '{}' in stage {:?}",
        dataset_name, lifecycle.lifecycle_stage
    );
    match lifecycle.lifecycle_stage {
        LifecycleStage::New => {
            // New datasets: ensure optimal properties for expected high activity
            apply_new_dataset_rules(dataset_name)?;
        }
        LifecycleStage::Active => {
            // Active datasets: monitor and optimize for performance
            apply_active_dataset_rules(dataset_name, lifecycle)?;
        }
        LifecycleStage::Aging => {
            // Aging datasets: prepare for potential migration to cold storage
            apply_aging_dataset_rules(dataset_name, lifecycle).await?;
        }
        LifecycleStage::Archived => {
            // Archived datasets: optimize for space efficiency
            apply_archived_dataset_rules(dataset_name).await?;
        }
        LifecycleStage::Obsolete => {
            // Obsolete datasets: prepare for cleanup
            apply_obsolete_dataset_rules(dataset_name).await?;
        }
    }

    Ok(())
}

// Individual action implementations

fn not_wired_action_result(action: &str) -> ActionResult {
    ActionResult {
        action: action.to_string(),
        success: false,
        message: NOT_WIRED_TO_ZFS_MSG.to_string(),
        timestamp: SystemTime::now(),
    }
}

fn execute_compression_action(dataset_name: &str) -> Result<ActionResult> {
    info!("Applying compression to dataset: {}", dataset_name);

    let timestamp = SystemTime::now();
    match std::process::Command::new("zfs")
        .args(["set", "compression=lz4", dataset_name])
        .output()
    {
        Ok(out) if out.status.success() => Ok(ActionResult {
            action: "compress".to_string(),
            success: true,
            message: "Compression property set to lz4".to_string(),
            timestamp,
        }),
        Ok(out) => Ok(ActionResult {
            action: "compress".to_string(),
            success: false,
            message: format!(
                "zfs set compression failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ),
            timestamp,
        }),
        Err(e) => Ok(ActionResult {
            action: "compress".to_string(),
            success: false,
            message: format!("failed to run zfs: {e}"),
            timestamp,
        }),
    }
}

/// Execute Migration Action
fn execute_migration_action(dataset_name: &str, target_tier: StorageTier) -> Result<ActionResult> {
    info!(
        "Migrating dataset '{}' to {:?} tier (not wired)",
        dataset_name, target_tier
    );

    Err(NestGateError::not_implemented(
        "ZFS tier migration engine not yet wired; use zfs send/recv or native pool tools until automation coordinates with the migration IPC",
    ))
}

/// Execute Snapshot Action
fn execute_snapshot_action(dataset_name: &str) -> Result<ActionResult> {
    info!("Creating snapshot for dataset: {}", dataset_name);

    let timestamp = SystemTime::now();
    let secs = timestamp
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let snapshot_name = format!("nestgate-auto-{secs}");
    let full_snap = format!("{dataset_name}@{snapshot_name}");

    match std::process::Command::new("zfs")
        .args(["snapshot", &full_snap])
        .output()
    {
        Ok(out) if out.status.success() => Ok(ActionResult {
            action: "create_snapshot".to_string(),
            success: true,
            message: format!("Snapshot created: {full_snap}"),
            timestamp,
        }),
        Ok(out) => Ok(ActionResult {
            action: "create_snapshot".to_string(),
            success: false,
            message: format!(
                "zfs snapshot failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ),
            timestamp,
        }),
        Err(e) => Ok(ActionResult {
            action: "create_snapshot".to_string(),
            success: false,
            message: format!("failed to run zfs: {e}"),
            timestamp,
        }),
    }
}

/// Execute Optimization Action
fn execute_optimization_action(
    dataset_name: &str,
    lifecycle: &DatasetLifecycle,
) -> Result<ActionResult> {
    info!(
        "Optimizing properties for dataset '{}' in stage {:?}",
        dataset_name, lifecycle.lifecycle_stage
    );

    Ok(ActionResult {
        action: "optimize_properties".to_string(),
        success: false,
        message: NOT_WIRED_TO_ZFS_MSG.to_string(),
        timestamp: SystemTime::now(),
    })
}

/// Execute Access Time Update
fn execute_access_time_update(dataset_name: &str) -> Result<ActionResult> {
    debug!("Updating access time for dataset: {}", dataset_name);

    Ok(not_wired_action_result("update_access_time"))
}

/// Execute Archive Action
fn execute_archive_action(dataset_name: &str) -> Result<ActionResult> {
    info!("Archiving dataset: {}", dataset_name);

    Ok(not_wired_action_result("archive"))
}

/// Execute Cleanup Action
fn execute_cleanup_action(dataset_name: &str) -> Result<ActionResult> {
    info!("Cleaning up temporary files for dataset: {}", dataset_name);

    Ok(not_wired_action_result("cleanup_temp_files"))
}

// Automatic stage rules implementation
fn apply_new_dataset_rules(dataset_name: &str) -> Result<()> {
    debug!("Applying rules for new dataset: {}", dataset_name);

    // For new datasets, ensure hot tier placement and optimal properties
    if let Err(e) = execute_migration_action(dataset_name, StorageTier::Hot) {
        warn!(
            "Skipping tier migration for new dataset {} (not implemented): {}",
            dataset_name, e
        );
    }
    execute_optimization_action(
        dataset_name,
        &DatasetLifecycle {
            dataset_name: dataset_name.to_string(),
            current_tier: StorageTier::Hot,
            created: SystemTime::now(),
            last_accessed: Some(SystemTime::now()),
            access_count: 0,
            total_migrations: 0,
            last_optimization: None,
            lifecycle_stage: LifecycleStage::New,
            automation_history: Vec::new(),
        },
    )?;

    Ok(())
}

/// Apply Active Dataset Rules
fn apply_active_dataset_rules(dataset_name: &str, lifecycle: &DatasetLifecycle) -> Result<()> {
    debug!("Applying rules for active dataset: {}", dataset_name);

    // For active datasets, monitor performance and maintain optimal tier
    if lifecycle.current_tier != StorageTier::Hot && lifecycle.access_count > 100 {
        if let Err(e) = execute_migration_action(dataset_name, StorageTier::Hot) {
            warn!(
                "Skipping tier migration for active dataset {} (not implemented): {}",
                dataset_name, e
            );
        }
    }

    // Create periodic snapshots for active datasets
    execute_snapshot_action(dataset_name)?;

    Ok(())
}

/// Apply Aging Dataset Rules
async fn apply_aging_dataset_rules(dataset_name: &str, lifecycle: &DatasetLifecycle) -> Result<()> {
    debug!("Applying rules for aging dataset: {}", dataset_name);

    // For aging datasets, prepare for migration to cold storage
    if lifecycle.current_tier == StorageTier::Hot {
        if let Err(e) = execute_migration_action(dataset_name, StorageTier::Warm) {
            warn!(
                "Skipping tier migration for aging dataset {} (not implemented): {}",
                dataset_name, e
            );
        }
    }

    // Apply compression to save space
    execute_compression_action(dataset_name)?;

    Ok(())
}

/// Apply Archived Dataset Rules
async fn apply_archived_dataset_rules(dataset_name: &str) -> Result<()> {
    debug!("Applying rules for archived dataset: {}", dataset_name);

    // For archived datasets, ensure cold storage and maximum compression
    if let Err(e) = execute_migration_action(dataset_name, StorageTier::Cold) {
        warn!(
            "Skipping tier migration for archived dataset {} (not implemented): {}",
            dataset_name, e
        );
    }
    execute_compression_action(dataset_name)?;

    Ok(())
}

/// Apply Obsolete Dataset Rules
async fn apply_obsolete_dataset_rules(dataset_name: &str) -> Result<()> {
    debug!("Applying rules for obsolete dataset: {}", dataset_name);

    // For obsolete datasets, prepare for cleanup
    execute_cleanup_action(dataset_name)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::automation::types::DatasetLifecycle;
    use crate::types::StorageTier;
    use nestgate_core::NestGateError;
    use std::time::SystemTime;

    fn lifecycle(
        name: &str,
        stage: LifecycleStage,
        tier: StorageTier,
        access_count: u64,
    ) -> DatasetLifecycle {
        DatasetLifecycle {
            dataset_name: name.to_string(),
            current_tier: tier,
            created: SystemTime::UNIX_EPOCH,
            last_accessed: Some(SystemTime::UNIX_EPOCH),
            access_count,
            total_migrations: 0,
            last_optimization: None,
            lifecycle_stage: stage,
            automation_history: vec![],
        }
    }

    #[test]
    fn execute_lifecycle_action_not_wired_returns_honest_failure() {
        let ds = "tank/data/app";
        let lc = lifecycle(ds, LifecycleStage::Active, StorageTier::Hot, 0);
        for action in [
            "optimize_properties",
            "update_access_time",
            "archive",
            "cleanup_temp_files",
        ] {
            let r = execute_lifecycle_action(ds, &lc, action).expect("test: known action executes");
            assert!(
                !r.success && r.message == NOT_WIRED_TO_ZFS_MSG,
                "action={action} msg={}",
                r.message
            );
        }
    }

    #[test]
    fn execute_lifecycle_action_compress_and_snapshot_invoke_zfs_cli() {
        let ds = "tank/data/app";
        let lc = lifecycle(ds, LifecycleStage::Active, StorageTier::Hot, 0);
        let compress = execute_lifecycle_action(ds, &lc, "compress").expect("compress");
        let snapshot = execute_lifecycle_action(ds, &lc, "create_snapshot").expect("snapshot");
        // Without a real pool/dataset these typically fail honestly; with ZFS they may succeed.
        assert!(
            compress.success || compress.message.contains("zfs"),
            "compress: {}",
            compress.message
        );
        assert!(
            snapshot.success || snapshot.message.contains("zfs"),
            "snapshot: {}",
            snapshot.message
        );
    }

    #[test]
    fn execute_lifecycle_action_migration_returns_not_implemented() {
        let ds = "tank/data/app";
        let lc = lifecycle(ds, LifecycleStage::Active, StorageTier::Hot, 0);
        for action in ["migrate_to_cold", "migrate_to_warm", "migrate_to_hot"] {
            let err = execute_lifecycle_action(ds, &lc, action).expect_err("migration not wired");
            assert!(
                matches!(err, NestGateError::NotImplemented(_)),
                "action={action} err={err:?}"
            );
        }
    }

    #[test]
    fn execute_lifecycle_action_unknown_returns_failure_result() {
        let r = execute_lifecycle_action(
            "z",
            &lifecycle("z", LifecycleStage::New, StorageTier::Hot, 0),
            "not_a_real_action",
        )
        .expect("test: unknown action returns Ok");
        assert!(!r.success);
        assert!(r.message.contains("Unknown action"));
    }

    #[tokio::test]
    async fn apply_automatic_stage_rules_new_and_active() {
        let ds = "pool/fs1";
        apply_automatic_stage_rules(
            ds,
            &lifecycle(ds, LifecycleStage::New, StorageTier::Warm, 0),
        )
        .await
        .expect("test: new stage rules");

        apply_automatic_stage_rules(
            ds,
            &lifecycle(ds, LifecycleStage::Active, StorageTier::Hot, 10),
        )
        .await
        .expect("test: active hot low access");

        apply_automatic_stage_rules(
            ds,
            &lifecycle(ds, LifecycleStage::Active, StorageTier::Warm, 150),
        )
        .await
        .expect("test: active warm high access migrates");
    }

    #[tokio::test]
    async fn apply_automatic_stage_rules_aging_archived_obsolete() {
        let ds = "pool/fs2";
        apply_automatic_stage_rules(
            ds,
            &lifecycle(ds, LifecycleStage::Aging, StorageTier::Hot, 0),
        )
        .await
        .expect("test: aging");

        apply_automatic_stage_rules(
            ds,
            &lifecycle(ds, LifecycleStage::Archived, StorageTier::Cold, 0),
        )
        .await
        .expect("test: archived");

        apply_automatic_stage_rules(
            ds,
            &lifecycle(ds, LifecycleStage::Obsolete, StorageTier::Cold, 0),
        )
        .await
        .expect("test: obsolete");
    }

    #[test]
    fn execute_optimize_properties_not_wired() {
        let ds = "tank/opt";
        let lc = lifecycle(ds, LifecycleStage::Active, StorageTier::Hot, 1);
        let r = execute_lifecycle_action(ds, &lc, "optimize_properties")
            .expect("test: optimize_properties");
        assert!(!r.success);
        assert_eq!(r.message, NOT_WIRED_TO_ZFS_MSG);
    }
}
