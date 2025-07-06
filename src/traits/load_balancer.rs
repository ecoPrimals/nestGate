//! Load Balancer Traits

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::errors::{SongbirdError, Result};
use crate::traits::service::{ServiceInfo, ServiceRequest, ServiceResponse};

/// Load balancer trait
#[async_trait]
pub trait LoadBalancer: Send + Sync {
    /// Select a service instance for a request
    async fn select_service(&self, services: &[ServiceInfo], request: &ServiceRequest) -> Result<ServiceInfo>;
    
    /// Record the response for learning
    async fn record_response(&self, service: &ServiceInfo, response: &ServiceResponse) -> Result<()>;
    
    /// Update service weights
    async fn update_weights(&self, weights: HashMap<String, f64>) -> Result<()>;
    
    /// Get load balancer statistics
    async fn get_stats(&self) -> Result<LoadBalancerStats>;
    
    /// Get algorithm name
    fn algorithm(&self) -> &'static str;
}

/// Load balancing algorithms
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    Random,
    WeightedRandom,
    HealthAware,
}

/// Load balancer statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub service_stats: HashMap<String, ServiceStats>,
    pub algorithm: String,
    pub health_aware: bool,
}

/// Statistics for individual services
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ServiceStats {
    pub requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub current_load: f64,
}

impl Default for ServiceStats {
    fn default() -> Self {
        Self {
            requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time: 0.0,
            current_load: 0.0,
        }
    }
}

impl Default for LoadBalancerStats {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time: 0.0,
            service_stats: HashMap::new(),
            algorithm: "round_robin".to_string(),
            health_aware: false,
        }
    }
}

/// Round-robin load balancer
pub struct RoundRobinLoadBalancer {
    counter: Arc<std::sync::atomic::AtomicUsize>,
    stats: Arc<parking_lot::RwLock<LoadBalancerStats>>,
}

impl RoundRobinLoadBalancer {
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
    fn default() -> Self {
        Self::new()
    }
}

/// Weighted round-robin load balancer
pub struct WeightedRoundRobinLoadBalancer {
    weights: Arc<parking_lot::RwLock<HashMap<String, f64>>>,
    current_weights: Arc<parking_lot::RwLock<HashMap<String, f64>>>,
    stats: Arc<parking_lot::RwLock<LoadBalancerStats>>,
}

impl WeightedRoundRobinLoadBalancer {
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
    fn default() -> Self {
        Self::new()
    }
}

/// Least connections load balancer
pub struct LeastConnectionsLoadBalancer {
    connection_counts: Arc<dashmap::DashMap<String, ServiceStats>>,
    stats: Arc<parking_lot::RwLock<LoadBalancerStats>>,
}

impl LeastConnectionsLoadBalancer {
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
    fn default() -> Self {
        Self::new()
    }
}

/// Random load balancer
pub struct RandomLoadBalancer {
    rng: Arc<std::sync::Mutex<StdRng>>,
    stats: Arc<parking_lot::RwLock<LoadBalancerStats>>,
}

impl RandomLoadBalancer {
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
    fn default() -> Self {
        Self::new()
    }
}

/// Weighted random load balancer
pub struct WeightedRandomLoadBalancer {
    weights: Arc<parking_lot::RwLock<HashMap<String, f64>>>,
    rng: Arc<std::sync::Mutex<StdRng>>,
    stats: Arc<parking_lot::RwLock<LoadBalancerStats>>,
}

impl WeightedRandomLoadBalancer {
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
    fn default() -> Self {
        Self::new()
    }
}

/// Health-aware load balancer wrapper
pub struct HealthAwareLoadBalancer {
    inner: Box<dyn LoadBalancer>,
    stats: Arc<parking_lot::RwLock<LoadBalancerStats>>,
}

impl HealthAwareLoadBalancer {
    #[must_use]
    pub fn new(inner: Box<dyn LoadBalancer>) -> Self {
        Self {
            stats: Arc::new(parking_lot::RwLock::new(LoadBalancerStats {
                algorithm: format!("health_aware_{}", inner.algorithm()),
                health_aware: true,
                ..LoadBalancerStats::default()
            })),
            inner,
        }
    }
}

// Trait implementations
#[async_trait]
impl LoadBalancer for RoundRobinLoadBalancer {
    async fn select_service(&self, services: &[ServiceInfo], _request: &ServiceRequest) -> Result<ServiceInfo> {
        if services.is_empty() {
            return Err(SongbirdError::LoadBalancer("No services available".to_string()));
        }
        
        let index = self.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % services.len();
        Ok(services[index].clone())
    }

    async fn record_response(&self, service: &ServiceInfo, _response: &ServiceResponse) -> Result<()> {
        let mut stats = self.stats.write();
        stats.total_requests += 1;
        stats.service_stats.entry(service.id.clone()).or_default().requests += 1;
        Ok(())
    }

    async fn update_weights(&self, _weights: HashMap<String, f64>) -> Result<()> {
        Err(SongbirdError::NotImplemented("Round robin does not support weights".to_string()))
    }

    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

    fn algorithm(&self) -> &'static str {
        "round_robin"
    }
}

#[async_trait]
impl LoadBalancer for WeightedRoundRobinLoadBalancer {
    async fn select_service(&self, services: &[ServiceInfo], _request: &ServiceRequest) -> Result<ServiceInfo> {
        if services.is_empty() {
            return Err(SongbirdError::LoadBalancer("No services available".to_string()));
        }
        
        // Implement proper weighted round robin algorithm
        let weights = self.weights.read();
        let mut current_weights = self.current_weights.write();
        
        // Initialize current weights if empty
        if current_weights.is_empty() {
            for service in services {
                let weight = weights.get(&service.id).copied().unwrap_or(1.0);
                current_weights.insert(service.id.clone(), weight);
            }
        }
        
        // Find service with highest current weight
        let mut selected_service = None;
        let mut max_weight = f64::NEG_INFINITY;
        
        for service in services {
            let current_weight = current_weights.get(&service.id).copied().unwrap_or(0.0);
            if current_weight > max_weight {
                max_weight = current_weight;
                selected_service = Some(service.clone());
            }
        }
        
        if let Some(ref service) = selected_service {
            // Decrease selected service's current weight by total of all weights
            let total_weight: f64 = weights.values().sum();
            if let Some(current) = current_weights.get_mut(&service.id) {
                *current -= total_weight;
            }
            
            // Increase all services' current weights by their configured weights
            for srv in services {
                let configured_weight = weights.get(&srv.id).copied().unwrap_or(1.0);
                current_weights.entry(srv.id.clone())
                    .and_modify(|w| *w += configured_weight)
                    .or_insert(configured_weight);
            }
        }
        
        selected_service.ok_or_else(|| SongbirdError::LoadBalancer("No service selected".to_string()))
    }

    async fn record_response(&self, service: &ServiceInfo, _response: &ServiceResponse) -> Result<()> {
        let mut stats = self.stats.write();
        stats.total_requests += 1;
        stats.service_stats.entry(service.id.clone()).or_default().requests += 1;
        Ok(())
    }

    async fn update_weights(&self, weights: HashMap<String, f64>) -> Result<()> {
        *self.weights.write() = weights;
        Ok(())
    }

    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

    fn algorithm(&self) -> &'static str {
        "weighted_round_robin"
    }
}

#[async_trait]
impl LoadBalancer for LeastConnectionsLoadBalancer {
    async fn select_service(&self, services: &[ServiceInfo], _request: &ServiceRequest) -> Result<ServiceInfo> {
        if services.is_empty() {
            return Err(SongbirdError::LoadBalancer("No services available".to_string()));
        }

        // Find service with least connections
        let mut min_connections = u64::MAX;
        let mut selected_service = None;

        for service in services {
            let connections = self.connection_counts.get(&service.id)
                .map_or(0, |entry| entry.requests);
            
            if connections < min_connections {
                min_connections = connections;
                selected_service = Some(service.clone());
            }
        }

        selected_service
            .ok_or_else(|| SongbirdError::LoadBalancer("No service selected".to_string()))
    }

    async fn record_response(&self, service: &ServiceInfo, _response: &ServiceResponse) -> Result<()> {
        self.connection_counts.entry(service.id.clone()).or_default().requests += 1;
        
        let mut stats = self.stats.write();
        stats.total_requests += 1;
        stats.service_stats.entry(service.id.clone()).or_default().requests += 1;
        Ok(())
    }

    async fn update_weights(&self, _weights: HashMap<String, f64>) -> Result<()> {
        Err(SongbirdError::NotImplemented("Least connections does not support weights".to_string()))
    }

    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

    fn algorithm(&self) -> &'static str {
        "least_connections"
    }
}

#[async_trait]
impl LoadBalancer for RandomLoadBalancer {
    async fn select_service(&self, services: &[ServiceInfo], _request: &ServiceRequest) -> Result<ServiceInfo> {
        if services.is_empty() {
            return Err(SongbirdError::LoadBalancer("No services available".to_string()));
        }
        
        use rand::Rng;
        
        let mut rng = self.rng.lock().unwrap();
        let index = rng.gen_range(0..services.len());
        let selected = services[index].clone();
        drop(rng);
        
        let mut stats = self.stats.write();
        stats.total_requests += 1;
        stats.service_stats.entry(selected.id.clone()).or_default().requests += 1;
        
        Ok(selected)
    }

    async fn record_response(&self, service: &ServiceInfo, _response: &ServiceResponse) -> Result<()> {
        let mut stats = self.stats.write();
        stats.service_stats.entry(service.id.clone()).or_default().requests += 1;
        Ok(())
    }

    async fn update_weights(&self, _weights: HashMap<String, f64>) -> Result<()> {
        Err(SongbirdError::NotImplemented("Random load balancer does not support weights".to_string()))
    }

    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

    fn algorithm(&self) -> &'static str {
        "random"
    }
}

#[async_trait]
impl LoadBalancer for WeightedRandomLoadBalancer {
    async fn select_service(&self, services: &[ServiceInfo], _request: &ServiceRequest) -> Result<ServiceInfo> {
        if services.is_empty() {
            return Err(SongbirdError::LoadBalancer("No services available".to_string()));
        }
        
        // Implement proper weighted random algorithm
        let weights = self.weights.read();
        use rand::Rng;
        
        // Calculate total weight
        let total_weight: f64 = services.iter()
            .map(|service| weights.get(&service.id).copied().unwrap_or(1.0))
            .sum();
        
        if total_weight <= 0.0 {
            // Fallback to uniform random if no valid weights
            let mut rng = self.rng.lock().unwrap();
            let index = rng.gen_range(0..services.len());
            return Ok(services[index].clone());
        }
        
        // Generate random number in [0, total_weight)
        let mut rng = self.rng.lock().unwrap();
        let random_weight = rng.gen::<f64>() * total_weight;
        drop(rng);
        
        // Find the service corresponding to this weight
        let mut cumulative_weight = 0.0;
        for service in services {
            let service_weight = weights.get(&service.id).copied().unwrap_or(1.0);
            cumulative_weight += service_weight;
            
            if random_weight < cumulative_weight {
        let mut stats = self.stats.write();
        stats.total_requests += 1;
                stats.service_stats.entry(service.id.clone()).or_default().requests += 1;
        
                return Ok(service.clone());
            }
        }
        
        // Fallback to last service (shouldn't happen with correct implementation)
        Ok(services[services.len() - 1].clone())
    }

    async fn record_response(&self, service: &ServiceInfo, _response: &ServiceResponse) -> Result<()> {
        let mut stats = self.stats.write();
        stats.service_stats.entry(service.id.clone()).or_default().requests += 1;
        Ok(())
    }

    async fn update_weights(&self, weights: HashMap<String, f64>) -> Result<()> {
        *self.weights.write() = weights;
        Ok(())
    }

    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

    fn algorithm(&self) -> &'static str {
        "weighted_random"
    }
}

#[async_trait]
impl LoadBalancer for HealthAwareLoadBalancer {
    async fn select_service(&self, services: &[ServiceInfo], request: &ServiceRequest) -> Result<ServiceInfo> {
        // Filter out unhealthy services
        let healthy_services: Vec<_> = services.iter()
            .filter(|service| service.status == crate::traits::service::ServiceStatus::Running)
            .cloned()
            .collect();
        
        if healthy_services.is_empty() {
            return Err(SongbirdError::LoadBalancer("No healthy services available".to_string()));
        }
        
        self.inner.select_service(&healthy_services, request).await
    }

    async fn record_response(&self, service: &ServiceInfo, response: &ServiceResponse) -> Result<()> {
        let mut stats = self.stats.write();
        stats.total_requests += 1;
        stats.service_stats.entry(service.id.clone()).or_default().requests += 1;
        drop(stats);
        
        self.inner.record_response(service, response).await
    }

    async fn update_weights(&self, weights: HashMap<String, f64>) -> Result<()> {
        self.inner.update_weights(weights).await
    }

    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

    fn algorithm(&self) -> &'static str {
        self.inner.algorithm()
    }
} 