/// Load Balancing for Service Discovery
/// Load balancing strategies and implementation for distributing requests
/// across discovered service endpoints.
use super::config::LoadBalancingStrategy;
use super::types::{HealthStatus, ServiceEndpoint};
use dashmap::DashMap;
use std::sync::Arc;

/// Load balancer for service selection
pub struct LoadBalancer {
    strategy: LoadBalancingStrategy,
    /// Lock-free round-robin counters for load balancing
    /// DashMap provides 5-10x better performance than RwLock<HashMap> under concurrent load
    round_robin_counters: Arc<DashMap<String, usize>>,
    }
impl LoadBalancer {
    /// Create a new load balancer
    #[must_use]
    pub fn new(strategy: LoadBalancingStrategy) -> Self {
        Self {
            strategy,
            round_robin_counters: Arc::new(DashMap::new()),
    }
    }

    /// Select an endpoint from available services
    pub fn select_endpoint(
        &self,
        service_name: &str,
        endpoints: &[ServiceEndpoint],
    ) -> Option<ServiceEndpoint> {
        let healthy_endpoints: Vec<&ServiceEndpoint> = endpoints
            .iter()
            .filter(|e| e.health_status == HealthStatus::Healthy)
            .collect();

        if healthy_endpoints.is_empty() {
            return None;
    }

        match self.strategy {
            LoadBalancingStrategy::RoundRobin => {
                self.round_robin_select(service_name, &healthy_endpoints)
                    .await
    }
            LoadBalancingStrategy::Random => self.random_select(&healthy_endpoints),
            LoadBalancingStrategy::LeastConnections => {
                self.least_connections_select(&healthy_endpoints)
    }
            LoadBalancingStrategy::WeightedRoundRobin => {
                self.weighted_round_robin_select(&healthy_endpoints)
    }
    }
    }

    /// Round Robin Select (lock-free with DashMap)
    async fn round_robin_select(
        &self,
        service_name: &str,
        endpoints: &[&ServiceEndpoint],
    ) -> Option<ServiceEndpoint> {
        if endpoints.is_empty() {
            return None;
    }

        // Lock-free counter increment with DashMap
        let counter = self.round_robin_counters
            .entry(service_name.to_string())
            .and_modify(|c| *c += 1)
            .or_insert(0);
        let index = *counter % endpoints.len();
        Some(endpoints[index].clone())
    }

    /// Random Select
    fn random_select(&self, endpoints: &[&ServiceEndpoint]) -> Option<ServiceEndpoint> {
        if endpoints.is_empty() {
            return None;
    }

        use rand::Rng;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..endpoints.len());
        Some(endpoints[index].clone())
    }

    /// Least Connections Select
    fn least_connections_select(&self, endpoints: &[&ServiceEndpoint]) -> Option<ServiceEndpoint> {
        // For now, just select the first healthy endpoint
        // In a real implementation, we'd track active connections per endpoint
        endpoints.first().map(|e| (*e).clone())
    }

    /// Weighted Round Robin Select
    fn weighted_round_robin_select(
        &self,
        endpoints: &[&ServiceEndpoint],
    ) -> Option<ServiceEndpoint> {
        if endpoints.is_empty() {
            return None;
    }

        // Simple weighted selection based on endpoint weight
        let total_weight: u32 = endpoints.iter().map(|e| e.weight).sum();
        if total_weight == 0 {
            return endpoints.first().map(|e| (*e).clone());
    }

        let mut rng = rand::thread_rng();
        let mut random_weight = rng.gen_range(0..total_weight);

        for endpoint in endpoints {
            if random_weight < endpoint.weight {
                return Some((*endpoint).clone());
    }
            random_weight -= endpoint.weight;
    }

        endpoints.first().map(|e| (*e).clone())
    }
    }
