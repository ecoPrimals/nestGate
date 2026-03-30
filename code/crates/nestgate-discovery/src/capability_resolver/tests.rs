// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use crate::capability_resolver::types::CapabilityResolver;
use crate::unified_capabilities::{CapabilityMapper, UnifiedCapability};

use super::{CompositeResolver, EnvironmentResolver, InMemoryRegistryAdapter, ResolvedService};

#[test]
fn resolved_service_url_and_endpoint() {
    let s = ResolvedService {
        id: "1".into(),
        host: "127.0.0.1".into(),
        port: 8080,
        protocol: "http".into(),
        capabilities: vec![],
        is_healthy: true,
    };
    assert_eq!(s.url(), "http://127.0.0.1:8080");
    assert_eq!(s.endpoint(), "127.0.0.1:8080");
}

#[test]
fn resolved_service_url_includes_non_http_scheme() {
    let s = ResolvedService {
        id: "g".into(),
        host: "10.0.0.2".into(),
        port: 9090,
        protocol: "grpc".into(),
        capabilities: vec![],
        is_healthy: true,
    };
    assert_eq!(s.url(), "grpc://10.0.0.2:9090");
}

#[test]
fn capability_mapper_env_var_name_format() {
    let name = CapabilityMapper::env_var_name(&UnifiedCapability::Storage);
    assert!(name.contains("STORAGE"));
    assert!(name.starts_with("NESTGATE_CAPABILITY_"));
    assert!(name.ends_with("_ENDPOINT"));
}

#[tokio::test]
async fn composite_resolver_empty_chain_fails() {
    let c = CompositeResolver::new();
    let err = c
        .resolve_capability(&UnifiedCapability::Storage)
        .await
        .expect_err("no resolvers");
    assert!(!err.to_string().is_empty());
}

#[test]
fn unified_capability_display_covers_variants() {
    use UnifiedCapability::*;
    let samples = [
        (Storage, "storage"),
        (ZfsManagement, "zfs-management"),
        (Custom("x".into()), "custom:x"),
        (ArtificialIntelligence, "ai"),
    ];
    for (cap, needle) in samples {
        let s = cap.to_string();
        assert!(s.contains(needle), "{cap:?} -> {s}");
    }
}

#[tokio::test]
async fn composite_resolve_capability_all_errors_when_empty() {
    let cap = UnifiedCapability::Storage;
    let c = CompositeResolver::new();
    let err = c.resolve_capability_all(&cap).await.expect_err("empty");
    assert!(!err.to_string().is_empty());
}

#[test]
fn environment_resolver_new_and_default() {
    let _ = EnvironmentResolver::new();
    let _ = EnvironmentResolver::default();
}

#[tokio::test]
async fn in_memory_registry_adapter_resolve_storage_service() {
    use crate::service_discovery::registry::{InMemoryServiceRegistry, UniversalServiceRegistry};
    use crate::service_discovery::types::{
        CommunicationProtocol, IntegrationPreferences, ResourceSpec, ServiceCapability,
        ServiceEndpoint as SvcEp, ServiceMetadata, StorageType, UniversalServiceRegistration,
    };
    use uuid::Uuid;

    let reg = InMemoryServiceRegistry::new();
    let sid = Uuid::new_v4();
    let registration = UniversalServiceRegistration {
        service_id: sid,
        metadata: ServiceMetadata {
            name: "storage-a".into(),
            ..Default::default()
        },
        capabilities: vec![ServiceCapability::Storage(StorageType::Object)],
        resources: ResourceSpec::default(),
        endpoints: vec![SvcEp {
            url: "http://127.0.0.1:8080/path".into(),
            protocol: CommunicationProtocol::Http,
            health_check: None,
        }],
        integration: IntegrationPreferences::default(),
        extensions: Default::default(),
    };
    reg.register_service(registration).await.expect("register");

    let adapter = InMemoryRegistryAdapter::new(&reg);
    let resolved = adapter
        .resolve_capability(&UnifiedCapability::Storage)
        .await
        .expect("resolve");
    assert_eq!(resolved.host, "127.0.0.1");
    assert_eq!(resolved.port, 8080);
    assert_eq!(resolved.protocol.as_ref(), "http");

    let all = adapter
        .resolve_capability_all(&UnifiedCapability::Storage)
        .await
        .expect("resolve all");
    assert_eq!(all.len(), 1);

    assert!(adapter.has_capability(&UnifiedCapability::Storage).await);
}

#[tokio::test]
async fn environment_resolver_missing_env_returns_error() {
    let cap = UnifiedCapability::Custom("round3_missing_env_only".into());
    let err = EnvironmentResolver::new()
        .resolve_capability(&cap)
        .await
        .expect_err("unset env");
    assert!(!err.to_string().is_empty());
}

#[tokio::test]
async fn composite_resolver_in_memory_plus_environment_fallback() {
    use crate::service_discovery::registry::{InMemoryServiceRegistry, UniversalServiceRegistry};
    use crate::service_discovery::types::{
        CommunicationProtocol, IntegrationPreferences, ResourceSpec, ServiceCapability,
        ServiceEndpoint as SvcEp, ServiceMetadata, StorageType, UniversalServiceRegistration,
    };
    use uuid::Uuid;

    let reg = InMemoryServiceRegistry::new();
    let sid = Uuid::new_v4();
    reg.register_service(UniversalServiceRegistration {
        service_id: sid,
        metadata: ServiceMetadata {
            name: "s".into(),
            ..Default::default()
        },
        capabilities: vec![ServiceCapability::Storage(StorageType::Object)],
        resources: ResourceSpec::default(),
        endpoints: vec![SvcEp {
            url: "http://192.168.1.10:9000".into(),
            protocol: CommunicationProtocol::Http,
            health_check: None,
        }],
        integration: IntegrationPreferences::default(),
        extensions: Default::default(),
    })
    .await
    .expect("reg");

    let composite = CompositeResolver::new()
        .with_resolver(Box::new(InMemoryRegistryAdapter::new(&reg)))
        .with_resolver(Box::new(EnvironmentResolver::new()));
    let s = composite
        .resolve_capability(&UnifiedCapability::Storage)
        .await
        .expect("composite");
    assert_eq!(s.host, "192.168.1.10");

    let merged = composite
        .resolve_capability_all(&UnifiedCapability::Storage)
        .await
        .expect("all");
    assert!(!merged.is_empty());
    assert!(composite.has_capability(&UnifiedCapability::Storage).await);
}
