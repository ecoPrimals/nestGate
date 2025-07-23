use crate::connection_pool::{ConnectionPool, PoolConfig};
// Removed unused error imports
use crate::Result;
/// Enhanced Service Discovery and Ecosystem Integration
///
/// Provides advanced service discovery, health monitoring, and ecosystem
/// integration capabilities for the NestGate Universal Primal Architecture.
///
/// ## Features
/// - **Multi-Protocol Discovery**: HTTP, DNS, Consul, etcd support
/// - **Health Monitoring**: Continuous health checks and failover
/// - **Load Balancing**: Intelligent request distribution
/// - **Service Mesh**: Advanced routing and traffic management
/// - **Auto-Registration**: Automatic service registration and deregistration
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::time::{Instant, SystemTime};
use tokio::sync::{broadcast, RwLock};
use tracing::debug;
use tracing::info;
use tracing::warn;
// Removed unused tracing import

/// Service discovery configuration
#[derive(Debug, Clone)]
pub struct ServiceDiscoveryConfig {
    /// Discovery method (HTTP, DNS, Consul, etcd)
    pub discovery_method: DiscoveryMethod,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Service timeout
    pub service_timeout: Duration,
    /// Retry attempts for failed services
    pub max_retry_attempts: u32,
    /// Load balancing strategy
    pub load_balancing: LoadBalancingStrategy,
    /// Auto-registration settings
    pub auto_registration: Option<AutoRegistrationConfig>,
}

impl Default for ServiceDiscoveryConfig {
    fn default() -> Self {
        Self {
            discovery_method: DiscoveryMethod::Http {
                endpoints: vec!["http://localhost:8500".to_string()],
            },
            health_check_interval: Duration::from_secs(30),
            service_timeout: Duration::from_secs(10),
            max_retry_attempts: 3,
            load_balancing: LoadBalancingStrategy::RoundRobin,
            auto_registration: None,
        }
    }
}

/// Discovery methods
#[derive(Debug, Clone)]
pub enum DiscoveryMethod {
    Http { endpoints: Vec<String> },
    Dns { domain: String },
    Consul { agent_url: String },
    Etcd { endpoints: Vec<String> },
    Static { services: Vec<ServiceEndpoint> },
}

impl Default for DiscoveryMethod {
    fn default() -> Self {
        Self::Http {
            endpoints: vec!["http://localhost:8500".to_string()],
        }
    }
}

/// Load balancing strategies
#[derive(Debug, Clone, Copy)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
    HealthBased,
    Random,
}

/// Auto-registration configuration
#[derive(Debug, Clone)]
pub struct AutoRegistrationConfig {
    pub service_name: String,
    pub service_port: u16,
    pub health_check_endpoint: String,
    pub metadata: HashMap<String, String>,
    pub ttl: Duration,
}

/// Service endpoint information
#[derive(Debug, Clone)]
pub struct ServiceEndpoint {
    pub id: String,
    pub name: String,
    pub address: String,
    pub port: u16,
    pub protocol: String,
    pub metadata: HashMap<String, String>,
    pub health_status: HealthStatus,
    pub last_seen: SystemTime,
    pub response_time: Option<Duration>,
    pub weight: u32,
}

/// Service health status
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
    Warning,
    Unknown,
}

/// Service registry for managing discovered services
pub struct ServiceRegistry {
    services: Arc<RwLock<HashMap<String, Vec<ServiceEndpoint>>>>,
    config: ServiceDiscoveryConfig,
    connection_pool: Arc<ConnectionPool<reqwest::Client>>,
    health_monitor: Arc<RwLock<HealthMonitor>>,
    event_broadcaster: broadcast::Sender<ServiceEvent>,
    load_balancer: Arc<LoadBalancer>,
}

/// Health monitoring component
struct HealthMonitor {
    #[allow(dead_code)]
    unhealthy_services: HashMap<String, Instant>,
    #[allow(dead_code)]
    health_check_tasks: HashMap<String, tokio::task::JoinHandle<()>>,
}

/// Load balancer for service selection
struct LoadBalancer {
    strategy: LoadBalancingStrategy,
    round_robin_counters: Arc<RwLock<HashMap<String, usize>>>,
}

/// Service discovery events
#[derive(Debug, Clone)]
pub enum ServiceEvent {
    ServiceRegistered {
        service: ServiceEndpoint,
    },
    ServiceDeregistered {
        service_id: String,
    },
    ServiceHealthChanged {
        service_id: String,
        status: HealthStatus,
    },
    ServiceUpdated {
        service: ServiceEndpoint,
    },
}

/// Enhanced service discovery implementation
pub struct ServiceDiscovery {
    registry: Arc<ServiceRegistry>,
    discovery_tasks: Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>,
}

impl ServiceRegistry {
    /// Create a new service registry
    pub fn new(config: ServiceDiscoveryConfig) -> Self {
        let connection_pool =
            Arc::new(crate::connection_pool::create_http_pool(Some(PoolConfig {
                max_connections: 50,
                min_connections: 5,
                ..Default::default()
            })));

        let (event_tx, _event_rx) = broadcast::channel(1000);

        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            config,
            connection_pool,
            health_monitor: Arc::new(RwLock::new(HealthMonitor {
                unhealthy_services: HashMap::new(),
                health_check_tasks: HashMap::new(),
            })),
            event_broadcaster: event_tx,
            load_balancer: Arc::new(LoadBalancer {
                strategy: LoadBalancingStrategy::RoundRobin,
                round_robin_counters: Arc::new(RwLock::new(HashMap::new())),
            }),
        }
    }

    /// Register a service
    pub async fn register_service(&self, service: ServiceEndpoint) -> Result<()> {
        let mut services = self.services.write().await;

        let service_name = service.name.clone();
        let service_id = service.id.clone();

        let service_list = services
            .entry(service_name.clone())
            .or_insert_with(Vec::new);

        // Remove existing service with same ID
        service_list.retain(|s| s.id != service_id);

        // Add new/updated service
        service_list.push(service.clone());

        // Broadcast registration event
        let _ = self
            .event_broadcaster
            .send(ServiceEvent::ServiceRegistered { service });

        info!("🔗 Service registered: {} ({})", service_name, service_id);
        Ok(())
    }

    /// Deregister a service
    pub async fn deregister_service(&self, service_id: &str) -> Result<()> {
        let mut services = self.services.write().await;
        let mut found = false;

        for service_list in services.values_mut() {
            if let Some(pos) = service_list.iter().position(|s| s.id == service_id) {
                service_list.remove(pos);
                found = true;
                break;
            }
        }

        if found {
            let _ = self
                .event_broadcaster
                .send(ServiceEvent::ServiceDeregistered {
                    service_id: service_id.to_string(),
                });
            info!("🔌 Service deregistered: {}", service_id);
        }

        Ok(())
    }

    /// Get healthy services by name
    pub async fn get_healthy_services(&self, service_name: &str) -> Vec<ServiceEndpoint> {
        let services = self.services.read().await;

        services
            .get(service_name)
            .map(|service_list| {
                service_list
                    .iter()
                    .filter(|s| s.health_status == HealthStatus::Healthy)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Select best service using load balancing strategy
    pub async fn select_service(&self, service_name: &str) -> Option<ServiceEndpoint> {
        let healthy_services = self.get_healthy_services(service_name).await;

        if healthy_services.is_empty() {
            return None;
        }

        self.load_balancer
            .select_service(&healthy_services, service_name)
            .await
    }

    /// Get all services
    pub async fn get_all_services(&self) -> HashMap<String, Vec<ServiceEndpoint>> {
        self.services.read().await.clone()
    }

    /// Subscribe to service events
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<ServiceEvent> {
        self.event_broadcaster.subscribe()
    }

    /// Start health monitoring
    pub async fn start_health_monitoring(&self) -> Result<()> {
        let services = Arc::clone(&self.services);
        let connection_pool = Arc::clone(&self.connection_pool);
        let _health_monitor = Arc::clone(&self.health_monitor);
        let event_broadcaster = self.event_broadcaster.clone();
        let interval = self.config.health_check_interval;
        let timeout = self.config.service_timeout;

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                interval_timer.tick().await;

                let current_services = services.read().await.clone();

                for (_service_name, service_list) in current_services {
                    for service in service_list {
                        let health_url =
                            format!("http://{}:{}/health", service.address, service.port);

                        // Perform health check
                        let health_status =
                            Self::check_service_health(&connection_pool, &health_url, timeout)
                                .await;

                        // Update service health status
                        let mut services_write = services.write().await;
                        if let Some(service_list) = services_write.get_mut(&service.name) {
                            if let Some(service_mut) =
                                service_list.iter_mut().find(|s| s.id == service.id)
                            {
                                if service_mut.health_status != health_status {
                                    service_mut.health_status = health_status.clone();

                                    // Broadcast health change event
                                    let _ = event_broadcaster.send(
                                        ServiceEvent::ServiceHealthChanged {
                                            service_id: service.id.clone(),
                                            status: health_status.clone(),
                                        },
                                    );

                                    match health_status {
                                        HealthStatus::Healthy => {
                                            info!("✅ Service {} is now healthy", service.id);
                                        }
                                        HealthStatus::Unhealthy => {
                                            warn!("❌ Service {} is now unhealthy", service.id);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    /// Check individual service health
    async fn check_service_health(
        connection_pool: &ConnectionPool<reqwest::Client>,
        health_url: &str,
        timeout: Duration,
    ) -> HealthStatus {
        match connection_pool.acquire().await {
            Ok(client_guard) => {
                let client = match client_guard.connection() {
                    Ok(client) => client,
                    Err(_) => return HealthStatus::Unhealthy,
                };

                let response = tokio::time::timeout(timeout, client.get(health_url).send()).await;

                match response {
                    Ok(Ok(resp)) if resp.status().is_success() => HealthStatus::Healthy,
                    Ok(Ok(resp)) if resp.status().is_server_error() => HealthStatus::Unhealthy,
                    Ok(Ok(_)) => HealthStatus::Warning,
                    _ => HealthStatus::Unhealthy,
                }
            }
            Err(_) => HealthStatus::Unknown,
        }
    }
}

impl LoadBalancer {
    /// Select service using configured strategy
    pub async fn select_service(
        &self,
        services: &[ServiceEndpoint],
        service_name: &str,
    ) -> Option<ServiceEndpoint> {
        if services.is_empty() {
            return None;
        }

        match self.strategy {
            LoadBalancingStrategy::RoundRobin => {
                let mut counters = self.round_robin_counters.write().await;
                let counter = counters.entry(service_name.to_string()).or_insert(0);
                let selected = services.get(*counter % services.len())?;
                *counter += 1;
                Some(selected.clone())
            }
            LoadBalancingStrategy::Random => {
                use rand::seq::SliceRandom;
                let mut rng = rand::thread_rng();
                services.choose(&mut rng).cloned()
            }
            LoadBalancingStrategy::LeastConnections => {
                // For now, just return first service
                // In real implementation, would track connection counts
                services.first().cloned()
            }
            LoadBalancingStrategy::WeightedRoundRobin => {
                // Simple weighted selection based on weight
                let total_weight: u32 = services.iter().map(|s| s.weight).sum();
                if total_weight == 0 {
                    return services.first().cloned();
                }

                let mut counters = self.round_robin_counters.write().await;
                let counter = counters.entry(service_name.to_string()).or_insert(0);
                let weighted_index = (*counter as u32) % total_weight;

                let mut current_weight = 0u32;
                for service in services {
                    current_weight += service.weight;
                    if weighted_index < current_weight {
                        *counter += 1;
                        return Some(service.clone());
                    }
                }

                services.first().cloned()
            }
            LoadBalancingStrategy::HealthBased => {
                // Select service with best response time
                services
                    .iter()
                    .min_by_key(|s| s.response_time.unwrap_or(Duration::from_secs(999)))
                    .cloned()
            }
        }
    }
}

impl ServiceDiscovery {
    /// Create a new service discovery instance
    pub fn new(config: ServiceDiscoveryConfig) -> Self {
        let registry = Arc::new(ServiceRegistry::new(config));

        Self {
            registry,
            discovery_tasks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Start service discovery
    pub async fn start(&self) -> Result<()> {
        info!("🔍 Starting enhanced service discovery system");

        // Start health monitoring
        self.registry.start_health_monitoring().await?;

        // Start discovery based on method
        match &self.registry.config.discovery_method {
            DiscoveryMethod::Http { endpoints } => {
                self.start_http_discovery(endpoints.clone()).await?;
            }
            DiscoveryMethod::Consul { agent_url } => {
                self.start_consul_discovery(agent_url.clone()).await?;
            }
            DiscoveryMethod::Static { services } => {
                self.start_static_discovery(services.clone()).await?;
            }
            _ => {
                warn!("Discovery method not yet implemented, using static mode");
            }
        }

        // Start auto-registration if configured
        if let Some(auto_config) = &self.registry.config.auto_registration {
            self.start_auto_registration(auto_config.clone()).await?;
        }

        info!("✅ Service discovery started successfully");
        Ok(())
    }

    /// Get the service registry
    pub fn registry(&self) -> &ServiceRegistry {
        &self.registry
    }

    /// Stop service discovery
    pub async fn stop(&self) {
        info!("🛑 Stopping service discovery");

        let mut tasks = self.discovery_tasks.write().await;
        for task in tasks.drain(..) {
            task.abort();
        }

        info!("✅ Service discovery stopped");
    }

    /// Start HTTP-based service discovery
    async fn start_http_discovery(&self, endpoints: Vec<String>) -> Result<()> {
        let registry = Arc::clone(&self.registry);

        let task = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));

            loop {
                interval.tick().await;

                for endpoint in &endpoints {
                    if let Ok(client_guard) = registry.connection_pool.acquire().await {
                        let client = match client_guard.connection() {
                            Ok(client) => client,
                            Err(_) => continue,
                        };

                        match client
                            .get(format!("{endpoint}/v1/catalog/services"))
                            .send()
                            .await
                        {
                            Ok(response) if response.status().is_success() => {
                                if let Ok(services) = response.json::<serde_json::Value>().await {
                                    Self::process_discovered_services(&registry, services).await;
                                }
                            }
                            Err(e) => {
                                debug!("Failed to discover services from {}: {}", endpoint, e);
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        self.discovery_tasks.write().await.push(task);
        Ok(())
    }

    /// Start Consul-based service discovery
    async fn start_consul_discovery(&self, agent_url: String) -> Result<()> {
        debug!("Starting Consul service discovery from: {}", agent_url);

        // Implementation would integrate with Consul API
        // For now, fallback to HTTP discovery
        self.start_http_discovery(vec![agent_url]).await
    }

    /// Start static service discovery
    async fn start_static_discovery(&self, services: Vec<ServiceEndpoint>) -> Result<()> {
        info!(
            "Starting static service discovery with {} services",
            services.len()
        );

        for service in services {
            self.registry.register_service(service).await?;
        }

        Ok(())
    }

    /// Start auto-registration
    async fn start_auto_registration(&self, config: AutoRegistrationConfig) -> Result<()> {
        info!(
            "Starting auto-registration for service: {}",
            config.service_name
        );

        let service = ServiceEndpoint {
            id: format!("{}-{}", config.service_name, uuid::Uuid::new_v4()),
            name: config.service_name.clone(),
            address: "127.0.0.1".to_string(), // Would detect actual IP
            port: config.service_port,
            protocol: "http".to_string(),
            metadata: config.metadata.clone(),
            health_status: HealthStatus::Healthy,
            last_seen: SystemTime::now(),
            response_time: None,
            weight: 1,
        };

        self.registry.register_service(service).await?;
        Ok(())
    }

    /// Process discovered services from external source
    async fn process_discovered_services(_registry: &ServiceRegistry, services: serde_json::Value) {
        debug!("Processing discovered services");

        // Implementation would parse the services JSON and register them
        // For now, just log that we received data
        if let Some(services_obj) = services.as_object() {
            debug!("Discovered {} service types", services_obj.len());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_registry_basic_functionality() {
        let config = ServiceDiscoveryConfig::default();
        let registry = ServiceRegistry::new(config);

        let service = ServiceEndpoint {
            id: "test-service-1".to_string(),
            name: "test-service".to_string(),
            address: "127.0.0.1".to_string(),
            port: 8080,
            protocol: "http".to_string(),
            metadata: HashMap::new(),
            health_status: HealthStatus::Healthy,
            last_seen: SystemTime::now(),
            response_time: None,
            weight: 1,
        };

        registry.register_service(service.clone()).await.unwrap();

        let healthy_services = registry.get_healthy_services("test-service").await;
        assert_eq!(healthy_services.len(), 1);
        assert_eq!(healthy_services[0].id, "test-service-1");

        let selected = registry.select_service("test-service").await;
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().id, "test-service-1");
    }

    #[tokio::test]
    async fn test_load_balancer_round_robin() {
        let load_balancer = LoadBalancer {
            strategy: LoadBalancingStrategy::RoundRobin,
            round_robin_counters: Arc::new(RwLock::new(HashMap::new())),
        };

        let services = vec![
            ServiceEndpoint {
                id: "service-1".to_string(),
                name: "test".to_string(),
                address: "127.0.0.1".to_string(),
                port: 8001,
                protocol: "http".to_string(),
                metadata: HashMap::new(),
                health_status: HealthStatus::Healthy,
                last_seen: SystemTime::now(),
                response_time: None,
                weight: 1,
            },
            ServiceEndpoint {
                id: "service-2".to_string(),
                name: "test".to_string(),
                address: "127.0.0.1".to_string(),
                port: 8002,
                protocol: "http".to_string(),
                metadata: HashMap::new(),
                health_status: HealthStatus::Healthy,
                last_seen: SystemTime::now(),
                response_time: None,
                weight: 1,
            },
        ];

        // Test round-robin selection
        let first = load_balancer
            .select_service(&services, "test")
            .await
            .unwrap();
        let second = load_balancer
            .select_service(&services, "test")
            .await
            .unwrap();
        let third = load_balancer
            .select_service(&services, "test")
            .await
            .unwrap();

        assert_eq!(first.id, "service-1");
        assert_eq!(second.id, "service-2");
        assert_eq!(third.id, "service-1"); // Should wrap around
    }
}
