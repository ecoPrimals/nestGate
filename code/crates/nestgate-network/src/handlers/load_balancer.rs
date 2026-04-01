// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use crate::types::ServiceInfo;

/// Load balancer for distributing requests across services
pub struct LoadBalancer {
    services: Arc<RwLock<Vec<ServiceInfo>>>,
    strategy: LoadBalancingStrategy,
    current_index: Arc<RwLock<usize>>,
}
impl LoadBalancer {
    /// Create a new load balancer
    #[must_use]
    pub fn new(strategy: LoadBalancingStrategy) -> Self {
        Self {
            services: Arc::new(RwLock::new(Vec::new())),
            strategy,
            current_index: Arc::new(RwLock::new(0)),
        }
    }

    /// Add a service to the load balancer
    pub async fn add_service(&self, service: ServiceInfo) {
        let mut services = self.services.write().await;
        services.push(service);
        info!("Added service to load balancer");
    }

    /// Remove a service from the load balancer
    pub async fn remove_service(&self, service_id: &str) -> bool {
        let mut services = self.services.write().await;
        let initial_len = services.len();
        services.retain(|service| service.id() != service_id);
        let removed = services.len() < initial_len;
        if removed {
            info!("Removed service {} from load balancer", service_id);
        }
        removed
    }

    /// Get next service based on load balancing strategy
    pub async fn get_next_service(&self) -> Option<ServiceInfo> {
        let services = self.services.read().await;

        if services.is_empty() {
            return None;
        }

        match self.strategy {
            LoadBalancingStrategy::RoundRobin => {
                let mut index = self.current_index.write().await;
                let service = services[*index].clone();
                *index = (*index + 1) % services.len();
                Some(service)
            }
            LoadBalancingStrategy::Random => {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..services.len());
                Some(services[index].clone())
            }
            LoadBalancingStrategy::LeastConnections => {
                // For now, just return the first service
                // Real implementation would track connection counts
                Some(services[0].clone())
            }
        }
    }
}

/// Load balancing strategies
#[derive(Debug, Clone)]
/// Loadbalancingstrategy
pub enum LoadBalancingStrategy {
    /// Roundrobin
    RoundRobin,
    /// Random
    Random,
    /// Leastconnections
    LeastConnections,
}
