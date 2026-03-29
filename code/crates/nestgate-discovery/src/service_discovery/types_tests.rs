// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Unit tests for `service_discovery::types` (defaults, serde roundtrips).

use super::types::*;
use serde_json::json;
use uuid::Uuid;

#[test]
fn service_metadata_default_is_storage_category() {
    let m = ServiceMetadata::default();
    assert_eq!(m.name, "");
    assert_eq!(m.category, ServiceCategory::Storage);
}

#[test]
fn integration_preferences_default() {
    let p = IntegrationPreferences::default();
    assert_eq!(p.preferred_types, vec![IntegrationType::Api]);
    assert_eq!(
        p.preferred_patterns,
        vec![IntegrationPattern::RequestResponse]
    );
    assert_eq!(p.preferred_protocols, vec![CommunicationProtocol::Http]);
    assert_eq!(p.cost_sensitivity, CostSensitivity::Medium);
}

#[test]
fn selection_preferences_default() {
    let p = SelectionPreferences::default();
    assert!(p.prefer_local);
    assert_eq!(p.cost_sensitivity, CostSensitivity::Medium);
    assert!(!p.performance_priority);
}

#[test]
fn performance_requirements_default_availability() {
    let p = PerformanceRequirements::default();
    assert_eq!(p.availability_percent, Some(99.9));
    assert!(p.max_latency_ms.is_none());
}

#[test]
fn serde_roundtrip_service_metadata() {
    let original = ServiceMetadata {
        name: "svc-a".to_string(),
        category: ServiceCategory::Network,
        version: "2.0.0".to_string(),
        description: "desc".to_string(),
        health_endpoint: Some("/health".to_string()),
        metrics_endpoint: None,
    };
    let json = serde_json::to_string(&original).expect("serialize");
    let back: ServiceMetadata = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.name, original.name);
    assert_eq!(back.version, original.version);
    assert_eq!(back.description, original.description);
    assert_eq!(back.health_endpoint, original.health_endpoint);
}

#[test]
fn serde_roundtrip_universal_service_registration() {
    let reg = UniversalServiceRegistration {
        service_id: Uuid::nil(),
        metadata: ServiceMetadata {
            name: "reg".to_string(),
            category: ServiceCategory::AI,
            version: "1".to_string(),
            description: "d".to_string(),
            health_endpoint: None,
            metrics_endpoint: None,
        },
        capabilities: vec![ServiceCapability::Storage(StorageType::Object)],
        resources: ResourceSpec::default(),
        endpoints: vec![ServiceEndpoint {
            url: "http://localhost:8080".to_string(),
            protocol: CommunicationProtocol::Http,
            health_check: Some("/h".to_string()),
        }],
        integration: IntegrationPreferences::default(),
        extensions: [("k".to_string(), json!("v"))].into_iter().collect(),
    };
    let json = serde_json::to_string(&reg).expect("serialize");
    let back: UniversalServiceRegistration = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.service_id, reg.service_id);
    assert_eq!(back.metadata.name, "reg");
    assert_eq!(back.capabilities.len(), 1);
    assert_eq!(back.extensions.get("k"), Some(&json!("v")));
}

#[test]
fn service_category_custom_roundtrips() {
    let c = ServiceCategory::Custom("edge".to_string());
    let v = serde_json::to_value(&c).expect("to_value");
    let back: ServiceCategory = serde_json::from_value(v).expect("from_value");
    assert_eq!(back, c);
}

#[test]
fn discovered_service_default_has_localhost_and_http_port() {
    use nestgate_config::constants::hardcoding::{addresses, ports};

    let d = DiscoveredService::default();
    assert_eq!(d.endpoint, addresses::LOCALHOST_NAME);
    assert_eq!(d.port, ports::HTTP_DEFAULT);
    assert!(d.capabilities.is_empty());
}

#[test]
fn service_capability_variants_serde() {
    let cap = ServiceCapability::Custom {
        namespace: "ns".into(),
        capability: "cap".into(),
        version: "1".into(),
    };
    let j = serde_json::to_string(&cap).unwrap();
    let back: ServiceCapability = serde_json::from_str(&j).unwrap();
    assert_eq!(back, cap);
}

#[test]
fn cost_sensitivity_ordering_distinct() {
    assert_ne!(CostSensitivity::None, CostSensitivity::High);
    let s = serde_json::to_string(&CostSensitivity::Low).unwrap();
    assert!(s.contains("Low") || s.contains("low"));
}

#[test]
fn service_role_roundtrip() {
    let role = ServiceRole {
        name: "r1".into(),
        required_capabilities: vec![ServiceCapability::Storage(StorageType::Cache)],
        optional_capabilities: vec![],
        resource_requirements: ResourceSpec {
            cpu_cores: Some(1.0),
            memory_mb: Some(512),
            disk_gb: None,
            network_mbps: None,
            constraints: ResourceConstraints::default(),
        },
        performance_requirements: PerformanceRequirements::default(),
    };
    let j = serde_json::to_string(&role).unwrap();
    let back: ServiceRole = serde_json::from_str(&j).unwrap();
    assert_eq!(back.name, role.name);
}

#[test]
fn service_handle_and_info_serde() {
    let h = ServiceHandle {
        service_id: Uuid::nil(),
        name: "n".into(),
        endpoints: vec![ServiceEndpoint {
            url: "http://x".into(),
            protocol: CommunicationProtocol::Grpc,
            health_check: None,
        }],
    };
    let _: ServiceHandle = serde_json::from_str(&serde_json::to_string(&h).unwrap()).unwrap();

    let info = ServiceInfo {
        service_id: Uuid::nil(),
        metadata: ServiceMetadata::default(),
        capabilities: vec![],
        endpoints: vec![],
        last_seen: std::time::SystemTime::UNIX_EPOCH,
    };
    let _: ServiceInfo = serde_json::from_str(&serde_json::to_string(&info).unwrap()).unwrap();
}

#[test]
fn consistency_and_durability_enums_roundtrip() {
    let c = ConsistencyLevel::Session;
    let d = DurabilityLevel::Replicated;
    assert_eq!(
        serde_json::from_str::<ConsistencyLevel>(&serde_json::to_string(&c).unwrap()).unwrap(),
        c
    );
    assert_eq!(
        serde_json::from_str::<DurabilityLevel>(&serde_json::to_string(&d).unwrap()).unwrap(),
        d
    );
}
