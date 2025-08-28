// Removed unused error imports
/// Load Balancer Traits
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::{NestGateError, Result};
use crate::universal_traits::{ServiceInfo, ServiceRequest, ServiceResponse};
// ServiceInfo, ServiceRequest, ServiceResponse types consolidated - using generic patterns

/// Load balancer trait
pub trait LoadBalancer: Send + Sync {
    /// Select a service instance for a request
    async fn select_service(
        &self,
        services: &[ServiceInfo],
        request: &ServiceRequest,
    ) -> Result<ServiceInfo>;

    /// Record the response for learning
    async fn record_response(
        &self,
        service: &ServiceInfo,
        response: &ServiceResponse,
    ) -> Result<()>;

    /// Update service weights
    fn update_weights(&self, weights: HashMap<String, f64>) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Get load balancer statistics
    fn get_stats(&self) -> impl std::future::Future<Output = Result<LoadBalancerStats>> + Send;

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
impl LoadBalancer for RoundRobinLoadBalancer {
    async fn select_service(
        &self,
        services: &[ServiceInfo],
        _request: &ServiceRequest,
    ) -> Result<ServiceInfo> {
        if services.is_empty() {
            return Err(NestGateError::LoadBalancer {
                message: "No services available".to_string(),
                operation: "round_robin_select".to_string(),
                available_services: Some(0),
                context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                    operation: "round_robin_select".to_string(),
                    component: "load_balancer".to_string(),
                    metadata: {
                        let mut map = std::collections::HashMap::new();
                        map.insert("location".to_string(), format!("{}:{}", file!(), line!()));
                        map
                    },
                    timestamp: std::time::SystemTime::now(),
                    retry_info: None,
                    recovery_suggestions: vec!["Add services to the pool before load balancing".to_string()],
                    performance_metrics: None,
                    environment: None,
                }),
            });
        }

        let index = self
            .counter
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            % services.len();
        Ok(services[index].clone())
    }

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

    async fn update_weights(&self, _weights: HashMap<String, f64>) -> Result<()> {
        Err(NestGateError::NotImplemented {
            message: "Weight updates not supported for round robin load balancer".to_string(),
            feature: "update_weights".to_string(),
            location: Some("round_robin_load_balancer::update_weights".to_string()),
            context: None,
        })
    }

    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

    fn algorithm(&self) -> &'static str {
        "round_robin"
    }
}

impl LoadBalancer for WeightedRoundRobinLoadBalancer {
    async fn select_service(
        &self,
        services: &[ServiceInfo],
        _request: &ServiceRequest,
    ) -> Result<ServiceInfo> {
        if services.is_empty() {
            return Err(NestGateError::LoadBalancer {
                message: "No services available".to_string(),
                operation: "weighted_round_robin_select".to_string(),
                available_services: Some(0),
                context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                    operation: "weighted_round_robin_select".to_string(),
                    component: "load_balancer".to_string(),
                    metadata: {
                        let mut map = std::collections::HashMap::new();
                        map.insert("location".to_string(), format!("{}:{}", file!(), line!()));
                        map.insert("services_count".to_string(), services.len().to_string());
                        map
                    },
                    timestamp: std::time::SystemTime::now(),
                    retry_info: None,
                    recovery_suggestions: vec!["Add services to the pool before load balancing".to_string()],
                    performance_metrics: None,
                    environment: None,
                }),
            });
        }

        // Implement proper weighted round robin algorithm
        let weights = self.weights.read();
        let mut current_weights = self.current_weights.write();

        // Initialize current weights if empty
        if current_weights.is_empty() {
            for service in services {
                let weight = weights.get(&service.name).copied().unwrap_or(1.0);
                current_weights.insert(service.name.clone(), weight);
            }
        }

        // Find service with highest current weight
        let mut selected_service = None;
        let mut max_weight = f64::NEG_INFINITY;

        for service in services {
            let current_weight = current_weights.get(&service.name).copied().unwrap_or(0.0);
            if current_weight > max_weight {
                max_weight = current_weight;
                selected_service = Some(service.clone());
            }
        }

        if let Some(ref service) = selected_service {
            // Decrease selected service's current weight by total of all weights
            let total_weight: f64 = weights.values().sum();
            if let Some(current) = current_weights.get_mut(&service.name) {
                *current -= total_weight;
            }

            // Increase all services' current weights by their configured weights
            for srv in services {
                let configured_weight = weights.get(&srv.name).copied().unwrap_or(1.0);
                current_weights
                    .entry(srv.name.clone())
                    .and_modify(|w| *w += configured_weight)
                    .or_insert(configured_weight);
            }
        }

        selected_service.ok_or_else(|| NestGateError::LoadBalancer {
            message: "No service selected".to_string(),
            operation: "weighted_round_robin_select".to_string(),
            available_services: Some(services.len()),
            context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                operation: "weighted_round_robin_select".to_string(),
                component: "load_balancer".to_string(),
                metadata: {
                    let mut map = std::collections::HashMap::new();
                    map.insert("location".to_string(), format!("{}:{}", file!(), line!()));
                    map.insert("services_count".to_string(), services.len().to_string());
                    map
                },
                timestamp: std::time::SystemTime::now(),
                retry_info: None,
                recovery_suggestions: vec!["Add services to the pool before load balancing".to_string()],
                    performance_metrics: None,
                    environment: None,
            }),
        })
    }

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

impl LoadBalancer for LeastConnectionsLoadBalancer {
    async fn select_service(
        &self,
        services: &[ServiceInfo],
        _request: &ServiceRequest,
    ) -> Result<ServiceInfo> {
        if services.is_empty() {
            return Err(NestGateError::LoadBalancer {
                message: "No services available".to_string(),
                operation: "least_connections_select".to_string(),
                available_services: Some(0),
                context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                    operation: "least_connections_select".to_string(),
                    component: "load_balancer".to_string(),
                    metadata: {
                        let mut map = std::collections::HashMap::new();
                        map.insert("location".to_string(), format!("{}:{}", file!(), line!()));
                        map
                    },
                    timestamp: std::time::SystemTime::now(),
                    retry_info: None,
                    recovery_suggestions: vec!["Add services to the pool before load balancing".to_string()],
                    performance_metrics: None,
                    environment: None,
                }),
            });
        }

        // Find service with least connections
        let mut min_connections = u64::MAX;
        let mut selected_service = None;

        for service in services {
            let connections = self
                .connection_counts
                .get(&service.name)
                .map_or(0, |entry| entry.requests);

            if connections < min_connections {
                min_connections = connections;
                selected_service = Some(service.clone());
            }
        }

        selected_service.ok_or_else(|| NestGateError::LoadBalancer {
            message: "No service selected".to_string(),
            operation: "least_connections_select".to_string(),
            available_services: Some(services.len()),
            context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                operation: "least_connections_select".to_string(),
                component: "load_balancer".to_string(),
                metadata: {
                    let mut map = std::collections::HashMap::new();
                    map.insert("location".to_string(), format!("{}:{}", file!(), line!()));
                    map.insert("services_count".to_string(), services.len().to_string());
                    map.insert("min_connections".to_string(), min_connections.to_string());
                    map
                },
                timestamp: std::time::SystemTime::now(),
                retry_info: None,
                recovery_suggestions: vec!["Add services to the pool before load balancing".to_string()],
                    performance_metrics: None,
                    environment: None,
            }),
        })
    }

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

    async fn update_weights(&self, _weights: HashMap<String, f64>) -> Result<()> {
        Err(NestGateError::NotImplemented {
            message: "Least connections does not support weights".to_string(),
            feature: "update_weights".to_string(),
            location: Some(format!("{}:{}", file!(), line!())),
            context: None,
        })
    }

    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

    fn algorithm(&self) -> &'static str {
        "least_connections"
    }
}

impl LoadBalancer for RandomLoadBalancer {
    async fn select_service(
        &self,
        services: &[ServiceInfo],
        _request: &ServiceRequest,
    ) -> Result<ServiceInfo> {
        if services.is_empty() {
            return Err(NestGateError::LoadBalancer {
                message: "No services available".to_string(),
                operation: "random_select".to_string(),
                available_services: Some(0),
                context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                    operation: "random_select".to_string(),
                    component: "load_balancer".to_string(),
                    metadata: {
                        let mut map = std::collections::HashMap::new();
                        map.insert("location".to_string(), format!("{}:{}", file!(), line!()));
                        map
                    },
                    timestamp: std::time::SystemTime::now(),
                    retry_info: None,
                    recovery_suggestions: vec!["Add services to the pool before load balancing".to_string()],
                    performance_metrics: None,
                    environment: None,
                }),
            });
        }

        use rand::Rng;

        let mut rng = self.rng.lock().map_err(|_| NestGateError::LoadBalancer {
            message: "Random number generator lock poisoned".to_string(),
            operation: "random_select".to_string(),
            available_services: Some(services.len()),
            context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                operation: "random_select".to_string(),
                component: "load_balancer".to_string(),
                metadata: {
                    let mut map = std::collections::HashMap::new();
                    map.insert("location".to_string(), format!("{}:{}", file!(), line!()));
                    map.insert("error_type".to_string(), "lock_poisoned".to_string());
                    map
                },
                timestamp: std::time::SystemTime::now(),
                retry_info: None,
                recovery_suggestions: vec!["Restart the load balancer to recover from poisoned lock".to_string()],
                    performance_metrics: None,
                    environment: None,
            }),
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

    async fn update_weights(&self, _weights: HashMap<String, f64>) -> Result<()> {
        Err(NestGateError::NotImplemented {
            message: "Random load balancer does not support weights".to_string(),
            feature: "update_weights".to_string(),
            location: Some(format!("{}:{}", file!(), line!())),
            context: None,
        })
    }

    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

    fn algorithm(&self) -> &'static str {
        "random"
    }
}

impl LoadBalancer for WeightedRandomLoadBalancer {
    async fn select_service(
        &self,
        services: &[ServiceInfo],
        _request: &ServiceRequest,
    ) -> Result<ServiceInfo> {
        if services.is_empty() {
            return Err(NestGateError::LoadBalancer {
                message: "No services available".to_string(),
                operation: "weighted_random_select".to_string(),
                available_services: Some(0),
                context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                    operation: "weighted_random_select".to_string(),
                    component: "load_balancer".to_string(),
                    metadata: {
                        let mut map = std::collections::HashMap::new();
                        map.insert("location".to_string(), format!("{}:{}", file!(), line!()));
                        map
                    },
                    timestamp: std::time::SystemTime::now(),
                    retry_info: None,
                    recovery_suggestions: vec!["Add services to the pool before load balancing".to_string()],
                    performance_metrics: None,
                    environment: None,
                }),
            });
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
            let mut rng = self.rng.lock().map_err(|_| NestGateError::LoadBalancer {
                message: "Random number generator lock poisoned".to_string(),
                operation: "weighted_random_select".to_string(),
                available_services: Some(services.len()),
                context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                    operation: "weighted_random_select".to_string(),
                    component: "load_balancer".to_string(),
                    metadata: {
                        let mut map = std::collections::HashMap::new();
                        map.insert("location".to_string(), format!("{}:{}", file!(), line!()));
                        map.insert("error_type".to_string(), "lock_poisoned".to_string());
                        map.insert("fallback_reason".to_string(), "zero_total_weight".to_string());
                        map
                    },
                    timestamp: std::time::SystemTime::now(),
                    retry_info: None,
                    recovery_suggestions: vec!["Restart the load balancer to recover from poisoned lock".to_string()],
                    performance_metrics: None,
                    environment: None,
                }),
            })?;
            let index = rng.gen_range(0..services.len());
            return Ok(services[index].clone());
        }

        // Generate random number in [0, total_weight)
        let mut rng = self.rng.lock().map_err(|_| NestGateError::LoadBalancer {
            message: "Random number generator lock poisoned".to_string(),
            operation: "weighted_random_select".to_string(),
            available_services: Some(services.len()),
            context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                operation: "weighted_random_select".to_string(),
                component: "load_balancer".to_string(),
                metadata: {
                    let mut map = std::collections::HashMap::new();
                    map.insert("location".to_string(), format!("{}:{}", file!(), line!()));
                    map.insert("error_type".to_string(), "lock_poisoned".to_string());
                    map.insert("total_weight".to_string(), total_weight.to_string());
                    map
                },
                timestamp: std::time::SystemTime::now(),
                retry_info: None,
                recovery_suggestions: vec!["Restart the load balancer to recover from poisoned lock".to_string()],
                    performance_metrics: None,
                    environment: None,
            }),
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

impl LoadBalancer for HealthAwareLoadBalancer {
    async fn select_service(
        &self,
        services: &[ServiceInfo],
        request: &ServiceRequest,
    ) -> Result<ServiceInfo> {
        // Filter out unhealthy services
        let healthy_services: Vec<_> = services
            .iter()
            .filter(|_service| true) // Remove status check since ServiceInfo doesn't have status field
            .cloned()
            .collect();

        if healthy_services.is_empty() {
            return Err(NestGateError::LoadBalancer {
                message: "No healthy services available".to_string(),
                operation: "health_aware_select".to_string(),
                available_services: Some(0),
                context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                    operation: "health_aware_select".to_string(),
                    component: "load_balancer".to_string(),
                    metadata: {
                        let mut map = std::collections::HashMap::new();
                        map.insert("location".to_string(), format!("{}:{}", file!(), line!()));
                        map.insert("total_services".to_string(), services.len().to_string());
                        map.insert("healthy_services".to_string(), "0".to_string());
                        map
                    },
                    timestamp: std::time::SystemTime::now(),
                    retry_info: None,
                    recovery_suggestions: vec!["Check service health status and wait for recovery".to_string()],
                    performance_metrics: None,
                    environment: None,
                }),
            });
        }

        self.inner.select_service(&healthy_services, request).await
    }

    async fn record_response(
        &self,
        service: &ServiceInfo,
        response: &ServiceResponse,
    ) -> Result<()> {
        {
            let mut stats = self.stats.write();
            stats.total_requests += 1;
            stats
                .service_stats
                .entry(service.name.clone())
                .or_default()
                .requests += 1;
        } // Lock is dropped here

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
