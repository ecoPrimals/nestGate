// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unit tests for diagnostics types and manager (library-only, no I/O).

use super::{
    ComponentType, Diagnostic, DiagnosticLevel, DiagnosticsManager, DiskMetrics, NetworkMetrics,
    SystemMetrics,
};
use nestgate_types::unified_enums::UnifiedServiceState;

#[test]
fn diagnostic_level_display() {
    assert_eq!(DiagnosticLevel::Info.to_string(), "INFO");
    assert_eq!(DiagnosticLevel::Critical.to_string(), "CRITICAL");
}

#[test]
fn component_type_display() {
    for (ct, expected) in [
        (ComponentType::Cpu, "CPU"),
        (ComponentType::Memory, "Memory"),
        (ComponentType::Storage, "Storage"),
        (ComponentType::Network, "Network"),
        (ComponentType::System, "System"),
        (ComponentType::Application, "Application"),
        (ComponentType::Database, "Database"),
        (ComponentType::Cache, "Cache"),
    ] {
        assert_eq!(ct.to_string(), expected);
    }
}

#[test]
fn diagnostic_builders_resolve_and_age() {
    let d = Diagnostic::warning(ComponentType::Network, "latency".into());
    assert!(d.is_unresolved());
    assert!(!d.is_severe());
    let mut crit = Diagnostic::critical(ComponentType::System, "down".into());
    assert!(crit.is_severe());
    crit.resolve();
    assert!(!crit.is_unresolved());
    assert!(crit.resolved_at.is_some());

    let info = Diagnostic::info(ComponentType::Application, "ok".into())
        .with_details("extra".into())
        .with_resource("/cfg");
    assert_eq!(info.details.as_deref(), Some("extra"));
    assert_eq!(info.path.as_deref(), Some("/cfg"));
    assert!(info.age_seconds() <= 1);
}

#[test]
fn diagnostics_manager_empty_is_healthy() {
    let mgr = DiagnosticsManager::new();
    assert!(mgr.get_diagnostics().unwrap().is_empty());
    assert_eq!(
        mgr.calculate_health_status().unwrap(),
        nestgate_types::unified_enums::UnifiedHealthStatus::Healthy
    );
    assert!(mgr.get_unresolved_diagnostics().unwrap().is_empty());
    assert!(mgr.get_metrics().unwrap().memory_total > 0);
    assert!(mgr.update_metrics(SystemMetrics::default()).is_ok());
    assert!(mgr.clear_resolved().unwrap() == 0);
}

#[test]
fn diagnostics_subscribe_channel_exists() {
    let mgr = DiagnosticsManager::new();
    let mut rx = mgr.subscribe();
    assert!(rx.try_recv().is_err());
}

#[test]
fn diagnostics_clear_resolved_removes_resolved_entries() {
    let mgr = DiagnosticsManager::new();
    let mut d = Diagnostic::info(ComponentType::System, "note".into());
    d.resolve();
    mgr.add_diagnostic(d).unwrap();
    assert_eq!(mgr.clear_resolved().unwrap(), 1);
    assert!(mgr.get_diagnostics().unwrap().is_empty());
}

#[test]
fn diagnostics_unresolved_info_only_stays_healthy() {
    let mgr = DiagnosticsManager::new();
    mgr.add_diagnostic(Diagnostic::info(ComponentType::Application, "i".into()))
        .unwrap();
    assert_eq!(
        mgr.calculate_health_status().unwrap(),
        nestgate_types::unified_enums::UnifiedHealthStatus::Healthy
    );
}

#[test]
fn diagnostics_manager_unresolved_severity_order() {
    let mgr = DiagnosticsManager::new();
    mgr.add_diagnostic(Diagnostic::warning(ComponentType::Storage, "w".into()))
        .unwrap();
    assert_eq!(
        mgr.calculate_health_status().unwrap(),
        nestgate_types::unified_enums::UnifiedHealthStatus::Warning
    );

    let mgr = DiagnosticsManager::new();
    mgr.add_diagnostic(Diagnostic::error(ComponentType::Cache, "e".into()))
        .unwrap();
    assert_eq!(
        mgr.calculate_health_status().unwrap(),
        nestgate_types::unified_enums::UnifiedHealthStatus::Error
    );

    let mgr = DiagnosticsManager::new();
    mgr.add_diagnostic(Diagnostic::critical(ComponentType::Memory, "c".into()))
        .unwrap();
    assert_eq!(
        mgr.calculate_health_status().unwrap(),
        nestgate_types::unified_enums::UnifiedHealthStatus::Critical
    );
}

#[test]
fn types_service_info_default_and_system_metrics_default() {
    let s = super::types::ServiceInfo::default();
    assert_eq!(s.name, "unknown");
    let m = SystemMetrics::default();
    assert_eq!(m.process_count, 0);
}

#[test]
fn disk_and_network_metrics_serde() {
    let d = DiskMetrics {
        device: "/dev/sda1".into(),
        mount_point: "/".into(),
        filesystem: "ext4".into(),
        total_bytes: 1,
        used_bytes: 0,
        available_bytes: 1,
        usage_percent: 0.0,
        read_ops_per_sec: 0.0,
        write_ops_per_sec: 0.0,
        read_bytes_per_sec: 0,
        write_bytes_per_sec: 0,
    };
    let json = serde_json::to_string(&d).unwrap();
    let _: DiskMetrics = serde_json::from_str(&json).unwrap();

    let n = NetworkMetrics {
        interface: "eth0".into(),
        rx_bytes: 1,
        tx_bytes: 2,
        rx_packets: 0,
        tx_packets: 0,
        rx_errors: 0,
        tx_errors: 0,
        rx_drops: 0,
        tx_drops: 0,
        status: "up".into(),
        speed_mbps: 1000,
        duplex: "full".into(),
    };
    let json = serde_json::to_string(&n).unwrap();
    let _: NetworkMetrics = serde_json::from_str(&json).unwrap();
}

#[test]
fn metrics_service_info_serde() {
    let si = super::metrics::ServiceInfo {
        name: "nestgate".into(),
        status: UnifiedServiceState::Running,
        pid: Some(1),
        cpu_percent: 0.1,
        memory_bytes: 100,
        start_time: None,
        description: None,
        dependencies: vec![],
        command_line: None,
    };
    let j = serde_json::to_string(&si).unwrap();
    let _: super::metrics::ServiceInfo = serde_json::from_str(&j).unwrap();
}
