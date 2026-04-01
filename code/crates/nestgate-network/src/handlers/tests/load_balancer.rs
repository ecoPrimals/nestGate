// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use crate::handlers::{LoadBalancer, LoadBalancingStrategy};

use super::create_test_service;

#[tokio::test]
async fn test_load_balancer_round_robin() {
    let lb = LoadBalancer::new(LoadBalancingStrategy::RoundRobin);

    lb.add_service(create_test_service("svc1", "service1", true))
        .await;
    lb.add_service(create_test_service("svc2", "service2", true))
        .await;
    lb.add_service(create_test_service("svc3", "service3", true))
        .await;

    // Get next service 3 times - should cycle through all
    let svc1 = lb
        .get_next_service()
        .await
        .expect("Should get first service");
    let svc2 = lb
        .get_next_service()
        .await
        .expect("Should get second service");
    let svc3 = lb
        .get_next_service()
        .await
        .expect("Should get third service");
    let svc4 = lb
        .get_next_service()
        .await
        .expect("Should wrap to first service");

    assert_eq!(svc1.name(), "service1");
    assert_eq!(svc2.name(), "service2");
    assert_eq!(svc3.name(), "service3");
    assert_eq!(svc4.name(), "service1"); // Wrapped around
}

#[tokio::test]
async fn test_load_balancer_random() {
    let lb = LoadBalancer::new(LoadBalancingStrategy::Random);

    lb.add_service(create_test_service("svc1", "service1", true))
        .await;
    lb.add_service(create_test_service("svc2", "service2", true))
        .await;

    // Random should return some service
    let svc = lb.get_next_service().await;
    assert!(svc.is_some());
}

#[tokio::test]
async fn test_load_balancer_least_connections() {
    let lb = LoadBalancer::new(LoadBalancingStrategy::LeastConnections);

    lb.add_service(create_test_service("svc1", "service1", true))
        .await;
    lb.add_service(create_test_service("svc2", "service2", true))
        .await;

    let svc = lb.get_next_service().await;
    assert!(svc.is_some());
}

#[tokio::test]
async fn test_load_balancer_empty_services() {
    let lb = LoadBalancer::new(LoadBalancingStrategy::RoundRobin);

    let svc = lb.get_next_service().await;
    assert!(svc.is_none());
}

#[tokio::test]
async fn test_load_balancer_add_remove_service() {
    let lb = LoadBalancer::new(LoadBalancingStrategy::RoundRobin);

    lb.add_service(create_test_service("svc1", "service1", true))
        .await;
    lb.add_service(create_test_service("svc2", "service2", true))
        .await;

    // Verify we can remove a service
    let removed = lb.remove_service("svc1").await;
    assert!(removed);

    // After removal, should still work with remaining service
    let svc = lb.get_next_service().await;
    assert!(svc.is_some());
    assert_eq!(
        svc.expect("Should get remaining service").name(),
        "service2"
    );
}

#[tokio::test]
async fn test_load_balancer_remove_nonexistent() {
    let lb = LoadBalancer::new(LoadBalancingStrategy::RoundRobin);

    lb.add_service(create_test_service("svc1", "service1", true))
        .await;

    let removed = lb.remove_service("nonexistent").await;
    assert!(!removed);
}
