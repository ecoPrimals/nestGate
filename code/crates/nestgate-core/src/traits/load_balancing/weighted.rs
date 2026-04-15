// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Weighted Load Balancing Algorithms
//! Weighted functionality and utilities.
// Advanced load balancing algorithms that support service weights

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use std::sync::Arc;

use super::core::{LoadBalancer, LoadBalancerStats};
use crate::universal_traits::{ServiceInfo, ServiceRequest, ServiceResponse};
use crate::{NestGateError, Result};

/// Weighted round-robin load balancer
pub struct WeightedRoundRobinLoadBalancer {
    weights: Arc<parking_lot::RwLock<HashMap<String, f64>>>,
    current_weights: Arc<parking_lot::RwLock<HashMap<String, f64>>>,
    stats: Arc<parking_lot::RwLock<LoadBalancerStats>>,
}
impl WeightedRoundRobinLoadBalancer {
    /// Creates a new weighted round-robin load balancer.
    ///
    /// The balancer distributes requests proportionally based on endpoint weights,
    /// using a smooth weighted round-robin algorithm for fair distribution.
    #[must_use]
    pub fn new() -> Self {
        Self {
            weights: Arc::new(parking_lot::RwLock::new(HashMap::new())),
            current_weights: Arc::new(parking_lot::RwLock::new(HashMap::new())),
            stats: Arc::new(parking_lot::RwLock::new(LoadBalancerStats {
                algorithm: "weighted_round_robin".to_string(),
                ..LoadBalancerStats::default()
            })),
        }
    }
}

impl Default for WeightedRoundRobinLoadBalancer {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl LoadBalancer for WeightedRoundRobinLoadBalancer {
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
                    algorithm: Some("weighted_round_robin".into()),
                },
            )));
        }

        let per_service_weight: Vec<f64> = {
            let weights = self.weights.read();
            services
                .iter()
                .map(|s| weights.get(&s.name).copied().unwrap_or(1.0))
                .collect()
        };

        let selected_service = {
            let mut current_weights = self.current_weights.write();

            // Initialize current weights if needed
            for service in services {
                if !current_weights.contains_key(&service.name) {
                    current_weights.insert(service.name.clone(), 0.0);
                }
            }

            // Find service with highest current weight
            let mut max_weight = f64::NEG_INFINITY;
            let mut selected_service = None;

            for (service, service_weight) in services.iter().zip(per_service_weight.iter()) {
                let current_weight = current_weights.get_mut(&service.name).ok_or_else(|| {
                    crate::error::NestGateError::internal_error(
                        "weighted_balancer",
                        format!("Service {} not found in weight map", service.name),
                    )
                })?;
                *current_weight += *service_weight;

                if *current_weight > max_weight {
                    max_weight = *current_weight;
                    selected_service = Some(service.clone());
                }
            }

            if let Some(ref selected) = selected_service {
                // Reduce the selected service's current weight by the total of all weights
                let total_weight: f64 = per_service_weight.iter().sum();

                if let Some(current_weight) = current_weights.get_mut(&selected.name) {
                    *current_weight -= total_weight;
                }
            }

            selected_service
        };

        if let Some(ref selected) = selected_service {
            let mut stats = self.stats.write();
            stats.total_requests += 1;
            stats
                .service_stats
                .entry(selected.name.clone())
                .or_default()
                .requests += 1;
        }

        selected_service.ok_or_else(|| {
            NestGateError::LoadBalancer(Box::new(
                crate::error::variants::core_errors::LoadBalancerErrorDetails {
                    message: "Failed to select service with weighted round-robin".into(),
                    available_services: Some(services.len()),
                    algorithm: Some("weighted_round_robin".into()),
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
        self.stats
            .write()
            .service_stats
            .entry(service.name.clone())
            .or_default()
            .requests += 1;
        Ok(())
    }

    /// Updates  Weights
    async fn update_weights(&self, weights: &[(&str, f64)]) -> Result<()> {
        let mut map = HashMap::with_capacity(weights.len());
        for (k, v) in weights {
            map.insert((*k).to_string(), *v);
        }
        *self.weights.write() = map;
        Ok(())
    }

    /// Gets Stats
    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

    /// Algorithm
    fn algorithm(&self) -> &'static str {
        "weighted_round_robin"
    }
}

/// Weighted random load balancer
pub struct WeightedRandomLoadBalancer {
    weights: Arc<parking_lot::RwLock<HashMap<String, f64>>>,
    rng: Arc<parking_lot::Mutex<StdRng>>,
    stats: Arc<parking_lot::RwLock<LoadBalancerStats>>,
}
impl WeightedRandomLoadBalancer {
    /// Creates a new weighted random load balancer.
    ///
    /// The balancer selects endpoints randomly with probability proportional
    /// to their assigned weights, providing stochastic load distribution.
    #[must_use]
    pub fn new() -> Self {
        Self {
            weights: Arc::new(parking_lot::RwLock::new(HashMap::new())),
            rng: Arc::new(parking_lot::Mutex::new(StdRng::from_os_rng())),
            stats: Arc::new(parking_lot::RwLock::new(LoadBalancerStats {
                algorithm: "weighted_random".to_string(),
                ..LoadBalancerStats::default()
            })),
        }
    }
}

impl Default for WeightedRandomLoadBalancer {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl LoadBalancer for WeightedRandomLoadBalancer {
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
                    algorithm: Some("weighted_random".into()),
                },
            )));
        }

        // Implement proper weighted random algorithm: snapshot weights, then release the read lock
        let per_service_weights: Vec<f64> = {
            let weights = self.weights.read();
            services
                .iter()
                .map(|service| weights.get(&service.name).copied().unwrap_or(1.0))
                .collect()
        };

        // Calculate total weight
        let total_weight: f64 = per_service_weights.iter().sum();

        if total_weight <= 0.0 {
            let index = {
                let mut rng = self.rng.lock();
                rng.gen_range(0..services.len())
            };
            return Ok(services[index].clone());
        }

        let random_weight = {
            let mut rng = self.rng.lock();
            rng.r#gen::<f64>() * total_weight
        };

        // Find the service corresponding to this weight
        let mut cumulative_weight = 0.0;
        for (service, service_weight) in services.iter().zip(per_service_weights.iter()) {
            cumulative_weight += service_weight;

            if random_weight < cumulative_weight {
                {
                    let mut stats = self.stats.write();
                    stats.total_requests += 1;
                    stats
                        .service_stats
                        .entry(service.name.clone())
                        .or_default()
                        .requests += 1;
                }

                return Ok(service.clone());
            }
        }

        // Fallback to last service (shouldn't happen with correct implementation)
        Ok(services[services.len() - 1].clone())
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
    async fn update_weights(&self, weights: &[(&str, f64)]) -> Result<()> {
        let mut map = HashMap::with_capacity(weights.len());
        for (k, v) in weights {
            map.insert((*k).to_string(), *v);
        }
        *self.weights.write() = map;
        Ok(())
    }

    /// Gets Stats
    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

    /// Algorithm
    fn algorithm(&self) -> &'static str {
        "weighted_random"
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
    async fn weighted_round_robin_empty_services_errors() {
        let lb = WeightedRoundRobinLoadBalancer::new();
        let err = lb
            .select_service(&[], &dummy_request())
            .await
            .expect_err("test: wrr empty must error");
        assert!(err.to_string().contains("No services") || err.to_string().contains("available"));
        assert_eq!(lb.algorithm(), "weighted_round_robin");
    }

    #[tokio::test]
    async fn weighted_round_robin_selects_and_updates_stats() {
        let lb = WeightedRoundRobinLoadBalancer::new();
        let services = vec![svc("a"), svc("b")];
        let req = dummy_request();

        let s0 = lb
            .select_service(&services, &req)
            .await
            .expect("test: wrr first select");
        assert!(s0.name == "a" || s0.name == "b");

        lb.update_weights(&[("a", 2.0), ("b", 1.0)])
            .await
            .expect("test: wrr update weights");

        let _ = lb
            .select_service(&services, &req)
            .await
            .expect("test: wrr second select");

        let stats = lb.get_stats().await.expect("test: wrr get stats");
        assert_eq!(stats.algorithm, "weighted_round_robin");
        assert!(stats.total_requests >= 2);
    }

    #[tokio::test]
    async fn weighted_round_robin_record_response_increments_service_stats() {
        let lb = WeightedRoundRobinLoadBalancer::new();
        let service = svc("x");
        lb.record_response(&service, &ok_response())
            .await
            .expect("test: wrr record response");
        let stats = lb.get_stats().await.expect("test: wrr stats after record");
        assert_eq!(
            stats
                .service_stats
                .get("x")
                .expect("test: x stats")
                .requests,
            1
        );
    }

    #[tokio::test]
    async fn weighted_round_robin_default_matches_new() {
        let a = WeightedRoundRobinLoadBalancer::new();
        let b = WeightedRoundRobinLoadBalancer::default();
        assert_eq!(a.algorithm(), b.algorithm());
    }

    #[tokio::test]
    async fn weighted_random_empty_services_errors() {
        let lb = WeightedRandomLoadBalancer::new();
        let err = lb
            .select_service(&[], &dummy_request())
            .await
            .expect_err("test: wr empty must error");
        assert!(err.to_string().contains("No services") || err.to_string().contains("available"));
        assert_eq!(lb.algorithm(), "weighted_random");
    }

    #[tokio::test]
    async fn weighted_random_selects_from_list() {
        let lb = WeightedRandomLoadBalancer::new();
        let services = vec![svc("p"), svc("q")];
        let picked = lb
            .select_service(&services, &dummy_request())
            .await
            .expect("test: wr pick");
        assert!(picked.name == "p" || picked.name == "q");
    }

    #[tokio::test]
    async fn weighted_random_non_positive_total_weight_uses_uniform_fallback() {
        let lb = WeightedRandomLoadBalancer::new();
        let services = vec![svc("n1"), svc("n2")];
        lb.update_weights(&[("n1", -1.0), ("n2", -1.0)])
            .await
            .expect("test: wr negative weights");

        for _ in 0..8 {
            let picked = lb
                .select_service(&services, &dummy_request())
                .await
                .expect("test: wr fallback pick");
            assert!(picked.name == "n1" || picked.name == "n2");
        }
    }

    #[tokio::test]
    async fn weighted_random_update_weights_get_stats_record_response() {
        let lb = WeightedRandomLoadBalancer::new();
        lb.update_weights(&[("u", 3.0)])
            .await
            .expect("test: wr update weights");

        let service = svc("u");
        lb.record_response(&service, &ok_response())
            .await
            .expect("test: wr record");

        let stats = lb.get_stats().await.expect("test: wr stats");
        assert_eq!(stats.algorithm, "weighted_random");
        assert!(
            stats
                .service_stats
                .get("u")
                .expect("test: u stats")
                .requests
                >= 1
        );
    }

    #[tokio::test]
    async fn weighted_random_default_matches_new() {
        let a = WeightedRandomLoadBalancer::new();
        let b = WeightedRandomLoadBalancer::default();
        assert_eq!(a.algorithm(), b.algorithm());
    }
}
