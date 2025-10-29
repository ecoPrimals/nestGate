// **LOAD BALANCER IMPLEMENTATIONS - CANONICAL MODERNIZATION**
//! Implementations functionality and utilities.
// Concrete implementations and wrappers for load balancer functionality.
// Extracted for focused responsibility and clean architecture.

use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

use crate::universal_traits::{ServiceInfo, ServiceRequest, ServiceResponse};
use crate::{NestGateError, Result};

use super::stats::LoadBalancerStats;
use super::traits::{LoadBalancer, LoadBalancerImpl};

/// **CANONICAL IMPLEMENTATION**: Health-aware load balancer wrapper
#[derive(Debug)]
pub struct HealthAwareLoadBalancerImpl {
    health_checker: Arc<RwLock<crate::observability::HealthChecker>>,
    inner: Box<LoadBalancerImpl>,
}
impl HealthAwareLoadBalancerImpl {
    /// Create a new health-aware load balancer
    pub fn new(inner: LoadBalancerImpl) -> Self {
        Self {
            health_checker: Arc::new(RwLock::new(crate::observability::HealthChecker::new())),
            inner: Box::new(inner),
        }
    }

    /// Update service weights for weighted algorithms
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn update_weights(&self, weights: HashMap<String, f64>) -> Result<()>  {
        // Delegate to inner implementation
        match self.inner.as_ref() {
            LoadBalancerImpl::WeightedRoundRobin(lb) => lb.update_weights(weights).await,
            LoadBalancerImpl::WeightedRandom(lb) => lb.update_weights(weights).await,
            _ => Ok(()), // Other algorithms don't use weights
        }
    }
}

impl LoadBalancer for HealthAwareLoadBalancerImpl {
    async fn select_service(
        &self,
        services: &[ServiceInfo],
        request: &ServiceRequest,
    ) -> Result<ServiceInfo> {
        // Filter healthy services first
        let healthy_services: Vec<ServiceInfo> = services
            .iter()
            .filter(|service| {
                self.health_checker
                    .read()
                    .is_healthy(&service.id)
                    .unwrap_or(true) // Default to healthy if unknown
            })
            .cloned()
            .collect();

        if healthy_services.is_empty() {
            return Err(NestGateError::system(
                "No healthy services available",
                "health_aware_load_balancer",
            ));
        }

        // Delegate to inner load balancer with healthy services
        match self.inner.as_ref() {
            LoadBalancerImpl::RoundRobin(lb) => lb.select_service(&healthy_services, request).await,
            LoadBalancerImpl::WeightedRoundRobin(lb) => {
                lb.select_service(&healthy_services, request).await
            }
            LoadBalancerImpl::LeastConnections(lb) => {
                lb.select_service(&healthy_services, request).await
            }
            LoadBalancerImpl::Random(lb) => lb.select_service(&healthy_services, request).await,
            LoadBalancerImpl::WeightedRandom(lb) => {
                lb.select_service(&healthy_services, request).await
            }
            LoadBalancerImpl::HealthAware(lb) => lb.select_service(&healthy_services, request).await,
        }
    }

    async fn record_response(
        &self,
        service: &ServiceInfo,
        response: &ServiceResponse,
    ) -> Result<()> {
        // Update health status based on response
        let is_healthy = response.status_code < 500;
        self.health_checker
            .write()
            .update_health(&service.id, is_healthy);

        // Delegate to inner load balancer
        match self.inner.as_ref() {
            LoadBalancerImpl::RoundRobin(lb) => lb.record_response(service, response).await,
            LoadBalancerImpl::WeightedRoundRobin(lb) => lb.record_response(service, response).await,
            LoadBalancerImpl::LeastConnections(lb) => lb.record_response(service, response).await,
            LoadBalancerImpl::Random(lb) => lb.record_response(service, response).await,
            LoadBalancerImpl::WeightedRandom(lb) => lb.record_response(service, response).await,
            LoadBalancerImpl::HealthAware(lb) => lb.record_response(service, response).await,
        }
    }

    fn get_stats(&self) -> Result<LoadBalancerStats> {
        let mut stats = match self.inner.as_ref() {
            LoadBalancerImpl::RoundRobin(lb) => lb.get_stats().await?,
            LoadBalancerImpl::WeightedRoundRobin(lb) => lb.get_stats().await?,
            LoadBalancerImpl::LeastConnections(lb) => lb.get_stats().await?,
            LoadBalancerImpl::Random(lb) => lb.get_stats().await?,
            LoadBalancerImpl::WeightedRandom(lb) => lb.get_stats().await?,
            LoadBalancerImpl::HealthAware(lb) => lb.get_stats().await?,
        };

        stats.health_aware = true;
        Ok(stats)
    }

    fn algorithm(&self) -> &'static str {
        "health_aware"
    }
}

// ==================== ENUM IMPLEMENTATION ====================

impl LoadBalancer for LoadBalancerImpl {
    async fn select_service(
        &self
        services: &[ServiceInfo],
        request: &ServiceRequest,
    ) -> Result<ServiceInfo> {
        match self {
            LoadBalancerImpl::RoundRobin(lb) => lb.select_service(services, request).await
            LoadBalancerImpl::WeightedRoundRobin(lb) => lb.select_service(services, request).await
            LoadBalancerImpl::LeastConnections(lb) => lb.select_service(services, request).await
            LoadBalancerImpl::Random(lb) => lb.select_service(services, request).await
            LoadBalancerImpl::WeightedRandom(lb) => lb.select_service(services, request).await
            LoadBalancerImpl::HealthAware(lb) => {
                // Health-aware load balancing with proper health filtering
                let healthy_services: Vec<_> = services.iter()
                    .filter(|service| service.is_healthy())
                    .cloned()
                    .collect();
                
                if healthy_services.is_empty() {
                    // Fallback to all services if none are healthy
                    match lb.inner.as_ref() {
                    LoadBalancerImpl::RoundRobin(inner_lb) => {
                        inner_lb.select_service(services, request).await
                    }
    Ok(())
                    LoadBalancerImpl::WeightedRoundRobin(inner_lb) => {
                        inner_lb.select_service(services, request).await
                    }
    Ok(())
                    LoadBalancerImpl::LeastConnections(inner_lb) => {
                        inner_lb.select_service(services, request).await
                    }
    Ok(())
                    LoadBalancerImpl::Random(inner_lb) => {
                        inner_lb.select_service(services, request).await
                    }
    Ok(())
                    LoadBalancerImpl::WeightedRandom(inner_lb) => {
                        inner_lb.select_service(services, request).await
                    }
    Ok(())
                    LoadBalancerImpl::HealthAware(_) => Err(NestGateError::LoadBalancer {,
                        message: "HealthAware load balancer does not support weight updates".to_string(),
                    }),
                }
    Ok(())
            }
    Ok(())
        }
    Ok(())
    }
    Ok(())

    async fn record_response(
        &self
        service: &ServiceInfo,
        response: &ServiceResponse,
    ) -> Result<()> {
        match self {
            LoadBalancerImpl::RoundRobin(lb) => lb.record_response(service, response).await
            LoadBalancerImpl::WeightedRoundRobin(lb) => lb.record_response(service, response).await
            LoadBalancerImpl::LeastConnections(lb) => lb.record_response(service, response).await
            LoadBalancerImpl::Random(lb) => lb.record_response(service, response).await
            LoadBalancerImpl::WeightedRandom(lb) => lb.record_response(service, response).await
            LoadBalancerImpl::HealthAware(lb) => {
                // Record on both the health-aware wrapper and inner implementation
                Box::pin(lb.inner.record_response(service, response)).await
            }
    Ok(())
        }
    Ok(())
    }
    Ok(())

    fn update_weights(
        &self
        weights: HashMap<String, f64>
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        match self {
            LoadBalancerImpl::RoundRobin(_) => Ok(()), // Round robin doesn't use weights
            LoadBalancerImpl::WeightedRoundRobin(lb) => lb.update_weights(weights).await,
            LoadBalancerImpl::LeastConnections(_) => Ok(()), // Least connections doesn't use weights
            LoadBalancerImpl::Random(_) => Ok(()),           // Random doesn't use weights
            LoadBalancerImpl::WeightedRandom(lb) => lb.update_weights(weights).await,
            LoadBalancerImpl::HealthAware(lb) => Box::pin(lb.inner.update_weights(weights)).await,
        }
    Ok(())
    }
    Ok(())

    fn get_stats(&self) -> Result<LoadBalancerStats> {
        match self {
            LoadBalancerImpl::RoundRobin(lb) => Ok(lb.stats.read().clone()),
            LoadBalancerImpl::WeightedRoundRobin(lb) => Ok(lb.stats.read().clone()),
            LoadBalancerImpl::LeastConnections(lb) => Ok(lb.stats.read().clone()),
            LoadBalancerImpl::Random(lb) => Ok(lb.stats.read().clone()),
            LoadBalancerImpl::WeightedRandom(lb) => Ok(lb.stats.read().clone()),
            LoadBalancerImpl::HealthAware(lb) => {
                Ok(LoadBalancerStats {
                    algorithm: "health_aware".to_string(),
                    health_aware: true,
                    ..LoadBalancerStats::default()
                })
            }
    Ok(())
        }
    Ok(())
    }
    Ok(())

    fn algorithm(&self) -> &'static str {
        match self {
            LoadBalancerImpl::RoundRobin(_) => "round_robin",
            LoadBalancerImpl::WeightedRoundRobin(_) => "weighted_round_robin", 
            LoadBalancerImpl::LeastConnections(_) => "least_connections",
            LoadBalancerImpl::Random(_) => "random",
            LoadBalancerImpl::WeightedRandom(_) => "weighted_random",
            LoadBalancerImpl::HealthAware(lb) => lb.algorithm(),
        }
    Ok(())
    }
    Ok(())
}
    Ok(())
