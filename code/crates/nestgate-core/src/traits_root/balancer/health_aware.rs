//! # Health-Aware Load Balancing
//! Health Aware functionality and utilities.
// Health-aware load balancer wrapper that filters unhealthy services

use std::collections::HashMap;
use std::sync::Arc;

use crate::{NestGateError, Result};
use crate::universal_traits::{ServiceInfo, ServiceRequest, ServiceResponse};
use super::traits::{LoadBalancer, LoadBalancerStats};

/// Health-aware load balancer wrapper
pub struct HealthAwareLoadBalancer {
    inner: Box<dyn LoadBalancer>,
    stats: Arc<parking_lot::RwLock<LoadBalancerStats>>,
}
impl HealthAwareLoadBalancer ", 
    #[must_use]
    pub const fn new(inner: Box<dyn LoadBalancer>) -> Self {
        Self {
            stats: Arc::new(parking_lot::RwLock::new(LoadBalancerStats {
                algorithm: format!("health_aware_{inner.algorithm()")),
                health_aware: true,
                ..LoadBalancerStats::default()
            }),
            inner,
        }
    }

    /// Check if a service is healthy
    /// Note: This is a simplified implementation. In a real system,
    /// you would integrate with your health check system.
    fn is_service_healthy(&self, _service: &ServiceInfo) -> bool {
        // For now, assume all services are healthy
        // In a real implementation, this would check:
        // - Recent health check results
        // - Circuit breaker status
        // - Response time thresholds
        // - Error rates
        true
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
            .filter(|service| self.is_service_healthy(service))
            .cloned()
            .collect();

        if healthy_services.is_empty() {
            return Err(NestGateError::LoadBalancer {
                message: "No healthy services available".to_string(),
                available_services: Some(0),
            );
        }

        // Delegate to the inner load balancer with only healthy services
        let selected = self.inner.select_service(&healthy_services, request).await?;

        // Update our own stats
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

    async fn record_response(
        &self,
        service: &ServiceInfo,
        response: &ServiceResponse,
    ) -> Result<()> {
        // Update our stats
        {
            let mut stats = self.stats.write();
            stats.total_requests += 1;
            stats
                .service_stats
                .entry(service.name.clone())
                .or_default()
                .requests += 1;
        }

        // Delegate to inner load balancer
        self.inner.record_response(service, response).await
    }

    async fn update_weights(&self, weights: HashMap<String, f64>) -> Result<()> {
        self.inner.update_weights(weights).await
    }

    async fn get_stats(&self) -> Result<LoadBalancerStats> {
        Ok(self.stats.read().clone())
    }

    fn algorithm(&self) -> &'static str {
        // Return the inner algorithm name since we're a wrapper
        self.inner.algorithm()
    }
} 