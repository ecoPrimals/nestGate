// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Round 5: `Display`, `From`, `Default`, and serde coverage for consolidated types.

use crate::NestGateUnifiedError;
use crate::error::variants::core_errors::ErrorSeverity;
use crate::unified_enums::storage_access_types::{
    UnifiedAccessType, UnifiedStorageType as StorageAccessStorageType, UnifiedTierType,
};
use crate::unified_enums::storage_types::StorageTier;
use crate::unified_enums::system_health_types::{
    UnifiedMonitoringStatus, UnifiedSystemStatus, UnifiedTestType,
};
use crate::unified_enums::{StorageOperation, UnifiedStorageCapability};
use crate::unified_enums::{
    UnifiedAlertSeverity, UnifiedAlertType, UnifiedConnectionStatus, UnifiedContentType,
    UnifiedDataType, UnifiedErrorCategory, UnifiedFileType, UnifiedHealthStatus,
    UnifiedIntegrationType, UnifiedMessageType, UnifiedOperationType, UnifiedProtocolType,
    UnifiedProxyType, UnifiedServiceState, UnifiedServiceType, UnifiedStorageType,
};

#[test]
fn r5_unified_error_from_io_impl() {
    let io_err = std::io::Error::new(std::io::ErrorKind::Other, "disk full");
    let e: NestGateUnifiedError = io_err.into();
    assert!(matches!(e, NestGateUnifiedError::Internal(_)));
    assert!(e.to_string().contains("I/O error"));
}

#[test]
fn r5_unified_error_from_json_impl() {
    let json_err = serde_json::from_str::<serde_json::Value>("not-json").unwrap_err();
    let e: NestGateUnifiedError = json_err.into();
    assert!(matches!(e, NestGateUnifiedError::Validation(_)));
    assert!(e.to_string().contains("JSON error"));
}

#[test]
fn r5_unified_error_from_string_impl() {
    let e: NestGateUnifiedError = "boom".to_string().into();
    assert!(e.to_string().contains("boom"));
}

#[test]
fn r5_unified_error_from_str_impl() {
    let e: NestGateUnifiedError = "slice err".into();
    assert!(e.to_string().contains("slice err"));
}

#[test]
fn r5_unified_error_from_anyhow_impl() {
    let e: NestGateUnifiedError = anyhow::anyhow!("external failure").into();
    assert!(matches!(e, NestGateUnifiedError::External(_)));
    assert!(e.to_string().contains("External error"));
}

#[test]
fn r5_public_unified_storage_type_default_serde() {
    let v = UnifiedStorageType::default();
    assert_eq!(v, UnifiedStorageType::Local);
    let json = serde_json::to_string(&UnifiedStorageType::Zfs).unwrap();
    let back: UnifiedStorageType = serde_json::from_str(&json).unwrap();
    assert_eq!(back, UnifiedStorageType::Zfs);
}

#[test]
fn r5_storage_access_unified_storage_type_display_default_serde() {
    let v = StorageAccessStorageType::default();
    assert_eq!(v, StorageAccessStorageType::Local);
    assert_eq!(StorageAccessStorageType::Object.to_string(), "object");
    let x = StorageAccessStorageType::Custom("ceph".to_string());
    let json = serde_json::to_string(&x).unwrap();
    let back: StorageAccessStorageType = serde_json::from_str(&json).unwrap();
    assert_eq!(x, back);
}

#[test]
fn r5_storage_access_unified_access_type_display_default_serde() {
    assert_eq!(UnifiedAccessType::default(), UnifiedAccessType::Read);
    assert_eq!(UnifiedAccessType::ReadWrite.to_string(), "read_write");
    let x = UnifiedAccessType::Custom("scoped".to_string());
    let json = serde_json::to_string(&x).unwrap();
    let back: UnifiedAccessType = serde_json::from_str(&json).unwrap();
    assert_eq!(x, back);
}

#[test]
fn r5_storage_access_unified_tier_type_display_default_serde() {
    assert_eq!(UnifiedTierType::default(), UnifiedTierType::Hot);
    assert_eq!(UnifiedTierType::Frozen.to_string(), "frozen");
    let x = UnifiedTierType::Custom("glacier".to_string());
    let json = serde_json::to_string(&x).unwrap();
    let back: UnifiedTierType = serde_json::from_str(&json).unwrap();
    assert_eq!(x, back);
}

#[test]
fn r5_unified_system_status_display_default_serde() {
    assert_eq!(UnifiedSystemStatus::default(), UnifiedSystemStatus::Unknown);
    assert_eq!(UnifiedSystemStatus::Operational.to_string(), "operational");
    let x = UnifiedSystemStatus::Custom("drill".to_string());
    let json = serde_json::to_string(&x).unwrap();
    let back: UnifiedSystemStatus = serde_json::from_str(&json).unwrap();
    assert_eq!(x, back);
}

#[test]
fn r5_unified_test_type_display_default_serde() {
    assert_eq!(UnifiedTestType::default(), UnifiedTestType::Unit);
    assert_eq!(UnifiedTestType::Regression.to_string(), "regression");
    let x = UnifiedTestType::Custom("fuzz".to_string());
    let json = serde_json::to_string(&x).unwrap();
    let back: UnifiedTestType = serde_json::from_str(&json).unwrap();
    assert_eq!(x, back);
}

#[test]
fn r5_unified_monitoring_status_display_default_serde() {
    assert_eq!(
        UnifiedMonitoringStatus::default(),
        UnifiedMonitoringStatus::Active
    );
    assert_eq!(
        UnifiedMonitoringStatus::ConnectionError.to_string(),
        "connection_error"
    );
    let x = UnifiedMonitoringStatus::Custom("degraded".to_string());
    let json = serde_json::to_string(&x).unwrap();
    let back: UnifiedMonitoringStatus = serde_json::from_str(&json).unwrap();
    assert_eq!(x, back);
}

#[test]
fn r5_unified_message_type_display_default_serde() {
    assert_eq!(UnifiedMessageType::default(), UnifiedMessageType::Request);
    assert_eq!(
        UnifiedMessageType::Acknowledgment.to_string(),
        "acknowledgment"
    );
    let json = serde_json::to_string(&UnifiedMessageType::Heartbeat).unwrap();
    let back: UnifiedMessageType = serde_json::from_str(&json).unwrap();
    assert_eq!(back, UnifiedMessageType::Heartbeat);
}

#[test]
fn r5_unified_error_category_display_default_serde() {
    assert_eq!(
        UnifiedErrorCategory::default(),
        UnifiedErrorCategory::SystemStart
    );
    assert_eq!(
        UnifiedErrorCategory::ConfigurationChange.to_string(),
        "configuration_change"
    );
    let json = serde_json::to_string(&UnifiedErrorCategory::SecurityEvent).unwrap();
    let back: UnifiedErrorCategory = serde_json::from_str(&json).unwrap();
    assert_eq!(back, UnifiedErrorCategory::SecurityEvent);
}

#[test]
fn r5_unified_operation_type_display_default_serde() {
    assert_eq!(UnifiedOperationType::default(), UnifiedOperationType::Read);
    assert_eq!(
        UnifiedOperationType::HealthCheck.to_string(),
        "health_check"
    );
    let json = serde_json::to_string(&UnifiedOperationType::Delete).unwrap();
    let back: UnifiedOperationType = serde_json::from_str(&json).unwrap();
    assert_eq!(back, UnifiedOperationType::Delete);
}

#[test]
fn r5_unified_alert_type_default_serde() {
    assert_eq!(UnifiedAlertType::default(), UnifiedAlertType::Performance);
    let json = serde_json::to_string(&UnifiedAlertType::Security).unwrap();
    let back: UnifiedAlertType = serde_json::from_str(&json).unwrap();
    assert_eq!(back, UnifiedAlertType::Security);
}

#[test]
fn r5_unified_alert_severity_default_serde() {
    assert_eq!(UnifiedAlertSeverity::default(), UnifiedAlertSeverity::Info);
    let x = UnifiedAlertSeverity::Custom("p3".to_string());
    let json = serde_json::to_string(&x).unwrap();
    let back: UnifiedAlertSeverity = serde_json::from_str(&json).unwrap();
    assert_eq!(x, back);
}

#[test]
fn r5_unified_data_type_display_and_serde() {
    assert_eq!(UnifiedDataType::Binary.to_string(), "binary");
    let json = serde_json::to_string(&UnifiedDataType::Structured).unwrap();
    let back: UnifiedDataType = serde_json::from_str(&json).unwrap();
    assert_eq!(back, UnifiedDataType::Structured);
}

#[test]
fn r5_unified_content_type_mime_roundtrip() {
    for ct in [
        UnifiedContentType::Json,
        UnifiedContentType::Xml,
        UnifiedContentType::Yaml,
    ] {
        let json = serde_json::to_string(&ct).unwrap();
        let back: UnifiedContentType = serde_json::from_str(&json).unwrap();
        assert_eq!(ct, back);
    }
}

#[test]
fn r5_unified_file_type_display_custom() {
    assert_eq!(UnifiedFileType::Regular.to_string(), "regular");
    assert_eq!(
        UnifiedFileType::Custom("fuse".to_string()).to_string(),
        "fuse"
    );
}

#[test]
fn r5_unified_protocol_integration_proxy_roundtrip() {
    let p = UnifiedProtocolType::Grpc;
    let json = serde_json::to_string(&p).unwrap();
    let back: UnifiedProtocolType = serde_json::from_str(&json).unwrap();
    assert_eq!(p, back);
    assert_eq!(UnifiedIntegrationType::Webhook.to_string(), "webhook");
    let q = UnifiedProxyType::Socks5;
    let json = serde_json::to_string(&q).unwrap();
    let back: UnifiedProxyType = serde_json::from_str(&json).unwrap();
    assert_eq!(q, back);
}

#[test]
fn r5_unified_service_health_connection_roundtrip() {
    let s = UnifiedServiceType::Orchestration;
    let json = serde_json::to_string(&s).unwrap();
    let back: UnifiedServiceType = serde_json::from_str(&json).unwrap();
    assert_eq!(s, back);

    let h = UnifiedHealthStatus::Degraded;
    let json = serde_json::to_string(&h).unwrap();
    let back: UnifiedHealthStatus = serde_json::from_str(&json).unwrap();
    assert_eq!(h, back);

    let c = UnifiedConnectionStatus::Disconnected;
    let json = serde_json::to_string(&c).unwrap();
    let back: UnifiedConnectionStatus = serde_json::from_str(&json).unwrap();
    assert_eq!(c, back);
}

#[test]
fn r5_error_severity_serde_roundtrip() {
    for sev in [
        ErrorSeverity::Low,
        ErrorSeverity::Medium,
        ErrorSeverity::High,
        ErrorSeverity::Critical,
    ] {
        let json = serde_json::to_string(&sev).unwrap();
        let back: ErrorSeverity = serde_json::from_str(&json).unwrap();
        assert_eq!(sev, back);
    }
}

#[test]
fn r5_storage_operation_serde_roundtrip() {
    for op in [
        StorageOperation::Read,
        StorageOperation::Backup,
        StorageOperation::Restore,
    ] {
        let json = serde_json::to_string(&op).unwrap();
        let back: StorageOperation = serde_json::from_str(&json).unwrap();
        assert_eq!(op, back);
    }
}

#[test]
fn r5_unified_storage_capability_serde_roundtrip() {
    let cap = UnifiedStorageCapability::Replication;
    let json = serde_json::to_string(&cap).unwrap();
    let back: UnifiedStorageCapability = serde_json::from_str(&json).unwrap();
    assert_eq!(cap, back);
}

#[test]
fn r5_unified_service_state_default_roundtrip() {
    assert_eq!(UnifiedServiceState::default(), UnifiedServiceState::Unknown);
    let json = serde_json::to_string(&UnifiedServiceState::Starting).unwrap();
    let back: UnifiedServiceState = serde_json::from_str(&json).unwrap();
    assert_eq!(back, UnifiedServiceState::Starting);
}

#[test]
fn r5_storage_tier_default_ordering() {
    assert_eq!(StorageTier::default(), StorageTier::Hot);
    assert!(StorageTier::Hot < StorageTier::Warm);
}

#[test]
fn r5_storage_tier_serde_roundtrip() {
    let t = StorageTier::Frozen;
    let json = serde_json::to_string(&t).unwrap();
    let back: StorageTier = serde_json::from_str(&json).unwrap();
    assert_eq!(t, back);
}

#[test]
fn r5_unified_protocol_type_custom_display() {
    let p = UnifiedProtocolType::Custom("coap".to_string());
    assert_eq!(p.to_string(), "coap");
}

#[test]
fn r5_unified_integration_type_serde_roundtrip() {
    let i = UnifiedIntegrationType::EventDriven;
    let json = serde_json::to_string(&i).unwrap();
    let back: UnifiedIntegrationType = serde_json::from_str(&json).unwrap();
    assert_eq!(i, back);
}

#[test]
fn r5_unified_proxy_type_serde_roundtrip() {
    let p = UnifiedProxyType::Reverse;
    let json = serde_json::to_string(&p).unwrap();
    let back: UnifiedProxyType = serde_json::from_str(&json).unwrap();
    assert_eq!(p, back);
}

#[test]
fn r5_unified_health_status_custom_serde() {
    let h = UnifiedHealthStatus::Custom("amber".to_string());
    let json = serde_json::to_string(&h).unwrap();
    let back: UnifiedHealthStatus = serde_json::from_str(&json).unwrap();
    assert_eq!(h, back);
}

#[test]
fn r5_unified_message_type_broadcast_serde() {
    let m = UnifiedMessageType::Broadcast;
    let json = serde_json::to_string(&m).unwrap();
    let back: UnifiedMessageType = serde_json::from_str(&json).unwrap();
    assert_eq!(m, back);
}

#[test]
fn r5_unified_alert_type_custom_serde() {
    let a = UnifiedAlertType::Custom("disk".to_string());
    let json = serde_json::to_string(&a).unwrap();
    let back: UnifiedAlertType = serde_json::from_str(&json).unwrap();
    assert_eq!(a, back);
}

#[test]
fn r5_unified_data_type_custom_roundtrip() {
    let d = UnifiedDataType::Custom("parquet".to_string());
    let json = serde_json::to_string(&d).unwrap();
    let back: UnifiedDataType = serde_json::from_str(&json).unwrap();
    assert_eq!(d, back);
}

#[test]
fn r5_unified_content_type_html_roundtrip() {
    let c = UnifiedContentType::Html;
    let json = serde_json::to_string(&c).unwrap();
    let back: UnifiedContentType = serde_json::from_str(&json).unwrap();
    assert_eq!(c, back);
}

#[test]
fn r5_unified_file_type_socket_display() {
    assert_eq!(UnifiedFileType::Socket.to_string(), "socket");
}

#[test]
fn r5_unified_error_category_custom_display() {
    let e = UnifiedErrorCategory::Custom("audit".to_string());
    assert_eq!(e.to_string(), "audit");
}

#[test]
fn r5_unified_operation_type_custom_serde() {
    let o = UnifiedOperationType::Custom("migrate".to_string());
    let json = serde_json::to_string(&o).unwrap();
    let back: UnifiedOperationType = serde_json::from_str(&json).unwrap();
    assert_eq!(o, back);
}

#[test]
fn r5_unified_service_type_custom_roundtrip() {
    let s = UnifiedServiceType::Custom("ledger".to_string());
    let json = serde_json::to_string(&s).unwrap();
    let back: UnifiedServiceType = serde_json::from_str(&json).unwrap();
    assert_eq!(s, back);
}

#[test]
fn r5_unified_connection_status_custom_serde() {
    let c = UnifiedConnectionStatus::Custom("limbo".to_string());
    let json = serde_json::to_string(&c).unwrap();
    let back: UnifiedConnectionStatus = serde_json::from_str(&json).unwrap();
    assert_eq!(c, back);
}

#[test]
fn r5_unified_monitoring_status_paused_display() {
    assert_eq!(UnifiedMonitoringStatus::Paused.to_string(), "paused");
}
