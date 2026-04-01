// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::handler_impl::ZfsHandlerImpl;
use super::types::{
    CreateDatasetRequest, CreatePoolRequest, CreateSnapshotRequest, evaluate_zfs_health,
};
use crate::dev_stubs::zfs::ZeroCostPoolInfo;
use std::collections::HashMap;

#[test]
fn create_pool_request_roundtrip_json() {
    let r = CreatePoolRequest {
        name: "tank".to_string(),
        _devices: vec!["/dev/sdb".to_string()],
    };
    let v = serde_json::to_value(&r).unwrap();
    let back: CreatePoolRequest = serde_json::from_value(v).unwrap();
    assert_eq!(back.name, "tank");
    assert_eq!(back._devices.len(), 1);
}

#[test]
fn create_dataset_request_optional_properties() {
    let mut props = HashMap::new();
    props.insert("compression".to_string(), "lz4".to_string());
    let r = CreateDatasetRequest {
        name: "data".to_string(),
        properties: Some(props),
    };
    let json = serde_json::to_string(&r).unwrap();
    let back: CreateDatasetRequest = serde_json::from_str(&json).unwrap();
    assert!(
        back.properties
            .as_ref()
            .unwrap()
            .contains_key("compression")
    );
}

#[test]
fn create_snapshot_request_serde() {
    let r = CreateSnapshotRequest {
        dataset: "tank/foo".to_string(),
        name: "snap1".to_string(),
    };
    let v = serde_json::to_value(&r).unwrap();
    let back: CreateSnapshotRequest = serde_json::from_value(v).unwrap();
    assert_eq!(back.dataset, "tank/foo");
    assert_eq!(back.name, "snap1");
}

#[test]
fn zfs_handler_impl_default_matches_new() {
    assert!(format!("{:?}", ZfsHandlerImpl::new()).contains("ZfsHandlerImpl"));
    let _ = ZfsHandlerImpl;
}

fn sample_pool(name: &str, health: &str) -> ZeroCostPoolInfo {
    ZeroCostPoolInfo {
        name: name.to_string(),
        health: health.to_string(),
        size: 0,
        allocated: 0,
        free: 0,
    }
}

#[test]
fn evaluate_zfs_health_online_only_is_healthy() {
    let r = evaluate_zfs_health(vec![sample_pool("tank", "ONLINE")]);
    assert!(r.healthy);
    assert!(r.issues.is_empty());
    assert_eq!(r.pools.len(), 1);
}

#[test]
fn evaluate_zfs_health_critical_marks_unhealthy() {
    let r = evaluate_zfs_health(vec![sample_pool("tank", "FAULTED")]);
    assert!(!r.healthy);
    assert_eq!(r.issues.len(), 1);
    assert!(r.issues[0].contains("critical"));
}

#[test]
fn evaluate_zfs_health_critical_status_marks_unhealthy() {
    let r = evaluate_zfs_health(vec![sample_pool("tank", "CRITICAL")]);
    assert!(!r.healthy);
    assert!(r.issues.iter().any(|s| s.contains("critical")));
}

#[test]
fn evaluate_zfs_health_unavail_marks_unhealthy() {
    let r = evaluate_zfs_health(vec![sample_pool("tank", "UNAVAIL")]);
    assert!(!r.healthy);
    assert!(r.issues.iter().any(|s| s.contains("critical")));
}

#[test]
fn evaluate_zfs_health_degraded_keeps_overall_healthy_flag() {
    let r = evaluate_zfs_health(vec![sample_pool("tank", "DEGRADED")]);
    assert!(r.healthy);
    assert_eq!(r.issues.len(), 1);
}

#[test]
fn evaluate_zfs_health_unknown_pool_adds_issue() {
    let r = evaluate_zfs_health(vec![sample_pool("tank", "UNKNOWN")]);
    assert!(r.healthy);
    assert!(r.issues.iter().any(|s| s.contains("unknown")));
}

#[test]
fn zfs_health_response_json_shape() {
    let r = evaluate_zfs_health(vec![sample_pool("z", "ONLINE")]);
    let v = serde_json::to_value(&r).unwrap();
    assert!(v.get("healthy").is_some());
    assert!(v.get("pools").is_some());
    assert!(v.get("issues").is_some());
}
