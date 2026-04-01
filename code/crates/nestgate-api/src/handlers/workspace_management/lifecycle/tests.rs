// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#[allow(deprecated)]
use super::{
    BackupConfig, MigrationConfig, MigrationStrategy, RestoreConfig, backup_workspace,
    migrate_workspace, restore_workspace,
};
use axum::extract::{Json, Path};
use axum::http::StatusCode;

fn sample_backup_config() -> BackupConfig {
    BackupConfig {
        backup_name: "b1".to_string(),
        include_snapshots: false,
        compression_level: 0,
        encryption_enabled: false,
        description: None,
    }
}

fn sample_restore_config() -> RestoreConfig {
    RestoreConfig {
        backup_name: "b1".to_string(),
        target_workspace_id: None,
        restore_point: None,
        force: false,
    }
}

#[test]
fn migration_strategy_roundtrips_json() {
    let m = MigrationConfig {
        target_pool: "p".to_string(),
        target_host: None,
        strategy: MigrationStrategy::Replicate,
        bandwidth_limit: Some(1024),
    };
    let v = serde_json::to_value(&m).unwrap();
    let back: MigrationConfig = serde_json::from_value(v).unwrap();
    assert_eq!(back.strategy, MigrationStrategy::Replicate);
    assert_eq!(back.target_pool, "p");
}

#[tokio::test]
async fn backup_workspace_invalid_id_returns_bad_request() {
    let r = backup_workspace(Path(String::new()), Json(sample_backup_config())).await;
    assert!(matches!(r, Err(StatusCode::BAD_REQUEST)));
    let r = backup_workspace(Path("bad/id".to_string()), Json(sample_backup_config())).await;
    assert!(matches!(r, Err(StatusCode::BAD_REQUEST)));
}

#[tokio::test]
async fn restore_workspace_invalid_id_returns_bad_request() {
    let r = restore_workspace(Path(String::new()), Json(sample_restore_config())).await;
    assert!(matches!(r, Err(StatusCode::BAD_REQUEST)));
}

#[tokio::test]
async fn migrate_workspace_invalid_id_returns_bad_request() {
    let cfg = MigrationConfig {
        target_pool: "t".to_string(),
        target_host: None,
        strategy: MigrationStrategy::Copy,
        bandwidth_limit: None,
    };
    let r = migrate_workspace(Path("bad id".to_string()), Json(cfg)).await;
    assert!(matches!(r, Err(StatusCode::BAD_REQUEST)));
}
