// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # Load Balancing Algorithm Implementations
//! Algorithms functionality and utilities.
// Basic load balancing algorithms (Round Robin, Random, Least Connections)

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use std::sync::Arc;

use super::core::{LoadBalancer, LoadBalancerStats, ServiceStats};
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
                algorithm: "round_robin".to_string(),
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
        let mut stats = self.stats.write();
        stats.total_requests += 1;
        stats
            .service_stats
            .entry(service.name.clone())
            .or_default()
            .requests += 1;
        Ok(())
    }

    /// Updates  Weights
    async fn update_weights(&self, _weights: HashMap<String, f64>) -> Result<()> {
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
    connection_counts: Arc<dashmap::DashMap<String, ServiceStats>>,
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
                algorithm: "least_connections".to_string(),
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

        // Find service with least connections
        let mut min_connections = u64::MAX;
        let mut selected_service = None;

        for service in services {
            let connections = self
                .connection_counts
                .get(&service.name)
                .map_or(0, |stats| stats.requests);

            if connections < min_connections {
                min_connections = connections;
                selected_service = Some(service.clone());
            }
        }

        selected_service.ok_or_else(|| {
            NestGateError::LoadBalancer(Box::new(
                crate::error::variants::core_errors::LoadBalancerErrorDetails {
                    message: "Failed to select service with least connections".into(),
                    available_services: Some(services.len()),
                    algorithm: Some("least_connections".into()),
                },
            ))
        })
    }

    /// Record Response
    async fn record_response(
        &self,
        service: &ServiceInfo,
        _response: &ServiceResponse,
    ) -> Result<()> {
        self.connection_counts
            .entry(service.name.clone())
            .or_default()
            .requests += 1;

        let mut stats = self.stats.write();
        stats.total_requests += 1;
        stats
            .service_stats
            .entry(service.name.clone())
            .or_default()
            .requests += 1;
        Ok(())
    }

    /// Updates  Weights
    async fn update_weights(&self, _weights: HashMap<String, f64>) -> Result<()> {
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

/// Random load balancer
pub struct RandomLoadBalancer {
    rng: Arc<std::sync::Mutex<StdRng>>,
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
            rng: Arc::new(std::sync::Mutex::new(StdRng::from_entropy())),
            stats: Arc::new(parking_lot::RwLock::new(LoadBalancerStats {
                algorithm: "random".to_string(),
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

        let mut rng = self.rng.lock().map_err(|_| {
            NestGateError::LoadBalancer(Box::new(
                crate::error::variants::core_errors::LoadBalancerErrorDetails {
                    message: "Random number generator lock poisoned".into(),
                    available_services: Some(services.len()),
                    algorithm: Some("random".into()),
                },
            ))
        })?;
        let index = rng.gen_range(0..services.len());
        let selected = services[index].clone();
        drop(rng);

        let mut stats = self.stats.write();
        stats.total_requests += 1;
        stats
            .service_stats
            .entry(selected.name.clone())
            .or_default()
            .requests += 1;

        Ok(selected)
    }

    /// Record Response
    async fn record_response(
        &self,
        service: &ServiceInfo,
        _response: &ServiceResponse,
    ) -> Result<()> {
        let mut stats = self.stats.write();
        stats
            .service_stats
            .entry(service.name.clone())
            .or_default()
            .requests += 1;
        Ok(())
    }

    /// Updates  Weights
    async fn update_weights(&self, _weights: HashMap<String, f64>) -> Result<()> {
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
    use std::time::SystemTime;

    fn svc(name: &str) -> ServiceInfo {
        ServiceInfo {
            id: format!("id-{name}"),
            name: name.to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec![],
            status: ServiceStatus::Healthy,
            last_seen: SystemTime::UNIX_EPOCH,
        }
    }

    fn dummy_request() -> ServiceRequest {
        ServiceRequest {
            service_id: "svc".to_string(),
            action: "ping".to_string(),
            parameters: Default::default(),
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
        let err = lb
            .update_weights(Default::default())
            .await
            .expect_err("test: rr weights");
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
        lb.record_response(&first, &ok_response())
            .await
            .expect("test: lc record heavy");
        let second = lb
            .select_service(&services, &req)
            .await
            .expect("test: lc second");
        assert_eq!(second.name, "light");
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
        assert!(lb.update_weights(Default::default()).await.is_err());
    }
}
