// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::super::types::{
    AlertSeverity, AlertType, BottleneckSeverity, OptimizationState, PerformanceAlert,
    PerformanceOptimizationResult, ZfsBottleneckType, ZfsPerformanceMetrics, ZfsPoolMetrics,
};
use super::PerformanceOptimizationEngine;
use crate::config::ZfsConfig;
use crate::dataset::ZfsDatasetManager;
use crate::pool::ZfsPoolManager;
use std::sync::Arc;
use std::time::SystemTime;

fn test_engine() -> PerformanceOptimizationEngine {
    let config = ZfsConfig::default();
    let pool_manager = Arc::new(ZfsPoolManager::new_for_testing());
    let dataset_manager = Arc::new(ZfsDatasetManager::new(
        config.clone(),
        Arc::clone(&pool_manager),
    ));
    PerformanceOptimizationEngine::new(config, pool_manager, dataset_manager)
}

async fn seed_metrics(engine: &PerformanceOptimizationEngine, m: ZfsPerformanceMetrics) {
    let cache = engine.performance_monitor.get_metrics_cache();
    let mut w = cache.write().await;
    w.clear();
    w.insert("latest".into(), m);
}

fn metrics_with_pool_issue(latency: f64, cache_hit: f64, frag: f64) -> ZfsPerformanceMetrics {
    let mut m = ZfsPerformanceMetrics::default();
    m.pool_metrics.insert(
        "tank".into(),
        ZfsPoolMetrics {
            pool_name: "tank".into(),
            read_ops: 10.0,
            write_ops: 10.0,
            read_bandwidth: 1.0,
            write_bandwidth: 1.0,
            latency,
            cache_hit_ratio: cache_hit,
            fragmentation: frag,
        },
    );
    m.arc_stats.hit_ratio = 0.5;
    m.system_memory.total = 100;
    m.system_memory.used = 95;
    m.system_memory.available = 5;
    m
}

#[test]
fn optimization_state_default_is_idle() {
    assert_eq!(OptimizationState::default(), OptimizationState::Idle);
}

#[tokio::test]
async fn tune_zfs_parameters_returns_changes() {
    let engine = test_engine();
    let r = engine.tune_zfs_parameters("tank/fs").expect("tune");
    assert!(r.tuning_applied);
    assert!(r.validation_required);
    assert!(!r.parameter_changes.is_empty());
}

#[tokio::test]
async fn handle_performance_alert_returns_response() {
    let engine = test_engine();
    let alert = PerformanceAlert {
        alert_type: AlertType::BottleneckDetected,
        severity: AlertSeverity::Warning,
        pool_name: "p".into(),
        dataset_name: None,
        description: "d".into(),
        timestamp: SystemTime::UNIX_EPOCH,
    };
    let r = engine.handle_performance_alert(alert).expect("handle");
    assert!(r.mitigation_applied);
}

#[tokio::test]
async fn get_trending_data_empty() {
    let engine = test_engine();
    let rows = engine.get_trending_data().await.expect("trending");
    assert!(rows.is_empty());
}

#[tokio::test]
async fn detect_bottlenecks_with_seeded_hot_pool() {
    let engine = test_engine();
    seed_metrics(&engine, metrics_with_pool_issue(120.0, 0.5, 60.0)).await;
    let b = engine.test_detect_bottlenecks().await.expect("bottlenecks");
    assert!(!b.is_empty());
    assert!(
        b.iter()
            .any(|x| matches!(x.bottleneck_type, ZfsBottleneckType::HighLatency))
    );
}

#[tokio::test]
async fn optimize_performance_applies_for_detected_issues() {
    let engine = test_engine();
    seed_metrics(&engine, metrics_with_pool_issue(80.0, 0.7, 40.0)).await;
    let result = engine.optimize_performance().await.expect("optimize");
    assert!(!result.recommendations.is_empty());
}

#[test]
fn zfs_bottleneck_type_serde_roundtrip() {
    for t in [
        ZfsBottleneckType::HighLatency,
        ZfsBottleneckType::LowThroughput,
        ZfsBottleneckType::CacheMiss,
        ZfsBottleneckType::Fragmentation,
        ZfsBottleneckType::MemoryPressure,
        ZfsBottleneckType::CpuUtilization,
        ZfsBottleneckType::NetworkBandwidth,
        ZfsBottleneckType::DiskIo,
    ] {
        let j = serde_json::to_string(&t).unwrap();
        let back: ZfsBottleneckType = serde_json::from_str(&j).unwrap();
        assert_eq!(format!("{t:?}"), format!("{back:?}"));
    }
}

#[test]
fn bottleneck_severity_serde_roundtrip() {
    for s in [
        BottleneckSeverity::Low,
        BottleneckSeverity::Medium,
        BottleneckSeverity::High,
        BottleneckSeverity::Critical,
    ] {
        let j = serde_json::to_string(&s).unwrap();
        let back: BottleneckSeverity = serde_json::from_str(&j).unwrap();
        assert_eq!(s, back);
    }
}

#[test]
fn performance_optimization_result_merge_combines() {
    let mut a = PerformanceOptimizationResult {
        performance_improvement: 1.0,
        recommendations: vec!["a".into()],
        ..Default::default()
    };
    let b = PerformanceOptimizationResult {
        performance_improvement: 2.0,
        recommendations: vec!["b".into()],
        ..Default::default()
    };
    a.merge_with(b);
    assert!((a.performance_improvement - 3.0).abs() < f64::EPSILON);
    assert_eq!(a.recommendations.len(), 2);
}
