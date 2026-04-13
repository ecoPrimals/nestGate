// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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

fn serde_roundtrip<T>(value: &T) -> Result<T, serde_json::Error>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    let json = serde_json::to_string(value)?;
    serde_json::from_str(&json)
}

#[test]
fn r5_unified_error_from_io_impl() {
    let io_err = std::io::Error::other("disk full");
    let e: NestGateUnifiedError = io_err.into();
    assert!(matches!(e, NestGateUnifiedError::Internal(_)));
    assert!(e.to_string().contains("I/O error"));
}

#[test]
fn r5_unified_error_from_json_impl() -> std::result::Result<(), &'static str> {
    let Err(json_err) = serde_json::from_str::<serde_json::Value>("not-json") else {
        return Err("expected JSON parse failure");
    };
    let e: NestGateUnifiedError = json_err.into();
    assert!(matches!(e, NestGateUnifiedError::Validation(_)));
    assert!(e.to_string().contains("JSON error"));
    Ok(())
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
fn r5_public_unified_storage_type_default_serde() -> Result<(), serde_json::Error> {
    let v = UnifiedStorageType::default();
    assert_eq!(v, UnifiedStorageType::Local);
    let back = serde_roundtrip(&UnifiedStorageType::Zfs)?;
    assert_eq!(back, UnifiedStorageType::Zfs);
    Ok(())
}

#[test]
fn r5_storage_access_unified_storage_type_display_default_serde() -> Result<(), serde_json::Error> {
    let v = StorageAccessStorageType::default();
    assert_eq!(v, StorageAccessStorageType::Local);
    assert_eq!(StorageAccessStorageType::Object.to_string(), "object");
    let x = StorageAccessStorageType::Custom("ceph".to_string());
    let back = serde_roundtrip(&x)?;
    assert_eq!(x, back);
    Ok(())
}

#[test]
fn r5_storage_access_unified_access_type_display_default_serde() -> Result<(), serde_json::Error> {
    assert_eq!(UnifiedAccessType::default(), UnifiedAccessType::Read);
    assert_eq!(UnifiedAccessType::ReadWrite.to_string(), "read_write");
    let x = UnifiedAccessType::Custom("scoped".to_string());
    let back = serde_roundtrip(&x)?;
    assert_eq!(x, back);
    Ok(())
}

#[test]
fn r5_storage_access_unified_tier_type_display_default_serde() -> Result<(), serde_json::Error> {
    assert_eq!(UnifiedTierType::default(), UnifiedTierType::Hot);
    assert_eq!(UnifiedTierType::Frozen.to_string(), "frozen");
    let x = UnifiedTierType::Custom("glacier".to_string());
    let back = serde_roundtrip(&x)?;
    assert_eq!(x, back);
    Ok(())
}

#[test]
fn r5_unified_system_status_display_default_serde() -> Result<(), serde_json::Error> {
    assert_eq!(UnifiedSystemStatus::default(), UnifiedSystemStatus::Unknown);
    assert_eq!(UnifiedSystemStatus::Operational.to_string(), "operational");
    let x = UnifiedSystemStatus::Custom("drill".to_string());
    let back = serde_roundtrip(&x)?;
    assert_eq!(x, back);
    Ok(())
}

#[test]
fn r5_unified_test_type_display_default_serde() -> Result<(), serde_json::Error> {
    assert_eq!(UnifiedTestType::default(), UnifiedTestType::Unit);
    assert_eq!(UnifiedTestType::Regression.to_string(), "regression");
    let x = UnifiedTestType::Custom("fuzz".to_string());
    let back = serde_roundtrip(&x)?;
    assert_eq!(x, back);
    Ok(())
}

#[test]
fn r5_unified_monitoring_status_display_default_serde() -> Result<(), serde_json::Error> {
    assert_eq!(
        UnifiedMonitoringStatus::default(),
        UnifiedMonitoringStatus::Active
    );
    assert_eq!(
        UnifiedMonitoringStatus::ConnectionError.to_string(),
        "connection_error"
    );
    let x = UnifiedMonitoringStatus::Custom("degraded".to_string());
    let back = serde_roundtrip(&x)?;
    assert_eq!(x, back);
    Ok(())
}

#[test]
fn r5_unified_message_type_display_default_serde() -> Result<(), serde_json::Error> {
    assert_eq!(UnifiedMessageType::default(), UnifiedMessageType::Request);
    assert_eq!(
        UnifiedMessageType::Acknowledgment.to_string(),
        "acknowledgment"
    );
    let back = serde_roundtrip(&UnifiedMessageType::Heartbeat)?;
    assert_eq!(back, UnifiedMessageType::Heartbeat);
    Ok(())
}

#[test]
fn r5_unified_error_category_display_default_serde() -> Result<(), serde_json::Error> {
    assert_eq!(
        UnifiedErrorCategory::default(),
        UnifiedErrorCategory::SystemStart
    );
    assert_eq!(
        UnifiedErrorCategory::ConfigurationChange.to_string(),
        "configuration_change"
    );
    let back = serde_roundtrip(&UnifiedErrorCategory::SecurityEvent)?;
    assert_eq!(back, UnifiedErrorCategory::SecurityEvent);
    Ok(())
}

#[test]
fn r5_unified_operation_type_display_default_serde() -> Result<(), serde_json::Error> {
    assert_eq!(UnifiedOperationType::default(), UnifiedOperationType::Read);
    assert_eq!(
        UnifiedOperationType::HealthCheck.to_string(),
        "health_check"
    );
    let back = serde_roundtrip(&UnifiedOperationType::Delete)?;
    assert_eq!(back, UnifiedOperationType::Delete);
    Ok(())
}

#[test]
fn r5_unified_alert_type_default_serde() -> Result<(), serde_json::Error> {
    assert_eq!(UnifiedAlertType::default(), UnifiedAlertType::Performance);
    let back = serde_roundtrip(&UnifiedAlertType::Security)?;
    assert_eq!(back, UnifiedAlertType::Security);
    Ok(())
}

#[test]
fn r5_unified_alert_severity_default_serde() -> Result<(), serde_json::Error> {
    assert_eq!(UnifiedAlertSeverity::default(), UnifiedAlertSeverity::Info);
    let x = UnifiedAlertSeverity::Custom("p3".to_string());
    let back = serde_roundtrip(&x)?;
    assert_eq!(x, back);
    Ok(())
}

#[test]
fn r5_unified_data_type_display_and_serde() -> Result<(), serde_json::Error> {
    assert_eq!(UnifiedDataType::Binary.to_string(), "binary");
    let back = serde_roundtrip(&UnifiedDataType::Structured)?;
    assert_eq!(back, UnifiedDataType::Structured);
    Ok(())
}

#[test]
fn r5_unified_content_type_mime_roundtrip() -> Result<(), serde_json::Error> {
    for ct in [
        UnifiedContentType::Json,
        UnifiedContentType::Xml,
        UnifiedContentType::Yaml,
    ] {
        let back: UnifiedContentType = serde_roundtrip(&ct)?;
        assert_eq!(ct, back);
    }
    Ok(())
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
fn r5_unified_protocol_integration_proxy_roundtrip() -> Result<(), serde_json::Error> {
    let p = UnifiedProtocolType::Grpc;
    let back = serde_roundtrip(&p)?;
    assert_eq!(p, back);
    assert_eq!(UnifiedIntegrationType::Webhook.to_string(), "webhook");
    let q = UnifiedProxyType::Socks5;
    let back = serde_roundtrip(&q)?;
    assert_eq!(q, back);
    Ok(())
}

#[test]
fn r5_unified_service_health_connection_roundtrip() -> Result<(), serde_json::Error> {
    let s = UnifiedServiceType::Orchestration;
    let back = serde_roundtrip(&s)?;
    assert_eq!(s, back);

    let h = UnifiedHealthStatus::Degraded;
    let back = serde_roundtrip(&h)?;
    assert_eq!(h, back);

    let c = UnifiedConnectionStatus::Disconnected;
    let back = serde_roundtrip(&c)?;
    assert_eq!(c, back);
    Ok(())
}

#[test]
fn r5_error_severity_serde_roundtrip() -> Result<(), serde_json::Error> {
    for sev in [
        ErrorSeverity::Low,
        ErrorSeverity::Medium,
        ErrorSeverity::High,
        ErrorSeverity::Critical,
    ] {
        let back: ErrorSeverity = serde_roundtrip(&sev)?;
        assert_eq!(sev, back);
    }
    Ok(())
}

#[test]
fn r5_storage_operation_serde_roundtrip() -> Result<(), serde_json::Error> {
    for op in [
        StorageOperation::Read,
        StorageOperation::Backup,
        StorageOperation::Restore,
    ] {
        let back: StorageOperation = serde_roundtrip(&op)?;
        assert_eq!(op, back);
    }
    Ok(())
}

#[test]
fn r5_unified_storage_capability_serde_roundtrip() -> Result<(), serde_json::Error> {
    let cap = UnifiedStorageCapability::Replication;
    let back = serde_roundtrip(&cap)?;
    assert_eq!(cap, back);
    Ok(())
}

#[test]
fn r5_unified_service_state_default_roundtrip() -> Result<(), serde_json::Error> {
    assert_eq!(UnifiedServiceState::default(), UnifiedServiceState::Unknown);
    let back = serde_roundtrip(&UnifiedServiceState::Starting)?;
    assert_eq!(back, UnifiedServiceState::Starting);
    Ok(())
}

#[test]
fn r5_storage_tier_default_ordering() {
    assert_eq!(StorageTier::default(), StorageTier::Hot);
    assert!(StorageTier::Hot < StorageTier::Warm);
}

#[test]
fn r5_storage_tier_serde_roundtrip() -> Result<(), serde_json::Error> {
    let t = StorageTier::Frozen;
    let back = serde_roundtrip(&t)?;
    assert_eq!(t, back);
    Ok(())
}

#[test]
fn r5_unified_protocol_type_custom_display() {
    let p = UnifiedProtocolType::Custom("coap".to_string());
    assert_eq!(p.to_string(), "coap");
}

#[test]
fn r5_unified_integration_type_serde_roundtrip() -> Result<(), serde_json::Error> {
    let i = UnifiedIntegrationType::EventDriven;
    let back = serde_roundtrip(&i)?;
    assert_eq!(i, back);
    Ok(())
}

#[test]
fn r5_unified_proxy_type_serde_roundtrip() -> Result<(), serde_json::Error> {
    let p = UnifiedProxyType::Reverse;
    let back = serde_roundtrip(&p)?;
    assert_eq!(p, back);
    Ok(())
}

#[test]
fn r5_unified_health_status_custom_serde() -> Result<(), serde_json::Error> {
    let h = UnifiedHealthStatus::Custom("amber".to_string());
    let back = serde_roundtrip(&h)?;
    assert_eq!(h, back);
    Ok(())
}

#[test]
fn r5_unified_message_type_broadcast_serde() -> Result<(), serde_json::Error> {
    let m = UnifiedMessageType::Broadcast;
    let back = serde_roundtrip(&m)?;
    assert_eq!(m, back);
    Ok(())
}

#[test]
fn r5_unified_alert_type_custom_serde() -> Result<(), serde_json::Error> {
    let a = UnifiedAlertType::Custom("disk".to_string());
    let back = serde_roundtrip(&a)?;
    assert_eq!(a, back);
    Ok(())
}

#[test]
fn r5_unified_data_type_custom_roundtrip() -> Result<(), serde_json::Error> {
    let d = UnifiedDataType::Custom("parquet".to_string());
    let back = serde_roundtrip(&d)?;
    assert_eq!(d, back);
    Ok(())
}

#[test]
fn r5_unified_content_type_html_roundtrip() -> Result<(), serde_json::Error> {
    let c = UnifiedContentType::Html;
    let back = serde_roundtrip(&c)?;
    assert_eq!(c, back);
    Ok(())
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
fn r5_unified_operation_type_custom_serde() -> Result<(), serde_json::Error> {
    let o = UnifiedOperationType::Custom("migrate".to_string());
    let back = serde_roundtrip(&o)?;
    assert_eq!(o, back);
    Ok(())
}

#[test]
fn r5_unified_service_type_custom_roundtrip() -> Result<(), serde_json::Error> {
    let s = UnifiedServiceType::Custom("ledger".to_string());
    let back = serde_roundtrip(&s)?;
    assert_eq!(s, back);
    Ok(())
}

#[test]
fn r5_unified_connection_status_custom_serde() -> Result<(), serde_json::Error> {
    let c = UnifiedConnectionStatus::Custom("limbo".to_string());
    let back = serde_roundtrip(&c)?;
    assert_eq!(c, back);
    Ok(())
}

#[test]
fn r5_unified_monitoring_status_paused_display() {
    assert_eq!(UnifiedMonitoringStatus::Paused.to_string(), "paused");
}
