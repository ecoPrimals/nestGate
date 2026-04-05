// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Exhaustive `Display` + serde roundtrips for unified enums (line coverage).

use serde::{Deserialize, Serialize};

use crate::unified_enums::UnifiedEnum;
use crate::unified_enums::data_types::{UnifiedContentType, UnifiedDataType, UnifiedFileType};
use crate::unified_enums::message_event_types::{
    UnifiedAlertSeverity, UnifiedAlertType, UnifiedEventType, UnifiedMessageType,
    UnifiedOperationType,
};
use crate::unified_enums::network_types::{
    UnifiedIntegrationType, UnifiedProtocolType, UnifiedProxyType,
};
use crate::unified_enums::service_types::{
    UnifiedConnectionStatus, UnifiedHealthStatus, UnifiedServiceState, UnifiedServiceType,
};
use crate::unified_enums::storage_access_types::{
    UnifiedAccessType, UnifiedStorageType as AccessPatternStorageType, UnifiedTierType,
};
use crate::unified_enums::storage_types::{
    StorageOperation, UnifiedStorageCapability, UnifiedStorageType as CoreUnifiedStorageType,
};
use crate::unified_enums::system_health_types::{
    UnifiedMonitoringStatus, UnifiedSystemStatus, UnifiedTestType,
};

fn serde_roundtrip<T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug>(v: &T) {
    let json = serde_json::to_string(v).expect("serialize");
    let back: T = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(v, &back);
}

#[test]
fn unified_data_types_display_and_serde() {
    for v in [
        UnifiedDataType::Binary,
        UnifiedDataType::Text,
        UnifiedDataType::Structured,
        UnifiedDataType::Multimedia,
        UnifiedDataType::Genomic,
        UnifiedDataType::Scientific,
        UnifiedDataType::Model,
        UnifiedDataType::Dataset,
        UnifiedDataType::Configuration,
        UnifiedDataType::Logs,
        UnifiedDataType::Archive,
        UnifiedDataType::Temporary,
        UnifiedDataType::Unknown,
        UnifiedDataType::Custom("c".into()),
    ] {
        let _ = v.to_string();
        serde_roundtrip(&v);
    }
    assert_eq!(UnifiedDataType::default(), UnifiedDataType::Unknown);

    for v in [
        UnifiedContentType::Json,
        UnifiedContentType::Xml,
        UnifiedContentType::Text,
        UnifiedContentType::Html,
        UnifiedContentType::Binary,
        UnifiedContentType::Yaml,
        UnifiedContentType::Toml,
        UnifiedContentType::Csv,
        UnifiedContentType::Markdown,
        UnifiedContentType::Pdf,
        UnifiedContentType::Image,
        UnifiedContentType::Audio,
        UnifiedContentType::Video,
        UnifiedContentType::Database,
        UnifiedContentType::TimeSeries,
        UnifiedContentType::Compressed,
        UnifiedContentType::Encrypted,
        UnifiedContentType::Geospatial,
        UnifiedContentType::Graph,
        UnifiedContentType::Custom("mime".into()),
    ] {
        let _ = v.to_string();
        serde_roundtrip(&v);
    }
    assert_eq!(UnifiedContentType::default(), UnifiedContentType::Text);

    for v in [
        UnifiedFileType::Regular,
        UnifiedFileType::Directory,
        UnifiedFileType::Symlink,
        UnifiedFileType::Hardlink,
        UnifiedFileType::Device,
        UnifiedFileType::Fifo,
        UnifiedFileType::Socket,
        UnifiedFileType::Unknown,
        UnifiedFileType::Custom("ft".into()),
    ] {
        let _ = v.to_string();
        serde_roundtrip(&v);
    }
}

#[test]
fn message_and_event_types_display_and_serde() {
    for v in [
        UnifiedMessageType::Request,
        UnifiedMessageType::Response,
        UnifiedMessageType::Event,
        UnifiedMessageType::Status,
        UnifiedMessageType::Error,
        UnifiedMessageType::Heartbeat,
        UnifiedMessageType::Acknowledgment,
        UnifiedMessageType::Command,
        UnifiedMessageType::Query,
        UnifiedMessageType::Broadcast,
        UnifiedMessageType::Custom("m".into()),
    ] {
        let _ = v.to_string();
        serde_roundtrip(&v);
    }

    for v in [
        UnifiedEventType::SystemStart,
        UnifiedEventType::SystemStop,
        UnifiedEventType::ServiceLifecycle,
        UnifiedEventType::ConfigurationChange,
        UnifiedEventType::NetworkEvent,
        UnifiedEventType::StorageEvent,
        UnifiedEventType::SecurityEvent,
        UnifiedEventType::PerformanceEvent,
        UnifiedEventType::UserEvent,
        UnifiedEventType::ApiEvent,
        UnifiedEventType::FileSystemEvent,
        UnifiedEventType::DatabaseEvent,
        UnifiedEventType::ErrorEvent,
        UnifiedEventType::Custom("e".into()),
    ] {
        let _ = v.to_string();
        serde_roundtrip(&v);
    }

    for v in [
        UnifiedOperationType::Create,
        UnifiedOperationType::Read,
        UnifiedOperationType::Update,
        UnifiedOperationType::Delete,
        UnifiedOperationType::List,
        UnifiedOperationType::Search,
        UnifiedOperationType::Backup,
        UnifiedOperationType::Restore,
        UnifiedOperationType::Sync,
        UnifiedOperationType::Monitor,
        UnifiedOperationType::HealthCheck,
        UnifiedOperationType::Configure,
        UnifiedOperationType::Security,
        UnifiedOperationType::Custom("op".into()),
    ] {
        let _ = v.to_string();
        serde_roundtrip(&v);
    }

    for v in [
        UnifiedAlertType::Performance,
        UnifiedAlertType::Security,
        UnifiedAlertType::Storage,
        UnifiedAlertType::Network,
        UnifiedAlertType::Service,
        UnifiedAlertType::Configuration,
        UnifiedAlertType::Resource,
        UnifiedAlertType::Error,
        UnifiedAlertType::Custom("a".into()),
    ] {
        serde_roundtrip(&v);
    }

    for v in [
        UnifiedAlertSeverity::Critical,
        UnifiedAlertSeverity::High,
        UnifiedAlertSeverity::Medium,
        UnifiedAlertSeverity::Low,
        UnifiedAlertSeverity::Info,
        UnifiedAlertSeverity::Custom("s".into()),
    ] {
        serde_roundtrip(&v);
    }
}

#[test]
fn network_classification_enums_display_and_serde() {
    for v in [
        UnifiedProtocolType::Http,
        UnifiedProtocolType::Https,
        UnifiedProtocolType::WebSocket,
        UnifiedProtocolType::Tcp,
        UnifiedProtocolType::Udp,
        UnifiedProtocolType::Grpc,
        UnifiedProtocolType::Rest,
        UnifiedProtocolType::GraphQL,
        UnifiedProtocolType::MessageQueue,
        UnifiedProtocolType::Custom("p".into()),
    ] {
        let _ = v.to_string();
        serde_roundtrip(&v);
    }

    for v in [
        UnifiedIntegrationType::DirectApi,
        UnifiedIntegrationType::Database,
        UnifiedIntegrationType::FileSystem,
        UnifiedIntegrationType::MessageQueue,
        UnifiedIntegrationType::Webhook,
        UnifiedIntegrationType::EventDriven,
        UnifiedIntegrationType::BatchProcessing,
        UnifiedIntegrationType::RealTimeStreaming,
        UnifiedIntegrationType::Custom("i".into()),
    ] {
        let _ = v.to_string();
        serde_roundtrip(&v);
    }

    for v in [
        UnifiedProxyType::Http,
        UnifiedProxyType::Https,
        UnifiedProxyType::Socks4,
        UnifiedProxyType::Socks5,
        UnifiedProxyType::Transparent,
        UnifiedProxyType::Reverse,
        UnifiedProxyType::LoadBalancer,
        UnifiedProxyType::None,
        UnifiedProxyType::Custom("pr".into()),
    ] {
        let _ = v.to_string();
        serde_roundtrip(&v);
    }
}

#[test]
fn service_state_enums_display_and_serde() {
    for v in [
        UnifiedServiceType::AI,
        UnifiedServiceType::Storage,
        UnifiedServiceType::Orchestration,
        UnifiedServiceType::Security,
        UnifiedServiceType::Compute,
        UnifiedServiceType::Network,
        UnifiedServiceType::Monitoring,
        UnifiedServiceType::Adapter,
        UnifiedServiceType::Gateway,
        UnifiedServiceType::Worker,
        UnifiedServiceType::Generic,
        UnifiedServiceType::Unknown,
        UnifiedServiceType::Custom("svc".into()),
    ] {
        let _ = v.to_string();
        let _ = v.as_str();
        let _ = v.is_custom();
        serde_roundtrip(&v);
    }
    assert_eq!(
        UnifiedServiceType::from_str("monitoring"),
        UnifiedServiceType::Monitoring
    );

    for v in [
        UnifiedHealthStatus::Healthy,
        UnifiedHealthStatus::Degraded,
        UnifiedHealthStatus::Unhealthy,
        UnifiedHealthStatus::Offline,
        UnifiedHealthStatus::Starting,
        UnifiedHealthStatus::Stopping,
        UnifiedHealthStatus::Maintenance,
        UnifiedHealthStatus::Unknown,
        UnifiedHealthStatus::Warning,
        UnifiedHealthStatus::Critical,
        UnifiedHealthStatus::Error,
        UnifiedHealthStatus::Custom("h".into()),
    ] {
        let _ = v.to_string();
        serde_roundtrip(&v);
    }

    for v in [
        UnifiedServiceState::Running,
        UnifiedServiceState::Stopped,
        UnifiedServiceState::Starting,
        UnifiedServiceState::Stopping,
        UnifiedServiceState::Error,
        UnifiedServiceState::Paused,
        UnifiedServiceState::Maintenance,
        UnifiedServiceState::Unknown,
        UnifiedServiceState::Custom("st".into()),
    ] {
        let _ = v.to_string();
        serde_roundtrip(&v);
    }

    for v in [
        UnifiedConnectionStatus::Connected,
        UnifiedConnectionStatus::Connecting,
        UnifiedConnectionStatus::Disconnected,
        UnifiedConnectionStatus::Failed,
        UnifiedConnectionStatus::Retrying,
        UnifiedConnectionStatus::Timeout,
        UnifiedConnectionStatus::Refused,
        UnifiedConnectionStatus::Unknown,
        UnifiedConnectionStatus::Custom("cn".into()),
    ] {
        let _ = v.to_string();
        serde_roundtrip(&v);
    }
}

#[test]
fn storage_access_pattern_enums_display_and_serde() {
    for v in [
        AccessPatternStorageType::Local,
        AccessPatternStorageType::Nfs,
        AccessPatternStorageType::Smb,
        AccessPatternStorageType::Object,
        AccessPatternStorageType::Block,
        AccessPatternStorageType::Zfs,
        AccessPatternStorageType::Database,
        AccessPatternStorageType::Memory,
        AccessPatternStorageType::Cache,
        AccessPatternStorageType::Cloud,
        AccessPatternStorageType::Distributed,
        AccessPatternStorageType::Custom("st".into()),
    ] {
        let _ = v.to_string();
        serde_roundtrip(&v);
    }

    for v in [
        UnifiedAccessType::Read,
        UnifiedAccessType::Write,
        UnifiedAccessType::ReadWrite,
        UnifiedAccessType::Execute,
        UnifiedAccessType::Admin,
        UnifiedAccessType::None,
        UnifiedAccessType::Custom("ac".into()),
    ] {
        let _ = v.to_string();
        serde_roundtrip(&v);
    }

    for v in [
        UnifiedTierType::Hot,
        UnifiedTierType::Warm,
        UnifiedTierType::Cool,
        UnifiedTierType::Cold,
        UnifiedTierType::Frozen,
        UnifiedTierType::Custom("t".into()),
    ] {
        let _ = v.to_string();
        serde_roundtrip(&v);
    }
}

#[test]
fn system_health_enums_display_and_serde() {
    for v in [
        UnifiedSystemStatus::Operational,
        UnifiedSystemStatus::Starting,
        UnifiedSystemStatus::Stopping,
        UnifiedSystemStatus::Maintenance,
        UnifiedSystemStatus::Degraded,
        UnifiedSystemStatus::Down,
        UnifiedSystemStatus::Unknown,
        UnifiedSystemStatus::Custom("sys".into()),
    ] {
        let _ = v.to_string();
        serde_roundtrip(&v);
    }

    for v in [
        UnifiedTestType::Unit,
        UnifiedTestType::Integration,
        UnifiedTestType::EndToEnd,
        UnifiedTestType::Performance,
        UnifiedTestType::Load,
        UnifiedTestType::Stress,
        UnifiedTestType::Security,
        UnifiedTestType::Chaos,
        UnifiedTestType::Functional,
        UnifiedTestType::Regression,
        UnifiedTestType::Smoke,
        UnifiedTestType::Custom("tst".into()),
    ] {
        let _ = v.to_string();
        serde_roundtrip(&v);
    }

    for v in [
        UnifiedMonitoringStatus::Active,
        UnifiedMonitoringStatus::Paused,
        UnifiedMonitoringStatus::Disabled,
        UnifiedMonitoringStatus::ConfigError,
        UnifiedMonitoringStatus::ConnectionError,
        UnifiedMonitoringStatus::Custom("mon".into()),
    ] {
        let _ = v.to_string();
        serde_roundtrip(&v);
    }
}

#[test]
fn core_storage_module_enums_roundtrip() {
    for v in [
        CoreUnifiedStorageType::Local,
        CoreUnifiedStorageType::Zfs,
        CoreUnifiedStorageType::Network,
        CoreUnifiedStorageType::Cloud,
        CoreUnifiedStorageType::Memory,
        CoreUnifiedStorageType::Database,
        CoreUnifiedStorageType::Custom("core".into()),
    ] {
        serde_roundtrip(&v);
    }

    for v in [
        StorageOperation::Read,
        StorageOperation::Write,
        StorageOperation::Delete,
        StorageOperation::List,
        StorageOperation::Create,
        StorageOperation::Move,
        StorageOperation::Copy,
        StorageOperation::Sync,
        StorageOperation::Backup,
        StorageOperation::Restore,
    ] {
        serde_roundtrip(&v);
    }

    for v in [
        UnifiedStorageCapability::Compression,
        UnifiedStorageCapability::Encryption,
        UnifiedStorageCapability::Deduplication,
        UnifiedStorageCapability::Snapshots,
        UnifiedStorageCapability::Journaling,
        UnifiedStorageCapability::Replication,
        UnifiedStorageCapability::Versioning,
        UnifiedStorageCapability::Backup,
    ] {
        serde_roundtrip(&v);
    }
}
