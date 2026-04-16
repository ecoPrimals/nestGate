// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unit tests for workspace lifecycle handlers and config types.

use axum::extract::{Json, Path};
use axum::http::StatusCode;

use super::lifecycle::{
    BackupConfig, MigrationConfig, MigrationStrategy, RestoreConfig, backup_workspace,
    list_workspace_backups, migrate_workspace, restore_workspace,
};

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

fn sample_migration_config() -> MigrationConfig {
    MigrationConfig {
        target_pool: "pool2".to_string(),
        target_host: None,
        strategy: MigrationStrategy::Copy,
        bandwidth_limit: None,
    }
}

#[tokio::test]
async fn backup_workspace_rejects_invalid_ids() {
    let cfg = sample_backup_config();
    for bad in ["", "a/b", "has space"] {
        let r = backup_workspace(Path(bad.to_string()), Json(cfg.clone()))
            .await
            .expect_err("bad request");
        assert_eq!(r, StatusCode::BAD_REQUEST);
    }
}

#[tokio::test]
async fn restore_workspace_rejects_invalid_ids() {
    let cfg = sample_restore_config();
    for bad in ["", "x/y", "bad id"] {
        let r = restore_workspace(Path(bad.to_string()), Json(cfg.clone()))
            .await
            .expect_err("bad request");
        assert_eq!(r, StatusCode::BAD_REQUEST);
    }
}

#[tokio::test]
async fn migrate_workspace_rejects_invalid_ids() {
    let cfg = sample_migration_config();
    for bad in ["", "pool/x", " "] {
        let r = migrate_workspace(Path(bad.to_string()), Json(cfg.clone()))
            .await
            .expect_err("bad request");
        assert_eq!(r, StatusCode::BAD_REQUEST);
    }
}

#[tokio::test]
async fn list_workspace_backups_returns_success_json() {
    let res = list_workspace_backups(Path("ws1".to_string()))
        .await
        .expect("ok");
    let v = res.0;
    assert_eq!(v["status"], "success");
    assert_eq!(v["workspace_id"], "ws1");
    assert!(v.get("backups").is_some());
}

#[test]
fn migration_strategy_roundtrip_serde() {
    for s in [
        MigrationStrategy::Copy,
        MigrationStrategy::Move,
        MigrationStrategy::Replicate,
    ] {
        let j = serde_json::to_string(&s).unwrap();
        let back: MigrationStrategy = serde_json::from_str(&j).unwrap();
        assert_eq!(s, back);
    }
}

#[test]
fn backup_restore_migration_config_serde() {
    let b = BackupConfig {
        backup_name: "n".into(),
        include_snapshots: true,
        compression_level: 9,
        encryption_enabled: true,
        description: Some("d".into()),
    };
    let b2: BackupConfig = serde_json::from_str(&serde_json::to_string(&b).unwrap()).unwrap();
    assert_eq!(b2.compression_level, 9);

    let r = RestoreConfig {
        backup_name: "n".into(),
        target_workspace_id: Some("t".into()),
        restore_point: Some("@snap".into()),
        force: true,
    };
    let _: RestoreConfig = serde_json::from_str(&serde_json::to_string(&r).unwrap()).unwrap();

    let m = sample_migration_config();
    let _: MigrationConfig = serde_json::from_str(&serde_json::to_string(&m).unwrap()).unwrap();
}
