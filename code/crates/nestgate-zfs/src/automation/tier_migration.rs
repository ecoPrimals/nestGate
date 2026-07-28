// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS tier migration planning and execution (dry-run by default).

use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use nestgate_core::NestGateError;
use tracing::{debug, info};

use crate::config::{TierConfig, TierConfigurations};
use crate::types::StorageTier;

use super::Result;

/// Environment variable that must be `true` to allow mutating ZFS operations.
pub const MUTATIONS_ENV_VAR: &str = "NESTGATE_ZFS_ALLOW_MUTATIONS";

/// Planned steps for a tier migration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TierMigrationPlan {
    /// Source dataset path (`pool/dataset`).
    pub dataset_name: String,
    /// Current storage tier.
    pub source_tier: StorageTier,
    /// Target storage tier.
    pub target_tier: StorageTier,
    /// Source ZFS pool for the tier.
    pub source_pool: String,
    /// Target ZFS pool for the tier.
    pub target_pool: String,
    /// Destination dataset path after migration.
    pub target_dataset: String,
    /// Snapshot name used for send/receive.
    pub snapshot_name: String,
    /// Full snapshot identifier (`dataset@snap`).
    pub full_snapshot: String,
    /// Whether this plan is a dry-run analysis only.
    pub dry_run: bool,
    /// Human-readable migration steps.
    pub steps: Vec<String>,
    /// Estimated bytes to transfer, when `zfs list` succeeds.
    pub estimated_bytes: Option<u64>,
}

/// Returns `true` when real ZFS mutations are permitted.
#[must_use]
pub fn mutations_allowed() -> bool {
    std::env::var(MUTATIONS_ENV_VAR)
        .map(|value| value == "true")
        .unwrap_or(false)
}

/// Load tier configuration for the pool that owns `dataset_name`.
#[must_use]
pub fn tier_configuration_for_dataset(dataset_name: &str) -> TierConfigurations {
    let pool_name = pool_name_from_dataset(dataset_name);
    TierConfigurations::auto_detect_tiers(pool_name)
}

/// Validate that source and target tiers are present in the configuration.
///
/// # Errors
///
/// Returns an error when either tier is missing from the configuration map.
pub fn validate_tiers<'a>(
    tiers: &'a TierConfigurations,
    source_tier: &StorageTier,
    target_tier: &StorageTier,
) -> Result<(&'a TierConfig, &'a TierConfig)> {
    let source = tier_config_for(tiers, source_tier).ok_or_else(|| {
        NestGateError::validation_error(format!("source tier {source_tier:?} is not configured"))
    })?;
    let target = tier_config_for(tiers, target_tier).ok_or_else(|| {
        NestGateError::validation_error(format!("target tier {target_tier:?} is not configured"))
    })?;
    Ok((source, target))
}

/// Build a migration plan without executing ZFS mutations.
///
/// # Errors
///
/// Returns an error when tier validation fails or the dataset name is invalid.
pub fn plan_migration(
    dataset_name: &str,
    source_tier: StorageTier,
    target_tier: StorageTier,
    tiers: &TierConfigurations,
) -> Result<TierMigrationPlan> {
    if dataset_name.trim().is_empty() {
        return Err(NestGateError::validation_error(
            "dataset name cannot be empty",
        ));
    }

    if source_tier == target_tier {
        return Err(NestGateError::validation_error(format!(
            "dataset {dataset_name} is already on tier {target_tier:?}"
        )));
    }

    let tier_transition = format!("validate tiers: {source_tier:?} -> {target_tier:?}");
    let (source_config, target_config) = validate_tiers(tiers, &source_tier, &target_tier)?;
    let snapshot_name = migration_snapshot_name();
    let full_snapshot = format!("{dataset_name}@{snapshot_name}");
    let target_dataset = target_dataset_path(dataset_name, target_config);
    let estimated_bytes = query_dataset_used_bytes(dataset_name);
    let dry_run = !mutations_allowed();

    let steps = vec![
        tier_transition,
        format!(
            "source pool {} (tier {}) -> target pool {} (tier {})",
            source_config.pool_name,
            source_config.name,
            target_config.pool_name,
            target_config.name
        ),
        format!("create snapshot {full_snapshot}"),
        format!("zfs send {full_snapshot} | zfs receive {target_dataset}"),
        format!("retire source dataset {dataset_name} after successful receive"),
    ];

    Ok(TierMigrationPlan {
        dataset_name: dataset_name.to_string(),
        source_tier,
        target_tier,
        source_pool: source_config.pool_name.clone(),
        target_pool: target_config.pool_name.clone(),
        target_dataset,
        snapshot_name,
        full_snapshot,
        dry_run,
        steps,
        estimated_bytes,
    })
}

/// Execute a migration plan (dry-run by default).
///
/// # Errors
///
/// Returns an error when validation fails or a real migration command fails.
pub fn execute_migration_plan(plan: &TierMigrationPlan) -> Result<String> {
    if plan.dry_run {
        info!(
            "Dry-run tier migration for {} -> {:?}: {}",
            plan.dataset_name, plan.target_tier, plan.target_dataset
        );
        return Ok(format!(
            "Dry-run migration plan: {} ({:?} -> {:?}) via {}; estimated bytes: {}; set {MUTATIONS_ENV_VAR}=true to execute",
            plan.dataset_name,
            plan.source_tier,
            plan.target_tier,
            plan.target_dataset,
            plan.estimated_bytes
                .map_or_else(|| "unknown".to_string(), |bytes| bytes.to_string())
        ));
    }

    run_real_migration(plan)
}

/// Engine-facing migration entry point.
///
/// # Errors
///
/// Returns an error when planning or execution fails.
pub fn migrate_dataset(plan: &TierMigrationPlan) -> Result<()> {
    execute_migration_plan(plan).map(|_| ())
}

fn run_real_migration(plan: &TierMigrationPlan) -> Result<String> {
    info!(
        "Executing tier migration for {} -> {}",
        plan.dataset_name, plan.target_dataset
    );

    let snapshot_output = Command::new("zfs")
        .args(["snapshot", &plan.full_snapshot])
        .output()
        .map_err(|error| NestGateError::storage_error(format!("zfs snapshot failed: {error}")))?;

    if !snapshot_output.status.success() {
        let stderr = String::from_utf8_lossy(&snapshot_output.stderr);
        return Err(NestGateError::storage_error(format!(
            "zfs snapshot {} failed: {}",
            plan.full_snapshot,
            stderr.trim()
        )));
    }

    let mut send = Command::new("zfs")
        .args(["send", &plan.full_snapshot])
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|error| NestGateError::storage_error(format!("zfs send failed: {error}")))?;

    let send_stdout = send.stdout.take().ok_or_else(|| {
        NestGateError::storage_error("zfs send did not provide stdout pipe".to_string())
    })?;

    let receive_output = Command::new("zfs")
        .args(["receive", "-F", &plan.target_dataset])
        .stdin(send_stdout)
        .output()
        .map_err(|error| NestGateError::storage_error(format!("zfs receive failed: {error}")))?;

    let send_status = send
        .wait()
        .map_err(|error| NestGateError::storage_error(format!("zfs send wait failed: {error}")))?;

    if !send_status.success() {
        return Err(NestGateError::storage_error(format!(
            "zfs send {} failed with status {}",
            plan.full_snapshot, send_status
        )));
    }

    if !receive_output.status.success() {
        let stderr = String::from_utf8_lossy(&receive_output.stderr);
        return Err(NestGateError::storage_error(format!(
            "zfs receive {} failed: {}",
            plan.target_dataset,
            stderr.trim()
        )));
    }

    Ok(format!(
        "Migrated {} ({:?} -> {:?}) to {} via {}",
        plan.dataset_name,
        plan.source_tier,
        plan.target_tier,
        plan.target_dataset,
        plan.full_snapshot
    ))
}

const fn tier_config_for<'a>(
    tiers: &'a TierConfigurations,
    tier: &StorageTier,
) -> Option<&'a TierConfig> {
    match tier {
        StorageTier::Hot | StorageTier::Cache => Some(&tiers.hot),
        StorageTier::Warm => Some(&tiers.warm),
        StorageTier::Cold | StorageTier::Archive => Some(&tiers.cold),
    }
}

fn pool_name_from_dataset(dataset_name: &str) -> &str {
    dataset_name.split('/').next().unwrap_or("default")
}

fn target_dataset_path(dataset_name: &str, target: &TierConfig) -> String {
    let suffix = dataset_name
        .split('/')
        .skip(1)
        .collect::<Vec<_>>()
        .join("/");

    if suffix.is_empty() {
        format!("{}/{}", target.pool_name, target.dataset_prefix)
    } else {
        format!("{}/{}/{}", target.pool_name, target.dataset_prefix, suffix)
    }
}

fn migration_snapshot_name() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    format!("nestgate-migrate-{secs}")
}

fn query_dataset_used_bytes(dataset_name: &str) -> Option<u64> {
    let output = Command::new("zfs")
        .args(["list", "-Hp", "-o", "used", dataset_name])
        .output()
        .ok()?;

    if !output.status.success() {
        debug!(
            "Could not query used bytes for {}: {}",
            dataset_name,
            String::from_utf8_lossy(&output.stderr).trim()
        );
        return None;
    }

    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
    value.parse::<u64>().ok()
}

/// Infer storage tier from dataset naming hints.
#[must_use]
pub fn infer_tier_from_dataset_name(dataset_name: &str) -> StorageTier {
    let lower = dataset_name.to_ascii_lowercase();
    if lower.contains("hot") {
        StorageTier::Hot
    } else if lower.contains("cold") || lower.contains("archive") {
        StorageTier::Cold
    } else {
        StorageTier::Warm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_migration_dry_run_by_default() {
        let tiers = TierConfigurations::auto_detect_tiers("mypool");
        let plan = plan_migration(
            "mypool/app/data",
            StorageTier::Hot,
            StorageTier::Cold,
            &tiers,
        )
        .expect("plan");

        assert!(plan.dry_run);
        assert_eq!(plan.source_tier, StorageTier::Hot);
        assert_eq!(plan.target_tier, StorageTier::Cold);
        assert!(plan.target_dataset.contains("cold"));
        assert!(!plan.steps.is_empty());
    }

    #[test]
    fn validate_tiers_rejects_same_tier() {
        let tiers = TierConfigurations::default();
        let err = plan_migration("pool/data", StorageTier::Warm, StorageTier::Warm, &tiers)
            .expect_err("same tier");
        assert!(err.to_string().to_lowercase().contains("already"));
    }

    #[test]
    fn execute_migration_plan_dry_run_succeeds() {
        let tiers = TierConfigurations::default();
        let plan = plan_migration(
            "default/hot/app",
            StorageTier::Hot,
            StorageTier::Warm,
            &tiers,
        )
        .expect("plan");
        let message = execute_migration_plan(&plan).expect("dry-run");
        assert!(message.contains("Dry-run"));
    }

    #[test]
    fn mutations_allowed_requires_exact_true_value() {
        assert_eq!(MUTATIONS_ENV_VAR, "NESTGATE_ZFS_ALLOW_MUTATIONS");
        if std::env::var(MUTATIONS_ENV_VAR).as_deref() != Ok("true") {
            assert!(!mutations_allowed());
        }
    }

    #[test]
    fn infer_tier_from_dataset_name_hints() {
        assert_eq!(
            infer_tier_from_dataset_name("pool/hot/data"),
            StorageTier::Hot
        );
        assert_eq!(
            infer_tier_from_dataset_name("pool/cold/archive"),
            StorageTier::Cold
        );
        assert_eq!(
            infer_tier_from_dataset_name("pool/general"),
            StorageTier::Warm
        );
    }
}
