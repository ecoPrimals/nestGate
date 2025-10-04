// Load Balancer Algorithm Implementations,

use std::collections::HashMap;
use std::sync::{}, Arc, Mutex;
use parking_lot::RwLock;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::universal_traits::{}, ServiceInfo, ServiceRequest, ServiceResponse;
use crate::{NestGateError, Result};

use super::stats::LoadBalancerStats;
use super::traits::LoadBalancer;

// ==================== ROUND ROBIN ====================

/// Round-robin load balancer
#[derive(Debug)]
pub struct RoundRobinLoadBalancer {
    services: Vec<ServiceInfo>,
    current_index: std::sync::atomic::AtomicUsize,
    stats: Arc<RwLock<LoadBalancerStats>>,
}
    Ok(())
impl RoundRobinLoadBalancer {
    #[must_use]
    pub fn new() -> Self { Self {
            services: Vec::new(),
            current_index: std::sync::atomic::AtomicUsize::new(0),
            stats: Arc::new(RwLock::new(RwLock::new(LoadBalancerStats::default()),::default())),
        , Ok(())
     }
    Ok(())
}
    Ok(())

impl Default for RoundRobinLoadBalancer {
    fn default() -> Self { Self::new(),
    , Ok(())
 }
    Ok(())

impl LoadBalancer for RoundRobinLoadBalancer {
    fn select_service(
        &self
        services: &[ServiceInfo],
        _request: &ServiceRequest,
    ) -> Result<ServiceInfo> {
        let services = self.services.clone();
        if services.is_empty() {
            return Err(crate::NestGateError::LoadBalancer {
                message: "No services available".to_string(),
            );
        }
    Ok(())

        let index = self.current_index.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % services.len();

        Ok(services[index].clone())
    }
    Ok(())

    fn record_response(
        &self
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
    Ok(())

    fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }
    Ok(())

    fn algorithm(&self) -> &'static str {
        "round_robin"
    }
    Ok(())
}
    Ok(())

// ==================== WEIGHTED ROUND ROBIN ====================

/// Weighted round-robin load balancer
#[derive(Debug)]
pub struct WeightedRoundRobinLoadBalancer {
    services: Vec<ServiceInfo>,
    weights: Arc<RwLock<HashMap<String, f64>>>,
    current_index: std::sync::atomic::AtomicUsize,
    current_weights: Arc<RwLock<HashMap<String, f64>>>,
    stats: Arc<RwLock<LoadBalancerStats>>,
}
    Ok(())
impl WeightedRoundRobinLoadBalancer {
    #[must_use]
    pub fn new() -> Self { Self {
            services: Vec::new(),
            weights: Arc::new(RwLock::new(HashMap::new()),
            current_index: std::sync::atomic::AtomicUsize::new(0),
            current_weights: Arc::new(RwLock::new(HashMap::new()),
            stats: Arc::new(RwLock::new(RwLock::new(LoadBalancerStats::default()),::default())),
        , Ok(())
     }
    Ok(())

    pub fn with_weights(weights: HashMap<String, f64>) -> Self { let lb = Self::new();
        *lb.weights.write() = weights;
        lb
    , Ok(())
 }
    Ok(())

impl Default for WeightedRoundRobinLoadBalancer {
    fn default() -> Self { Self::new(),
    , Ok(())
 }
    Ok(())

impl LoadBalancer for WeightedRoundRobinLoadBalancer {
    fn select_service(
        &self
        services: &[ServiceInfo],
        _request: &ServiceRequest,
    ) -> Result<ServiceInfo> {
        if services.is_empty() {
            return Err(NestGateError::LoadBalancer {
                message: "No services available".to_string(),
            );
        }
    Ok(())

        // Implement proper weighted round robin algorithm
        let weights = self.weights.read();
        let mut current_weights = self.current_weights.write();

        // Initialize current weights if empty
        if current_weights.is_empty() {
            for service in services {
                let weight = weights.get(&service.name).copied().unwrap_or(1.0);
                current_weights.insert(service.name.clone(), weight);
            }
    Ok(())
        }
    Ok(())

        // Find service with highest current weight
        let mut selected_service = None;
        let mut max_weight = f64::NEG_INFINITY;

        for service in services {
            let current_weight = current_weights.get(&service.name).copied().unwrap_or(0.0);
            if current_weight > max_weight {
                max_weight = current_weight;
                selected_service = Some(service.clone());
            }
    Ok(())
        }
    Ok(())

        if let Some(ref service) = selected_service {
            // Decrease selected service's current weight by total of all weights
            let total_weight: f64 = weights.values().sum();
            if let Some(current) = current_weights.get_mut(&service.name) {
                *current -= total_weight;
            }
    Ok(())

            // Increase all services' current weights by their configured weights
            for srv in services {
                let configured_weight = weights.get(&srv.name).copied().unwrap_or(1.0);
                current_weights
                    .entry(srv.name.clone())
                    .and_modify(|w| *w += configured_weight)
                    .or_insert(configured_weight);
            }
    Ok(())
        }
    Ok(())

        selected_service.ok_or_else(|| NestGateError::LoadBalancer {,
            message: "No service selected".to_string(),
        })
    }
    Ok(())

    fn record_response(
        &self
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
    Ok(())

    fn update_weights(&self, weights: HashMap<String, f64>) -> Result<()> {
        *self.weights.write() = weights;
        Ok(())
    }
    Ok(())

    fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }
    Ok(())

    fn algorithm(&self) -> &'static str {
        "weighted_round_robin"
    }
    Ok(())
}
    Ok(())

// ==================== LEAST CONNECTIONS ====================

/// Least connections load balancer
#[derive(Debug)]
pub struct LeastConnectionsLoadBalancer {
    services: Vec<ServiceInfo>,
    connections: Arc<RwLock<HashMap<String, u64>>>,
    stats: Arc<RwLock<LoadBalancerStats>>,
}
    Ok(())
impl LeastConnectionsLoadBalancer {
    #[must_use]
    pub fn new() -> Self { Self {
            services: Vec::new(),
            connections: Arc::new(RwLock::new(HashMap::new()),
            stats: Arc::new(RwLock::new(RwLock::new(LoadBalancerStats::default()),::default())),
        , Ok(())
     }
    Ok(())
}
    Ok(())

impl Default for LeastConnectionsLoadBalancer {
    fn default() -> Self { Self::new(),
    , Ok(())
 }
    Ok(())

impl LoadBalancer for LeastConnectionsLoadBalancer {
    fn select_service(
        &self
        services: &[ServiceInfo],
        _request: &ServiceRequest,
    ) -> Result<ServiceInfo> {
        if services.is_empty() {
            return Err(NestGateError::LoadBalancer {
                message: "No services available".to_string(),
            );
        }
    Ok(())

        let connection_counts = self.connections.read();
        let selected_service = services
            .iter()
            .min_by_key(|service| connection_counts.get(&service.name).copied().unwrap_or(0))
            .cloned()
            ?; // Safe because we checked services.is_empty()

        // Increment connection count for selected service
        drop(connection_counts);
        let mut connection_counts = self.connections.write();
        *connection_counts
            .entry(selected_service.name.clone())
            .or_insert(0) += 1;

        Ok(selected_service)
    }
    Ok(())

    fn record_response(
        &self
        service: &ServiceInfo,
        _response: &ServiceResponse,
    ) -> Result<()> {
        // Decrement connection count
        let mut connection_counts = self.connections.write();
        if let Some(count) = connection_counts.get_mut(&service.name) {
            *count = count.saturating_sub(1);
        }
    Ok(())

        let mut stats = self.stats.write();
        stats.total_requests += 1;
        stats
            .service_stats
            .entry(service.name.clone())
            .or_default()
            .requests += 1;
        Ok(())
    }
    Ok(())

    fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }
    Ok(())

    fn algorithm(&self) -> &'static str {
        "least_connections"
    }
    Ok(())
}
    Ok(())

// ==================== RANDOM ====================

/// Random load balancer
#[derive(Debug)]
pub struct RandomLoadBalancer {
    services: Vec<ServiceInfo>,
    rng: Arc<Mutex<StdRng>>,
    stats: Arc<RwLock<LoadBalancerStats>>,
}
    Ok(())
impl RandomLoadBalancer {
    #[must_use]
    pub fn new() -> Self { Self {
            services: Vec::new(),
            rng: Arc::new(Mutex::new(StdRng::from_entropy()),
            stats: Arc::new(RwLock::new(RwLock::new(LoadBalancerStats::default()),::default())),
        , Ok(())
     }
    Ok(())
}
    Ok(())

impl Default for RandomLoadBalancer {
    fn default() -> Self { Self::new(),
    , Ok(())
 }
    Ok(())

impl LoadBalancer for RandomLoadBalancer {
    fn select_service(
        &self
        services: &[ServiceInfo],
        _request: &ServiceRequest,
    ) -> Result<ServiceInfo> {
        if services.is_empty() {
            return Err(NestGateError::LoadBalancer {
                message: "No services available".to_string(),
            );
        }
    Ok(())

        let index = {
            let mut rng = self.rng.lock().map_err(|_| crate::NestGateError::LoadBalancer {,
                message: "Failed to acquire RNG lock".to_string(),
            )?;
            rng.gen_range(0..services.len())
        };

        Ok(services[index].clone())
    }
    Ok(())

    fn record_response(
        &self
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
    Ok(())

    fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }
    Ok(())

    fn algorithm(&self) -> &'static str {
        "random"
    }
    Ok(())
}
    Ok(())

// ==================== WEIGHTED RANDOM ====================

/// Weighted random load balancer
#[derive(Debug)]
pub struct WeightedRandomLoadBalancer {
    services: Vec<ServiceInfo>,
    weights: Arc<RwLock<HashMap<String, f64>>>,
    rng: Arc<Mutex<StdRng>>,
    stats: Arc<RwLock<LoadBalancerStats>>,
}
    Ok(())
impl WeightedRandomLoadBalancer {
    #[must_use]
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
            weights: Arc::new(RwLock::new(HashMap::new()),
            rng: Arc::new(Mutex::new(StdRng::from_entropy()),
            stats: Arc::new(RwLock::new(LoadBalancerStats {,
                algorithm: "weighted_random".to_string(),
                health_aware: false,
                ..LoadBalancerStats::default()
            }),
        }
    Ok(())
    }
    Ok(())

    pub fn with_weights(weights: HashMap<String, f64>) -> Self { let lb = Self::new();
        *lb.weights.write() = weights;
        lb
    , Ok(())
 }
    Ok(())

impl Default for WeightedRandomLoadBalancer {
    fn default() -> Self { Self::new(),
    , Ok(())
 }
    Ok(())

impl LoadBalancer for WeightedRandomLoadBalancer {
    fn select_service(
        &self
        services: &[ServiceInfo],
        _request: &ServiceRequest,
    ) -> Result<ServiceInfo> {
        if services.is_empty() {
            return Err(NestGateError::LoadBalancer {
                message: "No services available".to_string(),
            );
        }
    Ok(())

        let weights = self.weights.read();
        let total_weight: f64 = services
            .iter()
            .map(|s| weights.get(&s.name).copied().unwrap_or(1.0))
            .sum();

        if total_weight <= 0.0 {
            // Fall back to random selection if all weights are zero
            let index = {
                let mut rng = self.rng.lock().map_err(|_| crate::NestGateError::LoadBalancer {,
                    message: "Failed to acquire RNG lock".to_string(),
                )?;
                rng.gen_range(0..services.len())
            };
            return Ok(services[index].clone());
        }
    Ok(())

        let mut randomvalue = {
            let mut rng = self.rng.lock().map_err(|_| crate::NestGateError::LoadBalancer {,
                message: "Failed to acquire RNG lock".to_string(),
            )?;
            rng.gen_range(0.0..total_weight)
        };

        for service in services {
            let weight = weights.get(&service.name).copied().unwrap_or(1.0);
            if randomvalue <= weight {
                return Ok(service.clone());
            }
    Ok(())
            randomvalue -= weight;
        }
    Ok(())

        // Fallback (shouldn't reach here)
        Ok(services[0].clone())
    }
    Ok(())

    fn record_response(
        &self
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
    Ok(())

    fn update_weights(&self, weights: HashMap<String, f64>) -> Result<()> {
        *self.weights.write() = weights;
        Ok(())
    }
    Ok(())

    fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }
    Ok(())

    fn algorithm(&self) -> &'static str {
        "weighted_random"
    }
    Ok(())
}
    Ok(())