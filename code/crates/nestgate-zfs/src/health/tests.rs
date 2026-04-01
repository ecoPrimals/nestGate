// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::ZfsHealthMonitor;
use super::reporting::{dataset_health_from_zfs_list_text, pool_health_from_zpool_status_text};
use super::types::{Alert, AlertLevel, HealthReport, HealthStatus};
use crate::{config::ZfsConfig, dataset::ZfsDatasetManager, pool::ZfsPoolManager};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::SystemTime;

#[test]
fn test_health_status_is_critical_variants() {
    assert!(HealthStatus::Critical.is_critical());
    assert!(!HealthStatus::Healthy.is_critical());
}

#[test]
fn test_health_status_is_healthy_variants() {
    assert!(HealthStatus::Healthy.is_healthy());
    assert!(!HealthStatus::Critical.is_healthy());
}

#[test]
fn test_health_status_display_all() {
    assert_eq!(format!("{}", HealthStatus::Healthy), "Healthy");
    assert_eq!(format!("{}", HealthStatus::Warning), "Warning");
    assert_eq!(format!("{}", HealthStatus::Critical), "Critical");
    assert_eq!(format!("{}", HealthStatus::Unknown), "Unknown");
}

#[test]
fn test_health_report_serialization() {
    let report = HealthReport {
        component_type: "pool".to_string(),
        component_name: "tank".to_string(),
        status: HealthStatus::Healthy,
        last_check: SystemTime::now(),
        details: "OK".to_string(),
    };
    let json = serde_json::to_string(&report).unwrap();
    assert!(json.contains("tank"));
}

#[test]
fn test_alert_creation() {
    let alert = Alert {
        id: "a1".to_string(),
        level: AlertLevel::Info,
        message: "msg".to_string(),
        timestamp: SystemTime::now(),
        component: "pool".to_string(),
    };
    assert_eq!(alert.id, "a1");
    assert!(matches!(alert.level, AlertLevel::Info));
}

#[test]
fn test_health_monitor_new() {
    let config = ZfsConfig::default();
    let pool_manager = Arc::new(ZfsPoolManager::new_production(config.clone()));
    let dataset_manager = Arc::new(ZfsDatasetManager::new(config, pool_manager.clone()));
    let result = ZfsHealthMonitor::new(pool_manager, dataset_manager);
    assert!(result.is_ok());
}

#[test]
fn test_start_monitoring_idempotent() {
    let config = ZfsConfig::default();
    let pool_manager = Arc::new(ZfsPoolManager::new_production(config.clone()));
    let dataset_manager = Arc::new(ZfsDatasetManager::new(config, pool_manager.clone()));
    let mut monitor = ZfsHealthMonitor::new(pool_manager, dataset_manager).unwrap();
    let r1 = monitor.start_monitoring();
    assert!(r1.is_ok());
    let r2 = monitor.start_monitoring();
    assert!(r2.is_ok());
}

#[test]
fn alert_level_roundtrip_serde() {
    for level in [AlertLevel::Info, AlertLevel::Warning, AlertLevel::Critical] {
        let json = serde_json::to_string(&level).unwrap();
        let back: AlertLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(format!("{level:?}"), format!("{back:?}"));
    }
}

#[test]
fn health_status_partial_eq() {
    assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
    assert_ne!(HealthStatus::Healthy, HealthStatus::Critical);
}

#[test]
fn round5_health_status_display_impl() {
    assert_eq!(HealthStatus::Healthy.to_string(), "Healthy");
    assert_eq!(HealthStatus::Warning.to_string(), "Warning");
    assert_eq!(HealthStatus::Critical.to_string(), "Critical");
    assert_eq!(HealthStatus::Unknown.to_string(), "Unknown");
}

#[tokio::test]
async fn get_current_status_returns_enhanced_status() {
    let config = ZfsConfig::default();
    let pool_manager = Arc::new(ZfsPoolManager::new_production(config.clone()));
    let dataset_manager = Arc::new(ZfsDatasetManager::new(config, pool_manager.clone()));
    let monitor = ZfsHealthMonitor::new(pool_manager, dataset_manager).expect("monitor");
    let status = monitor.get_current_status().await.expect("status");
    assert!(matches!(
        status.overall_health,
        crate::manager::HealthState::Healthy
    ));
}

#[tokio::test]
async fn stop_monitoring_when_inactive_is_ok() {
    let config = ZfsConfig::default();
    let pool_manager = Arc::new(ZfsPoolManager::new_production(config.clone()));
    let dataset_manager = Arc::new(ZfsDatasetManager::new(config, pool_manager.clone()));
    let mut monitor = ZfsHealthMonitor::new(pool_manager, dataset_manager).expect("monitor");
    monitor.stop_monitoring().await.expect("stop");
}

#[tokio::test]
async fn stop_monitoring_when_active_drains_background_tasks() {
    let config = ZfsConfig::default();
    let pool_manager = Arc::new(ZfsPoolManager::new_production(config.clone()));
    let dataset_manager = Arc::new(ZfsDatasetManager::new(config, pool_manager.clone()));
    let mut monitor = ZfsHealthMonitor::new(pool_manager, dataset_manager).expect("monitor");
    monitor.start_monitoring().expect("start monitoring");
    let long = tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    });
    {
        let mut tasks = monitor.background_tasks.write().await;
        tasks.push(long);
    }
    monitor.stop_monitoring().await.expect("stop");
    assert!(!monitor.monitoring_active.load(Ordering::Relaxed));
}

#[tokio::test]
async fn health_monitor_start_then_stop_clears_health_data() {
    let config = ZfsConfig::default();
    let pool_manager = Arc::new(ZfsPoolManager::new_production(config.clone()));
    let dataset_manager = Arc::new(ZfsDatasetManager::new(config, pool_manager.clone()));
    let mut monitor = ZfsHealthMonitor::new(pool_manager, dataset_manager).expect("monitor");
    monitor.start().expect("start");
    assert!(monitor.monitoring_tasks.is_some());
    {
        let mut map = monitor.health_data.write().await;
        map.insert(
            "probe".to_string(),
            HealthReport {
                component_type: "pool".to_string(),
                component_name: "p".to_string(),
                status: HealthStatus::Healthy,
                last_check: SystemTime::now(),
                details: "test".to_string(),
            },
        );
    }
    monitor.stop().await.expect("stop");
    assert!(monitor.monitoring_tasks.is_none());
    let map = monitor.health_data.read().await;
    assert!(map.is_empty());
}

#[tokio::test]
async fn check_pool_health_invokes_zpool_for_unknown_pool() {
    let config = ZfsConfig::default();
    let pool_manager = Arc::new(ZfsPoolManager::new_production(config.clone()));
    let status =
        ZfsHealthMonitor::check_pool_health(&pool_manager, "nestgate_nonexistent_pool_🦀").await;
    assert!(matches!(
        status,
        HealthStatus::Healthy | HealthStatus::Warning | HealthStatus::Critical
    ));
}

#[tokio::test]
async fn check_dataset_health_invokes_zfs_for_unknown_pool() {
    let config = ZfsConfig::default();
    let pool_manager = Arc::new(ZfsPoolManager::new_production(config.clone()));
    let dataset_manager = Arc::new(ZfsDatasetManager::new(config, pool_manager.clone()));
    let status = ZfsHealthMonitor::check_dataset_health(
        &dataset_manager,
        "nestgate_nonexistent_dataset_root_🦀",
    )
    .await;
    assert!(matches!(
        status,
        HealthStatus::Healthy | HealthStatus::Warning | HealthStatus::Critical
    ));
}

#[test]
fn pool_health_text_online_and_no_errors_is_healthy() {
    assert_eq!(
        pool_health_from_zpool_status_text("  state: ONLINE\n"),
        HealthStatus::Healthy
    );
}

#[test]
fn pool_health_text_online_with_errors_colon_is_not_healthy() {
    assert_eq!(
        pool_health_from_zpool_status_text("ONLINE\nerrors: 42"),
        HealthStatus::Warning
    );
}

#[test]
fn pool_health_text_degraded_faulted_unavail_are_critical() {
    assert_eq!(
        pool_health_from_zpool_status_text("pool: DEGRADED\n"),
        HealthStatus::Critical
    );
    assert_eq!(
        pool_health_from_zpool_status_text("  FAULTED  \n"),
        HealthStatus::Critical
    );
    assert_eq!(
        pool_health_from_zpool_status_text("disk UNAVAIL\n"),
        HealthStatus::Critical
    );
}

#[test]
fn pool_health_text_unknown_without_keywords_is_warning() {
    assert_eq!(
        pool_health_from_zpool_status_text("no useful tokens here"),
        HealthStatus::Warning
    );
}

#[test]
fn pool_health_text_online_branch_checked_before_degraded_keywords() {
    // `contains("ONLINE") && !errors:` is evaluated first; both ONLINE and DEGRADED in text
    // still match the healthy branch when `errors:` is absent.
    let s = "ONLINE replica but DEGRADED due to missing disk";
    assert_eq!(pool_health_from_zpool_status_text(s), HealthStatus::Healthy);
}

#[test]
fn dataset_health_empty_stdout_is_healthy() {
    assert_eq!(dataset_health_from_zfs_list_text(""), HealthStatus::Healthy);
    assert_eq!(
        dataset_health_from_zfs_list_text("\n\n"),
        HealthStatus::Healthy
    );
}

#[test]
fn dataset_health_all_high_avail_is_healthy() {
    let two_gib = 2u64 * 1024 * 1024 * 1024;
    let line = format!("tank/a\t{two_gib}\ntank/b\t{two_gib}");
    assert_eq!(
        dataset_health_from_zfs_list_text(&line),
        HealthStatus::Healthy
    );
}

#[test]
fn dataset_health_all_low_avail_is_critical() {
    let line = "a\t100\nb\t200\nc\t300";
    assert_eq!(
        dataset_health_from_zfs_list_text(line),
        HealthStatus::Critical
    );
}

#[test]
fn dataset_health_threshold_warning_one_low_of_five() {
    let two_gib = 2u64 * 1024 * 1024 * 1024;
    let line = format!("a\t100\nb\t{two_gib}\nc\t{two_gib}\nd\t{two_gib}\ne\t{two_gib}");
    assert_eq!(
        dataset_health_from_zfs_list_text(&line),
        HealthStatus::Warning
    );
}

#[test]
fn dataset_health_threshold_critical_two_low_of_four() {
    let line = "a\t100\nb\t200\nc\t300\nd\t400";
    assert_eq!(
        dataset_health_from_zfs_list_text(line),
        HealthStatus::Critical
    );
}

#[test]
fn dataset_health_unparseable_avail_counts_row_but_not_low() {
    let two_gib = 2u64 * 1024 * 1024 * 1024;
    let line = format!("tank/x\tnot-a-number\ntank/y\t{two_gib}");
    assert_eq!(
        dataset_health_from_zfs_list_text(&line),
        HealthStatus::Healthy
    );
}

#[test]
fn dataset_health_single_low_dataset_is_critical() {
    assert_eq!(
        dataset_health_from_zfs_list_text("only\t512"),
        HealthStatus::Critical
    );
}

#[test]
fn health_status_warning_unknown_neither_healthy_nor_critical() {
    assert!(!HealthStatus::Warning.is_healthy());
    assert!(!HealthStatus::Warning.is_critical());
    assert!(!HealthStatus::Unknown.is_healthy());
    assert!(!HealthStatus::Unknown.is_critical());
}

#[test]
fn health_report_and_alert_defaults_constructors() {
    let r = HealthReport {
        component_type: String::new(),
        component_name: String::new(),
        status: HealthStatus::Unknown,
        last_check: SystemTime::UNIX_EPOCH,
        details: String::new(),
    };
    assert_eq!(r.status, HealthStatus::Unknown);
    let a = Alert {
        id: String::new(),
        level: AlertLevel::Warning,
        message: String::new(),
        timestamp: SystemTime::UNIX_EPOCH,
        component: String::new(),
    };
    assert!(matches!(a.level, AlertLevel::Warning));
}
