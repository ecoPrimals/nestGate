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
                    message: "No services available".to_string(),
                    available_services: Some(0),
                    algorithm: Some("weighted_round_robin".to_string()),
                },
            )));
        }

        let weights = self.weights.read();
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

        for service in services {
            let service_weight = weights.get(&service.name).copied().unwrap_or(1.0);
            let current_weight = current_weights.get_mut(&service.name).ok_or_else(|| {
                crate::error::NestGateError::internal_error(
                    "weighted_balancer",
                    format!("Service {} not found in weight map", service.name),
                )
            })?;
            *current_weight += service_weight;

            if *current_weight > max_weight {
                max_weight = *current_weight;
                selected_service = Some(service.clone());
            }
        }

        if let Some(ref selected) = selected_service {
            // Reduce the selected service's current weight by the total of all weights
            let total_weight: f64 = services
                .iter()
                .map(|s| weights.get(&s.name).copied().unwrap_or(1.0))
                .sum();

            if let Some(current_weight) = current_weights.get_mut(&selected.name) {
                *current_weight -= total_weight;
            }

            // Update stats
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
                    message: "Failed to select service with weighted round-robin".to_string(),
                    available_services: Some(services.len()),
                    algorithm: Some("weighted_round_robin".to_string()),
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
        let mut stats = self.stats.write();
        stats
            .service_stats
            .entry(service.name.clone())
            .or_default()
            .requests += 1;
        Ok(())
    }

    /// Updates  Weights
    async fn update_weights(&self, weights: HashMap<String, f64>) -> Result<()> {
        *self.weights.write() = weights;
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
    rng: Arc<std::sync::Mutex<StdRng>>,
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
            rng: Arc::new(std::sync::Mutex::new(StdRng::from_entropy())),
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
                    message: "No services available".to_string(),
                    available_services: Some(0),
                    algorithm: Some("weighted_random".to_string()),
                },
            )));
        }

        // Implement proper weighted random algorithm
        let weights = self.weights.read();

        // Calculate total weight
        let total_weight: f64 = services
            .iter()
            .map(|service| weights.get(&service.name).copied().unwrap_or(1.0))
            .sum();

        if total_weight <= 0.0 {
            // Fallback to uniform random if no valid weights
            let mut rng = self.rng.lock().map_err(|_| {
                NestGateError::LoadBalancer(Box::new(
                    crate::error::variants::core_errors::LoadBalancerErrorDetails {
                        message: "Random number generator lock poisoned".to_string(),
                        available_services: Some(services.len()),
                        algorithm: Some("weighted_random".to_string()),
                    },
                ))
            })?;
            let index = rng.gen_range(0..services.len());
            return Ok(services[index].clone());
        }

        // Generate random number in [0, total_weight)
        let mut rng = self.rng.lock().map_err(|_| {
            NestGateError::LoadBalancer(Box::new(
                crate::error::variants::core_errors::LoadBalancerErrorDetails {
                    message: "Random number generator lock poisoned".to_string(),
                    available_services: Some(services.len()),
                    algorithm: Some("weighted_random".to_string()),
                },
            ))
        })?;
        let random_weight = rng.gen::<f64>() * total_weight;
        drop(rng);

        // Find the service corresponding to this weight
        let mut cumulative_weight = 0.0;
        for service in services {
            let service_weight = weights.get(&service.name).copied().unwrap_or(1.0);
            cumulative_weight += service_weight;

            if random_weight < cumulative_weight {
                let mut stats = self.stats.write();
                stats.total_requests += 1;
                stats
                    .service_stats
                    .entry(service.name.clone())
                    .or_default()
                    .requests += 1;

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
        let mut stats = self.stats.write();
        stats
            .service_stats
            .entry(service.name.clone())
            .or_default()
            .requests += 1;
        Ok(())
    }

    /// Updates  Weights
    async fn update_weights(&self, weights: HashMap<String, f64>) -> Result<()> {
        *self.weights.write() = weights;
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
