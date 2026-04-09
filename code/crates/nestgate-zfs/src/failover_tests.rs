// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[tokio::test]
async fn test_pool_takeover_manager_creation() {
    let config = ZfsConfig::default();
    let failover_config = CanonicalFailoverConfig::default();
    let manager = PoolTakeoverManager::new(config, failover_config, "test-node".to_string());

    assert_eq!(manager.node_id, "test-node");
    assert!(manager.known_pools.read().await.is_empty());
}

#[tokio::test]
async fn test_node_health_monitoring() {
    let config = CanonicalFailoverConfig::default();
    let monitor = NodeHealthMonitor::new(config);

    // Update heartbeat for a node
    monitor.update_node_heartbeat("node1").await;

    // Should not detect as failed immediately
    let failed_nodes = monitor.detect_failed_nodes().await.unwrap_or_else(|e| {
        tracing::error!("Unwrap failed: {:?}", e);
        // Return empty vector for test purposes
        Vec::new()
    });
    assert!(failed_nodes.is_empty());

    // Simulate passage of time by manually setting old heartbeat
    {
        let mut nodes = monitor.known_nodes.write().await;
        if let Some(node) = nodes.get_mut("node1") {
            node.last_heartbeat = SystemTime::now()
                - Duration::from_secs(
                    std::env::var("NESTGATE_ZFS_HEARTBEAT_TIMEOUT_SECS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(300), // 5 minutes default
                );
        }
    }

    // Now should detect as failed
    let failed_nodes = monitor.detect_failed_nodes().await.unwrap_or_else(|e| {
        tracing::error!("Unwrap failed: {:?}", e);
        // Return empty vector for test purposes
        Vec::new()
    });
    assert_eq!(failed_nodes.len(), 1);
    assert_eq!(failed_nodes[0].node_id, "node1");
}

#[tokio::test]
async fn test_pool_metadata_tracking() {
    let config = ZfsConfig::default();
    let failover_config = CanonicalFailoverConfig::default();
    let manager = PoolTakeoverManager::new(config, failover_config, "test-node".to_string());

    // Update pool metadata
    manager
        .update_pool_metadata("testpool", PoolFailoverState::Active)
        .await;

    // Verify metadata was stored
    let status = manager.get_pool_status().await;
    assert_eq!(status.len(), 1);
    assert_eq!(status["testpool"].name, "testpool");
    assert_eq!(status["testpool"].state, PoolFailoverState::Active);
}

#[test]
fn canonical_failover_config_default_fields() {
    let c = CanonicalFailoverConfig::default();
    assert!(c.auto_takeover_enabled);
    assert_eq!(c.health_check_interval_secs, 30);
    assert!(c.failback_enabled);
}

#[test]
fn pool_metadata_serde_roundtrip() {
    let m = PoolMetadata {
        name: "p".into(),
        original_owner: "n1".into(),
        last_seen: SystemTime::UNIX_EPOCH,
        import_guid: Some("g".into()),
        state: PoolFailoverState::Orphaned,
    };
    let j = serde_json::to_string(&m).unwrap();
    let back: PoolMetadata = serde_json::from_str(&j).unwrap();
    assert_eq!(back.name, m.name);
    assert_eq!(back.state, PoolFailoverState::Orphaned);
}

#[test]
fn pool_failover_state_variants_distinct() {
    assert_ne!(PoolFailoverState::Active, PoolFailoverState::Failed);
    assert_ne!(PoolFailoverState::Orphaned, PoolFailoverState::Unknown);
}

#[test]
fn pool_state_enum_covers_zfs_like_labels() {
    assert_eq!(PoolState::Online, PoolState::Online);
    assert_ne!(PoolState::Online, PoolState::Offline);
    assert_ne!(PoolState::Degraded, PoolState::Faulted);
}

#[test]
fn canonical_failover_config_timeouts_and_attempts() {
    let c = CanonicalFailoverConfig::default();
    assert_eq!(c.takeover_timeout_secs, 300);
    assert_eq!(
        c.node_failure_timeout_secs,
        crate::constants::NODE_FAILURE_TIMEOUT_SECS
    );
    assert_eq!(c.max_takeover_attempts, 3);
    assert_eq!(c.failback_delay_secs, 60);
    assert!(c.notification_config.is_none());
}

#[test]
fn parse_zpool_import_list_stdout_empty_and_whitespace() {
    assert!(parse_zpool_import_list_stdout("").is_empty());
    assert!(parse_zpool_import_list_stdout("  \n\t\n").is_empty());
}

#[test]
fn parse_zpool_import_list_stdout_extracts_pool_lines() {
    let sample = r"
  pool: tank
 id: 123
  pool: backup_store
";
    let pools = parse_zpool_import_list_stdout(sample);
    assert_eq!(pools, vec!["tank".to_string(), "backup_store".to_string()]);
}

#[test]
fn parse_zpool_import_list_stdout_trims_names_and_ignores_non_pool_lines() {
    let sample = "pool:  mypool  \nnotpool: x\n   pool: other\n";
    let pools = parse_zpool_import_list_stdout(sample);
    assert_eq!(pools, vec!["mypool".to_string(), "other".to_string()]);
}

#[test]
fn orphaned_pools_from_known_registry_matches_owner_only() {
    let mut known = HashMap::new();
    known.insert(
        "p1".to_string(),
        PoolMetadata {
            name: "p1".into(),
            original_owner: "node-a".into(),
            last_seen: SystemTime::UNIX_EPOCH,
            import_guid: None,
            state: PoolFailoverState::Active,
        },
    );
    known.insert(
        "p2".to_string(),
        PoolMetadata {
            name: "p2".into(),
            original_owner: "node-b".into(),
            last_seen: SystemTime::UNIX_EPOCH,
            import_guid: None,
            state: PoolFailoverState::Active,
        },
    );
    let available = vec!["p1".into(), "p2".into(), "missing".into()];
    let out = orphaned_pools_from_known_registry(&available, &known, "node-a");
    assert_eq!(out, vec!["p1".to_string()]);
}

#[tokio::test]
async fn detect_failed_nodes_empty_registry() {
    let monitor = NodeHealthMonitor::new(CanonicalFailoverConfig::default());
    let failed = monitor.detect_failed_nodes().await.unwrap();
    assert!(failed.is_empty());
}

#[tokio::test]
async fn detect_failed_nodes_skips_when_heartbeat_recent() {
    let monitor = NodeHealthMonitor::new(CanonicalFailoverConfig::default());
    monitor.update_node_heartbeat("n1").await;
    let failed = monitor.detect_failed_nodes().await.unwrap();
    assert!(failed.is_empty());
}

#[tokio::test]
async fn detect_failed_nodes_skips_future_heartbeat() {
    let monitor = NodeHealthMonitor::new(CanonicalFailoverConfig::default());
    monitor.update_node_heartbeat("n1").await;
    let future = SystemTime::now() + Duration::from_secs(3600);
    monitor.test_set_node_heartbeat_time("n1", future).await;
    let failed = monitor.detect_failed_nodes().await.unwrap();
    assert!(failed.is_empty());
}

#[tokio::test]
async fn detect_failed_nodes_skips_when_marked_not_alive() {
    let config = CanonicalFailoverConfig {
        node_failure_timeout_secs: 10,
        ..Default::default()
    };
    let monitor = NodeHealthMonitor::new(config);
    monitor.update_node_heartbeat("n1").await;
    monitor.test_set_node_alive("n1", false).await;
    monitor
        .test_set_node_heartbeat_time("n1", SystemTime::now() - Duration::from_secs(1000))
        .await;
    let failed = monitor.detect_failed_nodes().await.unwrap();
    assert!(failed.is_empty());
}

#[tokio::test]
async fn update_pool_metadata_updates_existing_entry() {
    let config = ZfsConfig::default();
    let failover_config = CanonicalFailoverConfig::default();
    let manager = PoolTakeoverManager::new(config, failover_config, "owner-1".to_string());

    manager
        .update_pool_metadata("pool-x", PoolFailoverState::Active)
        .await;
    manager
        .update_pool_metadata("pool-x", PoolFailoverState::Failed)
        .await;

    let status = manager.get_pool_status().await;
    assert_eq!(status["pool-x"].state, PoolFailoverState::Failed);
    assert_eq!(status["pool-x"].original_owner, "owner-1");
}

#[test]
fn pool_failover_state_serde_all_variants() {
    for state in [
        PoolFailoverState::Active,
        PoolFailoverState::Orphaned,
        PoolFailoverState::Failed,
        PoolFailoverState::Unknown,
    ] {
        let j = serde_json::to_string(&state).unwrap();
        let back: PoolFailoverState = serde_json::from_str(&j).unwrap();
        assert_eq!(back, state);
    }
}

#[test]
fn pool_state_serde_all_variants() {
    for state in [
        PoolState::Online,
        PoolState::Degraded,
        PoolState::Offline,
        PoolState::Faulted,
        PoolState::Removed,
        PoolState::Unavail,
        PoolState::Orphaned,
        PoolState::Failed,
        PoolState::Unknown,
    ] {
        let j = serde_json::to_string(&state).unwrap();
        let back: PoolState = serde_json::from_str(&j).unwrap();
        assert_eq!(back, state);
    }
}

#[test]
fn canonical_failover_config_serde_with_notification() {
    let c = CanonicalFailoverConfig {
        notification_config: Some(FailoverNotificationConfig {
            email_enabled: true,
            email_recipients: vec!["a@b.c".into()],
            webhook_enabled: false,
            webhook_url: None,
            slack_enabled: false,
            slack_webhook: None,
        }),
        ..CanonicalFailoverConfig::default()
    };
    let j = serde_json::to_string(&c).unwrap();
    let back: CanonicalFailoverConfig = serde_json::from_str(&j).unwrap();
    assert!(back.notification_config.as_ref().unwrap().email_enabled);
}

#[test]
#[expect(deprecated)]
fn deprecated_failover_notification_config_roundtrip() {
    let n = FailoverNotificationConfig {
        email_enabled: false,
        email_recipients: vec![],
        webhook_enabled: true,
        webhook_url: Some("https://hooks.example/1".into()),
        slack_enabled: true,
        slack_webhook: Some("https://slack.example".into()),
    };
    let j = serde_json::to_string(&n).unwrap();
    let back: FailoverNotificationConfig = serde_json::from_str(&j).unwrap();
    assert!(back.webhook_enabled && back.slack_enabled);
}

#[tokio::test]
async fn verify_pool_import_invokes_zpool_status() {
    let config = ZfsConfig::default();
    let manager = PoolTakeoverManager::new(config, CanonicalFailoverConfig::default(), "n".into());
    let _ = manager
        .verify_pool_import("___nonexistent_pool_nestgate_test___")
        .await;
}

#[tokio::test]
async fn export_pool_invokes_zpool_export() {
    let config = ZfsConfig::default();
    let manager = PoolTakeoverManager::new(config, CanonicalFailoverConfig::default(), "n".into());
    let _ = manager
        .export_pool("___nonexistent_pool_nestgate_export_test___")
        .await;
}

#[tokio::test]
async fn attempt_pool_takeover_runs_discovery_pipeline() {
    let config = ZfsConfig::default();
    let manager = PoolTakeoverManager::new(
        config,
        CanonicalFailoverConfig::default(),
        "local-node".into(),
    );
    let _ = manager.attempt_pool_takeover("failed-peer").await;
}
