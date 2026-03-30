// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use super::*;

#[test]
fn test_nestgate_self_knowledge() {
    let knowledge = NestGateSelfKnowledge::new();

    // NestGate should know its own storage capabilities
    let capabilities = knowledge.get_advertised_capabilities();
    assert!(!capabilities.is_empty());

    // Should have ZFS capabilities
    assert!(capabilities.iter().any(|c| c.operation == "create_dataset"));
    assert!(capabilities.iter().any(|c| c.operation == "list_datasets"));
}

#[test]
fn test_capability_request_builder() {
    let request = CapabilityRequest::new(CapabilityCategory::Orchestration, "deploy")
        .with_parameter("image", serde_json::json!("nginx:latest"))
        .with_timeout(60)
        .optional();

    assert_eq!(request.category, CapabilityCategory::Orchestration);
    assert_eq!(request.operation, "deploy");
    assert!(!request.required);
    assert_eq!(request.timeout_seconds, 60);
}

#[test]
fn test_service_capability_discovery() {
    // Test capability discovery logic
    let knowledge = NestGateSelfKnowledge::new();
    let caps = knowledge.get_advertised_capabilities();

    // Should have multiple capabilities advertised
    assert!(caps.len() >= 2);
}

#[test]
fn capability_category_maps_to_primal() {
    use crate::universal_primal_discovery::capability_based_discovery::PrimalCapability;
    assert!(matches!(
        CapabilityCategory::Storage.to_primal_capability(),
        PrimalCapability::ZfsStorage
    ));
    assert!(matches!(
        CapabilityCategory::Security.to_primal_capability(),
        PrimalCapability::Authentication
    ));
}

#[test]
fn capability_registry_register_and_find() {
    let mut reg = CapabilityRegistry::new();
    let mut svc = DiscoveredService::new("a", "t", "http://x");
    svc.healthy = true;
    let cap = ServiceCapability::storage("op", "d");
    let svc = svc.with_capability(cap);
    reg.register_service(svc);
    let found = reg.find_providers(&CapabilityCategory::Storage, "op");
    assert_eq!(found.len(), 1);
    reg.cleanup_unhealthy();
    assert_eq!(reg.all_services().len(), 1);
}

#[tokio::test]
async fn capability_router_routes_local_storage_create_dataset() {
    let router = CapabilityRouter::default();
    let req = CapabilityRequest::new(CapabilityCategory::Storage, "create_dataset");
    let resp = router
        .route_capability_request(req)
        .await
        .expect("local route");
    assert!(resp.success);
    assert!(resp.data.get("dataset_id").is_some());
}

#[tokio::test]
async fn capability_router_unknown_storage_operation_errors() {
    let router = CapabilityRouter::default();
    let req = CapabilityRequest::new(CapabilityCategory::Storage, "not_implemented_op");
    let err = router.route_capability_request(req).await;
    assert!(err.is_err());
}

#[tokio::test]
async fn capability_router_rejects_non_local_non_storage() {
    let router = CapabilityRouter::default();
    let req = CapabilityRequest::new(CapabilityCategory::Compute, "run");
    let err = router.route_capability_request(req).await;
    assert!(err.is_err());
}

#[test]
fn all_capability_categories_map_to_primal() {
    use crate::universal_primal_discovery::capability_based_discovery::PrimalCapability;
    let cats = [
        CapabilityCategory::Storage,
        CapabilityCategory::Orchestration,
        CapabilityCategory::Compute,
        CapabilityCategory::Security,
        CapabilityCategory::Intelligence,
        CapabilityCategory::Management,
        CapabilityCategory::Network,
        CapabilityCategory::Data,
    ];
    assert!(matches!(
        CapabilityCategory::Storage.to_primal_capability(),
        PrimalCapability::ZfsStorage
    ));
    assert!(matches!(
        CapabilityCategory::Data.to_primal_capability(),
        PrimalCapability::DataSync
    ));
    for c in cats {
        let _p = c.to_primal_capability();
    }
}

#[tokio::test]
async fn capability_router_local_list_datasets() {
    let router = CapabilityRouter::default();
    let req = CapabilityRequest::new(CapabilityCategory::Storage, "list_datasets");
    let resp = router
        .route_capability_request(req)
        .await
        .expect("list_datasets");
    assert!(resp.success);
    assert!(resp.data.get("datasets").is_some());
}

#[tokio::test]
async fn capability_router_local_management_not_implemented() {
    let router = CapabilityRouter::default();
    let req = CapabilityRequest::new(CapabilityCategory::Management, "health_check");
    let err = router.route_capability_request(req).await;
    assert!(err.is_err());
}

#[test]
fn capability_response_success_and_error_helpers() {
    let id = uuid::Uuid::nil();
    let ok = CapabilityResponse::success(id, serde_json::json!({"a": 1}));
    assert!(ok.success);
    let bad = CapabilityResponse::error(id, "e".into());
    assert!(!bad.success);
    assert_eq!(bad.error.as_deref(), Some("e"));
}

#[test]
fn capability_registry_remove_service_updates_index() {
    let mut reg = CapabilityRegistry::new();
    let mut svc = DiscoveredService::new("a", "t", "http://x");
    svc.healthy = true;
    let cap = ServiceCapability::storage("op", "d");
    let svc = svc.with_capability(cap);
    let id = svc.service_id;
    reg.register_service(svc);
    assert_eq!(
        reg.find_providers(&CapabilityCategory::Storage, "op").len(),
        1
    );
    reg.remove_service(&id);
    assert!(
        reg.find_providers(&CapabilityCategory::Storage, "op")
            .is_empty()
    );
}

#[test]
fn nestgate_self_knowledge_health_check_operation() {
    let k = NestGateSelfKnowledge::new();
    assert!(k.can_handle_capability(&CapabilityCategory::Management, "health_check"));
    assert!(!k.can_handle_capability(&CapabilityCategory::Compute, "run"));
}

#[test]
fn registry_excludes_unhealthy_providers() {
    let mut reg = CapabilityRegistry::new();
    let mut svc = DiscoveredService::new("a", "t", "http://x");
    svc.healthy = false;
    let cap = ServiceCapability::storage("op", "d");
    let svc = svc.with_capability(cap);
    reg.register_service(svc);
    assert!(
        reg.find_providers(&CapabilityCategory::Storage, "op")
            .is_empty()
    );
}

#[test]
fn discovered_service_provides_capability_negative() {
    let svc = DiscoveredService::new("a", "t", "http://x");
    assert!(!svc.provides_capability(&CapabilityCategory::Storage, "nope"));
}

#[test]
fn capability_request_optional_and_timeout() {
    let r = CapabilityRequest::new(CapabilityCategory::Storage, "list_datasets")
        .optional()
        .with_timeout(99);
    assert!(!r.required);
    assert_eq!(r.timeout_seconds, 99);
}

#[tokio::test]
async fn r6_capability_router_concurrent_list_datasets() {
    let router = CapabilityRouter::default();
    let (a, b, c) = tokio::join!(
        router.route_capability_request(CapabilityRequest::new(
            CapabilityCategory::Storage,
            "list_datasets",
        )),
        router.route_capability_request(CapabilityRequest::new(
            CapabilityCategory::Storage,
            "list_datasets",
        )),
        router.route_capability_request(CapabilityRequest::new(
            CapabilityCategory::Storage,
            "create_dataset",
        )),
    );
    assert!(a.expect("a").success);
    assert!(b.expect("b").success);
    assert!(c.expect("c").success);
}

#[test]
fn r6_capability_category_eq() {
    assert_eq!(CapabilityCategory::Storage, CapabilityCategory::Storage);
    assert_ne!(CapabilityCategory::Storage, CapabilityCategory::Compute);
}

#[test]
fn r6_service_capability_storage_constructor() {
    let c = ServiceCapability::storage("op", "desc");
    assert_eq!(c.operation, "op");
    assert!(c.description.contains("desc"));
}

#[test]
fn r6_capability_request_with_parameter() {
    let r = CapabilityRequest::new(CapabilityCategory::Storage, "x")
        .with_parameter("k", serde_json::json!(1));
    assert_eq!(r.parameters.get("k"), Some(&serde_json::json!(1)));
}
