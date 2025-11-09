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

impl LoadBalancer for RoundRobinLoadBalancer {
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
                    algorithm: Some("round_robin".to_string()),
                },
            )));
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
        Err(NestGateError::NotImplemented(Box::new(
            crate::error::variants::core_errors::NotImplementedErrorDetails {
                feature: "update_weights".to_string(),
                message: Some("Round-robin does not support weights".to_string()),
                planned_version: None,
            },
        )))
    }

    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

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

impl LoadBalancer for LeastConnectionsLoadBalancer {
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
                    algorithm: Some("least_connections".to_string()),
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
                    message: "Failed to select service with least connections".to_string(),
                    available_services: Some(services.len()),
                    algorithm: Some("least_connections".to_string()),
                },
            ))
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
        Err(NestGateError::NotImplemented(Box::new(
            crate::error::variants::core_errors::NotImplementedErrorDetails {
                feature: "update_weights".to_string(),
                message: Some("Least connections does not support weights".to_string()),
                planned_version: None,
            },
        )))
    }

    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

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

impl LoadBalancer for RandomLoadBalancer {
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
                    algorithm: Some("random".to_string()),
                },
            )));
        }

        let mut rng = self.rng.lock().map_err(|_| {
            NestGateError::LoadBalancer(Box::new(
                crate::error::variants::core_errors::LoadBalancerErrorDetails {
                    message: "Random number generator lock poisoned".to_string(),
                    available_services: Some(services.len()),
                    algorithm: Some("random".to_string()),
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
        Err(NestGateError::NotImplemented(Box::new(
            crate::error::variants::core_errors::NotImplementedErrorDetails {
                feature: "update_weights".to_string(),
                message: Some("Random load balancer does not support weights".to_string()),
                planned_version: None,
            },
        )))
    }

    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

    fn algorithm(&self) -> &'static str {
        "random"
    }
}
