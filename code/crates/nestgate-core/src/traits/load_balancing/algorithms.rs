// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Load Balancing Algorithm Implementations
//! Algorithms functionality and utilities.
// Basic load balancing algorithms (Round Robin, Random, Least Connections)

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use super::core::{LoadBalancer, LoadBalancerStats};
use crate::universal_traits::{ServiceInfo, ServiceRequest, ServiceResponse};
use crate::{NestGateError, Result};

/// Round-robin load balancer
pub struct RoundRobinLoadBalancer {
    counter: Arc<std::sync::atomic::AtomicUsize>,
    stats: Arc<parking_lot::RwLock<LoadBalancerStats>>,
}
impl RoundRobinLoadBalancer {
    /// Creates a new round-robin load balancer
    #[must_use]
    pub fn new() -> Self {
        Self {
            counter: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            stats: Arc::new(parking_lot::RwLock::new(LoadBalancerStats {
                algorithm: "round_robin".into(),
                ..LoadBalancerStats::default()
            })),
        }
    }
}

impl Default for RoundRobinLoadBalancer {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl LoadBalancer for RoundRobinLoadBalancer {
    /// Select Service
    async fn select_service(
        &self,
        services: &[ServiceInfo],
        _request: &ServiceRequest,
    ) -> Result<ServiceInfo> {
        if services.is_empty() {
            return Err(NestGateError::LoadBalancer(Box::new(
                crate::error::variants::core_errors::LoadBalancerErrorDetails {
                    message: "No services available".into(),
                    available_services: Some(0),
                    algorithm: Some("round_robin".into()),
                },
            )));
        }

        let index = self
            .counter
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            % services.len();
        Ok(services[index].clone())
    }

    /// Record Response
    async fn record_response(
        &self,
        service: &ServiceInfo,
        _response: &ServiceResponse,
    ) -> Result<()> {
        {
            let mut stats = self.stats.write();
            stats.total_requests += 1;
            stats
                .service_stats
                .entry(service.name.clone())
                .or_default()
                .requests += 1;
        }
        Ok(())
    }

    /// Updates  Weights
    async fn update_weights(&self, _weights: &[(&str, f64)]) -> Result<()> {
        Err(NestGateError::NotImplemented(Box::new(
            crate::error::variants::core_errors::NotImplementedErrorDetails {
                feature: "update_weights".into(),
                message: Some("Round-robin does not support weights".into()),
                planned_version: None,
            },
        )))
    }

    /// Gets Stats
    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

    /// Algorithm
    fn algorithm(&self) -> &'static str {
        "round_robin"
    }
}

/// Least connections load balancer
pub struct LeastConnectionsLoadBalancer {
    connection_counts: Arc<dashmap::DashMap<String, Arc<AtomicU64>>>,
    stats: Arc<parking_lot::RwLock<LoadBalancerStats>>,
}
impl LeastConnectionsLoadBalancer {
    /// Creates a new least connections load balancer.
    ///
    /// The balancer tracks active connections per endpoint and routes
    /// new requests to the endpoint with the fewest active connections.
    #[must_use]
    pub fn new() -> Self {
        Self {
            connection_counts: Arc::new(dashmap::DashMap::new()),
            stats: Arc::new(parking_lot::RwLock::new(LoadBalancerStats {
                algorithm: "least_connections".into(),
                ..LoadBalancerStats::default()
            })),
        }
    }
}

impl Default for LeastConnectionsLoadBalancer {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl LeastConnectionsLoadBalancer {
    fn active_connections(&self, service_name: &str) -> u64 {
        self.connection_counts
            .get(service_name)
            .map_or(0, |counter| counter.load(Ordering::Relaxed))
    }

    fn increment_active_connections(&self, service_name: &str) {
        self.connection_counts
            .entry(service_name.to_string())
            .or_insert_with(|| Arc::new(AtomicU64::new(0)))
            .fetch_add(1, Ordering::Relaxed);
    }

    fn decrement_active_connections(&self, service_name: &str) {
        if let Some(counter) = self.connection_counts.get(service_name) {
            counter
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                    current.checked_sub(1)
                })
                .ok();
        }
    }
}

impl LoadBalancer for LeastConnectionsLoadBalancer {
    /// Select Service
    async fn select_service(
        &self,
        services: &[ServiceInfo],
        _request: &ServiceRequest,
    ) -> Result<ServiceInfo> {
        if services.is_empty() {
            return Err(NestGateError::LoadBalancer(Box::new(
                crate::error::variants::core_errors::LoadBalancerErrorDetails {
                    message: "No services available".into(),
                    available_services: Some(0),
                    algorithm: Some("least_connections".into()),
                },
            )));
        }

        let mut min_connections = u64::MAX;
        let mut selected_service = None;

        for service in services {
            let connections = self.active_connections(&service.name);
            if connections < min_connections {
                min_connections = connections;
                selected_service = Some(service.clone());
            }
        }

        let Some(selected) = selected_service else {
            return Err(NestGateError::LoadBalancer(Box::new(
                crate::error::variants::core_errors::LoadBalancerErrorDetails {
                    message: "Failed to select service with least connections".into(),
                    available_services: Some(services.len()),
                    algorithm: Some("least_connections".into()),
                },
            )));
        };

        self.increment_active_connections(&selected.name);

        {
            let mut stats = self.stats.write();
            stats.total_requests += 1;
            stats
                .service_stats
                .entry(selected.name.clone())
                .or_default()
                .requests += 1;
        }

        Ok(selected)
    }

    /// Record Response
    async fn record_response(
        &self,
        service: &ServiceInfo,
        _response: &ServiceResponse,
    ) -> Result<()> {
        self.decrement_active_connections(&service.name);

        {
            let mut stats = self.stats.write();
            stats.total_requests += 1;
            stats
                .service_stats
                .entry(service.name.clone())
                .or_default()
                .requests += 1;
        }
        Ok(())
    }

    /// Updates  Weights
    async fn update_weights(&self, _weights: &[(&str, f64)]) -> Result<()> {
        Err(NestGateError::NotImplemented(Box::new(
            crate::error::variants::core_errors::NotImplementedErrorDetails {
                feature: "update_weights".into(),
                message: Some("Least connections does not support weights".into()),
                planned_version: None,
            },
        )))
    }

    /// Gets Stats
    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

    /// Algorithm
    fn algorithm(&self) -> &'static str {
        "least_connections"
    }
}

/// Resource-based load balancer
pub struct ResourceBasedLoadBalancer {
    resource_capacity: Arc<dashmap::DashMap<String, (f64, f64)>>,
    stats: Arc<parking_lot::RwLock<LoadBalancerStats>>,
}

impl ResourceBasedLoadBalancer {
    /// Creates a new resource-based load balancer.
    ///
    /// Routes requests to the endpoint with the highest available CPU and memory
    /// capacity according to a weighted scoring function.
    #[must_use]
    pub fn new() -> Self {
        Self {
            resource_capacity: Arc::new(dashmap::DashMap::new()),
            stats: Arc::new(parking_lot::RwLock::new(LoadBalancerStats {
                algorithm: "resource_based".into(),
                ..LoadBalancerStats::default()
            })),
        }
    }

    /// Updates available CPU and memory capacity for services.
    ///
    /// Each tuple is `(service_name, cpu_available, memory_available)` with values
    /// in the range `0.0` (fully utilized) to `1.0` (fully available).
    pub fn update_resource_capacity(&self, capacities: &[(&str, f64, f64)]) -> Result<()> {
        for (name, cpu_available, memory_available) in capacities {
            if !cpu_available.is_finite() || !memory_available.is_finite() {
                return Err(NestGateError::LoadBalancer(Box::new(
                    crate::error::variants::core_errors::LoadBalancerErrorDetails {
                        message: format!(
                            "Invalid resource capacity for service {name}: \
                             cpu={cpu_available}, memory={memory_available}"
                        )
                        .into(),
                        available_services: None,
                        algorithm: Some("resource_based".into()),
                    },
                )));
            }

            self.resource_capacity.insert(
                (*name).to_string(),
                (
                    cpu_available.clamp(0.0, 1.0),
                    memory_available.clamp(0.0, 1.0),
                ),
            );
        }
        Ok(())
    }

    fn resource_score(&self, service_name: &str) -> f64 {
        let (cpu_available, memory_available) = self
            .resource_capacity
            .get(service_name)
            .map_or((1.0, 1.0), |entry| *entry.value());

        f64::midpoint(
            cpu_available.clamp(0.0, 1.0),
            memory_available.clamp(0.0, 1.0),
        )
    }
}

impl Default for ResourceBasedLoadBalancer {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl LoadBalancer for ResourceBasedLoadBalancer {
    /// Select Service
    async fn select_service(
        &self,
        services: &[ServiceInfo],
        _request: &ServiceRequest,
    ) -> Result<ServiceInfo> {
        if services.is_empty() {
            return Err(NestGateError::LoadBalancer(Box::new(
                crate::error::variants::core_errors::LoadBalancerErrorDetails {
                    message: "No services available".into(),
                    available_services: Some(0),
                    algorithm: Some("resource_based".into()),
                },
            )));
        }

        let mut best_score = f64::NEG_INFINITY;
        let mut selected_service = None;

        for service in services {
            let score = self.resource_score(&service.name);
            if score > best_score {
                best_score = score;
                selected_service = Some(service.clone());
            }
        }

        let Some(selected) = selected_service else {
            return Err(NestGateError::LoadBalancer(Box::new(
                crate::error::variants::core_errors::LoadBalancerErrorDetails {
                    message: "Failed to select service by resource availability".into(),
                    available_services: Some(services.len()),
                    algorithm: Some("resource_based".into()),
                },
            )));
        };

        {
            let mut stats = self.stats.write();
            stats.total_requests += 1;
            stats
                .service_stats
                .entry(selected.name.clone())
                .or_default()
                .requests += 1;
        }

        Ok(selected)
    }

    /// Record Response
    async fn record_response(
        &self,
        service: &ServiceInfo,
        _response: &ServiceResponse,
    ) -> Result<()> {
        self.stats
            .write()
            .service_stats
            .entry(service.name.clone())
            .or_default()
            .requests += 1;
        Ok(())
    }

    /// Updates  Weights
    async fn update_weights(&self, _weights: &[(&str, f64)]) -> Result<()> {
        Err(NestGateError::NotImplemented(Box::new(
            crate::error::variants::core_errors::NotImplementedErrorDetails {
                feature: "update_weights".into(),
                message: Some("Resource-based balancer uses update_resource_capacity".into()),
                planned_version: None,
            },
        )))
    }

    /// Gets Stats
    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

    /// Algorithm
    fn algorithm(&self) -> &'static str {
        "resource_based"
    }
}

/// Random load balancer
pub struct RandomLoadBalancer {
    rng: Arc<parking_lot::Mutex<StdRng>>,
    stats: Arc<parking_lot::RwLock<LoadBalancerStats>>,
}
impl RandomLoadBalancer {
    /// Creates a new random load balancer.
    ///
    /// The balancer uses cryptographically secure randomness to distribute
    /// requests evenly across available endpoints.
    #[must_use]
    pub fn new() -> Self {
        Self {
            rng: Arc::new(parking_lot::Mutex::new(StdRng::from_os_rng())),
            stats: Arc::new(parking_lot::RwLock::new(LoadBalancerStats {
                algorithm: "random".into(),
                ..LoadBalancerStats::default()
            })),
        }
    }
}

impl Default for RandomLoadBalancer {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl LoadBalancer for RandomLoadBalancer {
    /// Select Service
    async fn select_service(
        &self,
        services: &[ServiceInfo],
        _request: &ServiceRequest,
    ) -> Result<ServiceInfo> {
        if services.is_empty() {
            return Err(NestGateError::LoadBalancer(Box::new(
                crate::error::variants::core_errors::LoadBalancerErrorDetails {
                    message: "No services available".into(),
                    available_services: Some(0),
                    algorithm: Some("random".into()),
                },
            )));
        }

        let index = {
            let mut rng = self.rng.lock();
            rng.random_range(0..services.len())
        };
        let selected = services[index].clone();

        {
            let mut stats = self.stats.write();
            stats.total_requests += 1;
            stats
                .service_stats
                .entry(selected.name.clone())
                .or_default()
                .requests += 1;
        }

        Ok(selected)
    }

    /// Record Response
    async fn record_response(
        &self,
        service: &ServiceInfo,
        _response: &ServiceResponse,
    ) -> Result<()> {
        self.stats
            .write()
            .service_stats
            .entry(service.name.clone())
            .or_default()
            .requests += 1;
        Ok(())
    }

    /// Updates  Weights
    async fn update_weights(&self, _weights: &[(&str, f64)]) -> Result<()> {
        Err(NestGateError::NotImplemented(Box::new(
            crate::error::variants::core_errors::NotImplementedErrorDetails {
                feature: "update_weights".into(),
                message: Some("Random load balancer does not support weights".into()),
                planned_version: None,
            },
        )))
    }

    /// Gets Stats
    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

    /// Algorithm
    fn algorithm(&self) -> &'static str {
        "random"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::load_balancing::core::LoadBalancer;
    use crate::universal_traits::orchestration::ServiceStatus;
    use crate::universal_traits::{ServiceInfo, ServiceRequest, ServiceResponse};
    use std::collections::HashMap;
    use std::time::SystemTime;

    fn svc(name: &str) -> ServiceInfo {
        ServiceInfo {
            id: format!("id-{name}"),
            name: name.to_string(),
            version: "1.0.0".into(),
            capabilities: vec![],
            status: ServiceStatus::Healthy,
            last_seen: SystemTime::UNIX_EPOCH,
        }
    }

    fn dummy_request() -> ServiceRequest {
        ServiceRequest {
            service_id: "svc".into(),
            action: "ping".into(),
            parameters: HashMap::default(),
            timeout_seconds: Some(5),
        }
    }

    fn ok_response() -> ServiceResponse {
        ServiceResponse {
            success: true,
            data: None,
            error_message: None,
            execution_time_ms: 1,
        }
    }

    #[tokio::test]
    async fn round_robin_cycles_through_services_in_order() {
        let lb = RoundRobinLoadBalancer::new();
        let services = vec![svc("a"), svc("b"), svc("c")];
        let req = dummy_request();
        let s0 = lb
            .select_service(&services, &req)
            .await
            .expect("test: rr select 0");
        let s1 = lb
            .select_service(&services, &req)
            .await
            .expect("test: rr select 1");
        let s2 = lb
            .select_service(&services, &req)
            .await
            .expect("test: rr select 2");
        let s3 = lb
            .select_service(&services, &req)
            .await
            .expect("test: rr select 3");
        assert_eq!(s0.name, "a");
        assert_eq!(s1.name, "b");
        assert_eq!(s2.name, "c");
        assert_eq!(s3.name, "a");
        assert_eq!(lb.algorithm(), "round_robin");
    }

    #[tokio::test]
    async fn round_robin_empty_services_errors() {
        let lb = RoundRobinLoadBalancer::new();
        let err = lb
            .select_service(&[], &dummy_request())
            .await
            .expect_err("test: rr empty");
        assert!(err.to_string().contains("No services") || err.to_string().contains("available"));
    }

    #[tokio::test]
    async fn round_robin_record_response_updates_stats() {
        let lb = RoundRobinLoadBalancer::new();
        let a = svc("a");
        lb.record_response(&a, &ok_response())
            .await
            .expect("test: rr record");
        let stats = lb.get_stats().await.expect("test: rr stats");
        assert_eq!(stats.total_requests, 1);
        assert_eq!(stats.service_stats.get("a").map(|s| s.requests), Some(1));
    }

    #[tokio::test]
    async fn round_robin_update_weights_not_supported() {
        let lb = RoundRobinLoadBalancer::new();
        let err = lb.update_weights(&[]).await.expect_err("test: rr weights");
        assert!(err.to_string().contains("NotImplemented") || err.to_string().contains("weights"));
    }

    #[tokio::test]
    async fn least_connections_prefers_lower_load_then_first_tie() {
        let lb = LeastConnectionsLoadBalancer::new();
        let a = svc("heavy");
        let b = svc("light");
        let services = vec![a.clone(), b.clone()];
        let req = dummy_request();
        let first = lb
            .select_service(&services, &req)
            .await
            .expect("test: lc first");
        assert_eq!(first.name, "heavy");
        let second = lb
            .select_service(&services, &req)
            .await
            .expect("test: lc second");
        assert_eq!(second.name, "light");
        lb.record_response(&first, &ok_response())
            .await
            .expect("test: lc record heavy");
        lb.record_response(&second, &ok_response())
            .await
            .expect("test: lc record light");
        assert_eq!(lb.algorithm(), "least_connections");
    }

    #[tokio::test]
    async fn least_connections_empty_services_errors() {
        let lb = LeastConnectionsLoadBalancer::new();
        let err = lb
            .select_service(&[], &dummy_request())
            .await
            .expect_err("test: lc empty");
        assert!(err.to_string().contains("No services") || err.to_string().contains("available"));
    }

    #[tokio::test]
    async fn random_balancer_selects_from_non_empty_list() {
        let lb = RandomLoadBalancer::new();
        let services = vec![svc("x"), svc("y")];
        let picked = lb
            .select_service(&services, &dummy_request())
            .await
            .expect("test: random pick");
        assert!(picked.name == "x" || picked.name == "y");
        let stats = lb.get_stats().await.expect("test: random stats");
        assert!(stats.total_requests >= 1);
        assert_eq!(lb.algorithm(), "random");
    }

    #[tokio::test]
    async fn random_balancer_empty_services_errors() {
        let lb = RandomLoadBalancer::new();
        let err = lb
            .select_service(&[], &dummy_request())
            .await
            .expect_err("test: random empty");
        assert!(err.to_string().contains("No services") || err.to_string().contains("available"));
    }

    #[tokio::test]
    async fn least_connections_update_weights_not_supported() {
        let lb = LeastConnectionsLoadBalancer::new();
        assert!(lb.update_weights(&[]).await.is_err());
    }

    #[tokio::test]
    async fn random_balancer_record_response_and_update_weights() {
        let lb = RandomLoadBalancer::new();
        let s = svc("z");
        lb.record_response(&s, &ok_response())
            .await
            .expect("test: random record");
        let stats = lb.get_stats().await.expect("test: random stats");
        assert!(stats.service_stats.get("z").map_or(0, |x| x.requests) >= 1);
        assert!(lb.update_weights(&[]).await.is_err());
    }

    #[tokio::test]
    async fn resource_based_selects_highest_capacity_then_first_tie() {
        let lb = ResourceBasedLoadBalancer::new();
        let low = svc("low");
        let high = svc("high");
        let services = vec![low.clone(), high.clone()];
        lb.update_resource_capacity(&[("low", 0.2, 0.2), ("high", 0.9, 0.8)])
            .expect("test: rb update capacity");

        let picked = lb
            .select_service(&services, &dummy_request())
            .await
            .expect("test: rb select");
        assert_eq!(picked.name, "high");
        assert_eq!(lb.algorithm(), "resource_based");
    }

    #[tokio::test]
    async fn resource_based_empty_services_errors() {
        let lb = ResourceBasedLoadBalancer::new();
        let err = lb
            .select_service(&[], &dummy_request())
            .await
            .expect_err("test: rb empty");
        assert!(err.to_string().contains("No services") || err.to_string().contains("available"));
    }

    #[tokio::test]
    async fn resource_based_rejects_non_finite_capacity() {
        let lb = ResourceBasedLoadBalancer::new();
        let err = lb
            .update_resource_capacity(&[("bad", f64::NAN, 0.5)])
            .expect_err("test: rb invalid capacity");
        assert!(err.to_string().contains("Invalid resource capacity"));
    }

    #[tokio::test]
    async fn resource_based_record_response_and_update_weights() {
        let lb = ResourceBasedLoadBalancer::new();
        let service = svc("node");
        lb.record_response(&service, &ok_response())
            .await
            .expect("test: rb record");
        let stats = lb.get_stats().await.expect("test: rb stats");
        assert_eq!(
            stats
                .service_stats
                .get("node")
                .expect("test: node stats")
                .requests,
            1
        );
        assert!(lb.update_weights(&[]).await.is_err());
    }
}
