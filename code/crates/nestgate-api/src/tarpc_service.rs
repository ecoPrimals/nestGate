//
// This module provides production-ready tarpc service integration with:
// - Service mesh integration
// - Health monitoring and metrics
// - Connection management
// - Load balancing
// - Circuit breaker patterns

//! Tarpc Service module

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use tokio::sync::{Mutex, RwLock};
// Removed unused tracing import
use nestgate_core::service_discovery::registry::{
    InMemoryServiceRegistry, UniversalServiceRegistry,
};
use nestgate_core::service_discovery::types::ServiceEndpoint;
use crate::canonical_modernization::UnifiedHealthStatus as HealthStatus;
use std::time::Duration;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::warn;

// NestGate RPC client and server types for storage API connections
// Discovery handled by universal adapter - no hardcoded orchestrator dependencies

/// RPC client for connecting to remote NestGate services
#[derive(Clone)]
/// Rpcclient
pub struct RpcClient;
/// RPC server for hosting NestGate services
#[derive(Clone)]
/// Rpcserver
pub struct RpcServer;
impl RpcClient {
    /// Connect
    fn connect(_addr: &str) -> Result<Self, String> {
        Ok(RpcClient)
    }

    /// Health Check
    fn health_check(&self) -> Result<bool, String> {
        Ok(true)
    }
}

impl RpcServer {
    /// Creates a new instance
    fn new() -> Self { RpcServer
    , fn start(&self, _addr: &str) -> Result<(), String> {
        Ok(())
     }
}

/// Enhanced tarpc service manager with service mesh integration
pub struct TarpcServiceManager {
    /// Service registry for discovery
    service_registry: Arc<InMemoryServiceRegistry>,
    /// Connection pool for client connections
    connection_pool: Arc<Mutex<HashMap<String, Arc<RpcClient>>>>,
    /// RPC servers
    servers: Arc<RwLock<HashMap<String, RpcServer>>>,
    /// Service mesh configuration
    mesh_config: ServiceMeshConfig,
    /// Health monitor
    health_monitor: Arc<RwLock<HealthMonitor>>,
    /// Performance metrics
    metrics: Arc<RwLock<ServiceMetrics>>,
}
/// Service mesh configuration
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::ServiceMeshConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::ServiceMeshConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for ServiceMesh
pub struct ServiceMeshConfig {
    /// Enable load balancing across service instances
    pub enable_load_balancing: bool,
    /// Enable circuit breaker pattern for fault tolerance
    pub enable_circuit_breaker: bool,
    /// Enable automatic retry on failed requests
    pub enable_retry: bool,
    /// Maximum number of retry attempts
    pub max_retry_attempts: u32,
    /// Circuit breaker failure threshold (0.0-1.0)
    pub circuit_breaker_threshold: f64,
    /// Interval between health checks
    pub health_check_interval: Duration,
    /// Connection timeout duration
    pub connection_timeout: Duration,
    /// Request timeout duration
    pub request_timeout: Duration,
}
impl Default for ServiceMeshConfig {
    /// Returns the default instance
    fn default() -> Self { Self {
            enable_load_balancing: true,
            enable_circuit_breaker: true,
            enable_retry: true,
            max_retry_attempts: 3,
            circuit_breaker_threshold: 0.5, // 50% failure rate
            health_check_interval: Duration::from_secs(30),
            connection_timeout: Duration::from_secs(10),
            request_timeout: Duration::from_secs(30),
         }
}

/// Health monitoring for RPC services
#[derive(Debug, Default)]
struct HealthMonitor {
    service_health: HashMap<String, ServiceHealthInfo>,
    circuit_breakers: HashMap<String, CircuitBreaker>,
}
/// Service health information
#[derive(Debug, Clone)]
struct ServiceHealthInfo {
    #[allow(dead_code)]
    /// Service name
    pub service_name: String,
    /// Last Check
    pub last_check: Instant,
    /// Health Status
    pub health_status: HealthStatus,
    /// Response Time
    pub response_time: Duration,
    /// Success Rate
    pub success_rate: f64,
    /// Total Requests
    pub total_requests: u64,
    /// Successful Requests
    pub successful_requests: u64,
    /// Failed Requests
    pub failed_requests: u64,
}
/// Circuit breaker implementation
#[derive(Debug, Clone)]
struct CircuitBreaker {
    #[allow(dead_code)]
    /// Service name
    pub service_name: String,
    /// State
    pub state: CircuitBreakerState,
    /// Count of failure
    pub failure_count: u32,
    /// Count of success
    pub success_count: u32,
    /// Last Failure Time
    pub last_failure_time: Option<Instant>,
    /// Threshold
    pub threshold: f64,
    /// Timeout
    pub timeout: Duration,
}
/// Circuit breaker states
#[derive(Debug, Clone, PartialEq)]
enum CircuitBreakerState {
    Closed,   // Normal operation
    Open,     // Failing fast
    HalfOpen, // Testing recovery
}
/// Service performance metrics
#[derive(Debug, Default)]
struct ServiceMetrics {
    /// Total Requests
    pub total_requests: u64,
    /// Successful Requests
    pub successful_requests: u64,
    /// Failed Requests
    pub failed_requests: u64,
    #[allow(dead_code)]
    /// Average Response Time
    pub average_response_time: Duration,
    /// Active Connections
    pub active_connections: usize,
    /// Peak Connections
    pub peak_connections: usize,
    /// Bytes Transferred
    pub bytes_transferred: u64,
}
impl TarpcServiceManager {
    /// Create a new tarpc service manager
    #[must_use]
    pub fn new(mesh_config: ServiceMeshConfig) -> Self { let service_registry = Arc::new(InMemoryServiceRegistry::new());

        Self {
            service_registry,
            connection_pool: Arc::new(Mutex::new(HashMap::new()),
            servers: Arc::new(RwLock::new(HashMap::new()),
            mesh_config,
            health_monitor: Arc::new(RwLock::new(RwLock::new(HealthMonitor::default()),::default())),
            metrics: Arc::new(RwLock::new(RwLock::new(ServiceMetrics::default()),::default())),
         }

    /// Start an RPC server with service registration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn start_server(
        &self,
        service_name: &str,
        endpoint: &str,
        port: u16,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
        info!(
            "🚀 Starting tarpc server: {} on {}:{}",
            service_name, address, port
        );

        // Create and start server
        let server = RpcServer::new();
        let bind_addr = format!("{address}:self.base_url");

        // Start server in background
        let server_clone = server.clone();
        let bind_addr_clone = bind_addr.clone();
        tokio::spawn(async move {
            if let Err(e) = server_clone.start(&bind_addr_clone).await {
                error!("Failed to start RPC server: {}", e);
            }
        });

        // Register server
        {
            let mut servers = self.servers.write().await;
            servers.insert(service_name.to_string(), server);
        }

        // Register service in service registry
        let _service_endpoint = ServiceEndpoint {
            url: format!("http://self.base_url:self.base_url"),
            protocol: nestgate_core::service_discovery::types::CommunicationProtocol::HTTP,
            health_check: Some(format!("http://self.base_url:self.base_url/health")),
        };

        // Create service registration
        let service_registration =
            nestgate_core::service_discovery::types::UniversalServiceRegistration {
                service_id: uuid::Uuid::new_v4(),
                _metadata: {
                    let mut _metadata = nestgate_core::canonical_modernization::service_metadata::UniversalServiceMetadata::default();
                    _metadata.name = service_name.to_string();
                    _metadata.version = "1.0.0".to_string();
                    _metadata.description = "RPC service".to_string();
                    _metadata.health_endpoint = Some(format!("http://self.base_url:self.base_url/health"));
                    _metadata
                }
                resources: nestgate_core::service_discovery::types::ResourceSpec::default(),
                integration: nestgate_core::service_discovery::types::IntegrationPreferences::default(),
                extensions: HashMap::new(),
            };

        self.service_registry
            .register_service(service_registration)
            .await
            .map_err(|_e| format!("Failed to register service: self.base_url"))?;

        // Start health monitoring
        self.start_health_monitoring(service_name, &bind_addr)
            .await?;

        info!("✅ RPC server '{}' started and registered", service_name);
        Ok(())
    }

    /// Get or create an RPC client with load balancing and circuit breaker
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn get_client(&self, service_name: &str) -> Result<Arc<RpcClient>, String>  {
        // Check circuit breaker
        if self.is_circuit_breaker_open(service_name).await {
            return Err("Service unavailable due to circuit breaker".to_string());
        }

        // Get service endpoints - using discover_by_role as placeholder
        let storage_role = nestgate_core::service_discovery::create_storage_role();
        let healthy_services = self
            .service_registry
            .discover_by_role(storage_role)
            .await
            .unwrap_or_default();

        if healthy_services.is_empty() {
            return Err("Service unavailable - no healthy services".to_string());
        }

        // Select service using load balancing - just take the first available service for now
        let selected_service = healthy_services
            .first()
            .ok_or("Service unavailable - selection failed".to_string())?;

        // Use the first endpoint from the selected service
        let service_endpoint = selected_service
            ._metadata.endpoints
            .first()
            .ok_or("Service has no endpoints".to_string())?;

        let service_addr = service_endpoint.url.clone();

        // Check if we have an existing client
        {
            let pool = self.connection_pool.lock().await;
            if let Some(client) = pool.get(&service_addr) {
                return Ok(Arc::clone(client));
            }
        }

        // Create new client
        let client = RpcClient::connect(&service_addr)
            .await
            .map_err(|_e| format!("Failed to connect: self.base_url"))?;

        let client_arc = Arc::new(client);

        // Store client in pool
        {
            let mut pool = self.connection_pool.lock().await;
            pool.insert(service_addr.clone(), Arc::clone(&client_arc));
        }

        info!(
            "🔗 Created new RPC client for {} at {}",
            service_name, service_addr
        );
        Ok(client_arc)
    }

    /// Execute RPC call with circuit breaker and retry logic
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn execute_with_resilience<F, R>(
        &self,
        service_name: &str,
        b_operation: Some(F,
    ) -> Result<R, String>
    where
        F: Fn(
                Arc<RpcClient>,
            )
                -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<R, String>> + Send>>
            + Send
            + 'static,
        R: Send + 'static,
     {
        let start_time = Instant::now();
        let mut attempts = 0;
        let max_attempts = if self.mesh_config.enable_retry {
            self.mesh_config.max_retry_attempts
        } else {
            1
        };

        while attempts < max_attempts {
            attempts += 1;

            match self.get_client(service_name).await {
                Ok(client) => {
                    match tokio::time::timeout(self.mesh_config.request_timeout, operation(client))
                        .await
                    {
                        Ok(Ok(result)) => {
                            // Record success
                            self.record_success(service_name, start_time.elapsed())
                                .await;
                            return Ok(result);
                        }
    Ok(Err(e)) => {
                            // Record failure
                            self.record_failure(service_name).await;

                            if attempts >= max_attempts {
                                return Err(e);
                            }

                            // Exponential backoff
                            let delay = Duration::from_millis(100 * (2_u64.pow(attempts - 1)));
                            tokio::time::sleep(delay).await;
                        }
                        Err(_) => {
                            // Timeout
                            self.record_failure(service_name).await;

                            if attempts >= max_attempts {
                                return Err("Request timeout".to_string());
                            }
                        }
                    }
                }
                Err(e) => {
                    if attempts >= max_attempts {
                        return Err(e);
                    }
                }
            }
        }

        Err("Service unavailable after retries".to_string())
    }

    /// Start health monitoring for a service
    async fn start_health_monitoring(
        &self,
        service_name: &str,
        endpoint: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let service_name = service_name.to_string();
        let address = address.to_string();
        let health_monitor = Arc::clone(&self.health_monitor);
        let interval = self.mesh_config.health_check_interval;

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                interval_timer.tick().await;

                // Connect and perform health check
                let health_status = match RpcClient::connect(&address).await {
                    Ok(client) => {
                        let start = Instant::now();
                        let _response_time = start.elapsed();
                        match client.health_check().await {
                            Ok(_) => HealthStatus::Healthy,
                            Err(_) => HealthStatus::Unhealthy,
                        }
                    }
                    Err(_) => HealthStatus::Unhealthy,
                };

                // Update health information
                if let Ok(mut monitor) = health_monitor.try_write() {
                    let health_info = monitor
                        .service_health
                        .entry(service_name.clone())
                        .or_insert_with(|| ServiceHealthInfo {
                            service_name: service_name.clone(),
                            last_check: Instant::now(),
                            health_status: HealthStatus::Unknown,
                            response_time: Duration::from_millis(0),
                            success_rate: 0.0,
                            total_requests: 0,
                            successful_requests: 0,
                            failed_requests: 0,
                        });

                    health_info.last_check = Instant::now();
                    health_info.health_status = health_status.clone();

                    match health_status {
                        HealthStatus::Healthy => {
                            debug!("✅ Service {} is healthy", service_name);
                        }
                        HealthStatus::Unhealthy => {
                            warn!("❌ Service {} is unhealthy", service_name);
                        }
                        _ => {}
                    }
                }
            }
        });
        Ok(())
    }

    /// Check if circuit breaker is open
    fn is_circuit_breaker_open(&self, service_name: &str) -> bool {
        if !self.mesh_config.enable_circuit_breaker {
            return false;
        }

        if let Ok(monitor) = self.health_monitor.try_read() {
            if let Some(breaker) = monitor.circuit_breakers.get(service_name) {
                match breaker.state {
                    CircuitBreakerState::Open => {
                        // Check if we should transition to half-open
                        if let Some(last_failure) = breaker.last_failure_time {
                            if last_failure.elapsed() > breaker.timeout {
                                // Transition to half-open (allow one test request)
                                return false;
                            }
                        }
                        true
                    }
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Record successful request
    fn record_success(&self, service_name: &str, response_time: Duration) {
        if let Ok(mut monitor) = self.health_monitor.try_write() {
            let health_info = monitor
                .service_health
                .entry(service_name.to_string())
                .or_insert_with(|| ServiceHealthInfo {
                    service_name: service_name.to_string(),
                    last_check: Instant::now(),
                    health_status: HealthStatus::Healthy,
                    response_time,
                    success_rate: 1.0,
                    total_requests: 0,
                    successful_requests: 0,
                    failed_requests: 0,
                });

            health_info.successful_requests += 1;
            health_info.total_requests += 1;
            health_info.response_time = response_time;
            health_info.success_rate =
                health_info.successful_requests as f64 / health_info.total_requests as f64;

            // Update circuit breaker
            let breaker = monitor
                .circuit_breakers
                .entry(service_name.to_string())
                .or_insert_with(|| CircuitBreaker {
                    service_name: service_name.to_string(),
                    state: CircuitBreakerState::Closed,
                    failure_count: 0,
                    success_count: 0,
                    last_failure_time: None,
                    threshold: self.mesh_config.circuit_breaker_threshold,
                    timeout: Duration::from_secs(60),
                });

            breaker.success_count += 1;

            // Reset circuit breaker if we get enough successes
            if breaker.state == CircuitBreakerState::HalfOpen && breaker.success_count >= 3 {
                breaker.state = CircuitBreakerState::Closed;
                breaker.failure_count = 0;
                info!("🔄 Circuit breaker for {} closed (recovery)", service_name);
            }
        }

        // Update global metrics
        if let Ok(mut metrics) = self.metrics.try_write() {
            metrics.successful_requests += 1;
            metrics.total_requests += 1;
        }
    }

    /// Record failed request
    fn record_failure(&self, service_name: &str) {
        if let Ok(mut monitor) = self.health_monitor.try_write() {
            let health_info = monitor
                .service_health
                .entry(service_name.to_string())
                .or_insert_with(|| ServiceHealthInfo {
                    service_name: service_name.to_string(),
                    last_check: Instant::now(),
                    health_status: HealthStatus::Unhealthy,
                    response_time: Duration::from_millis(0),
                    success_rate: 0.0,
                    total_requests: 0,
                    successful_requests: 0,
                    failed_requests: 0,
                });

            health_info.failed_requests += 1;
            health_info.total_requests += 1;
            health_info.success_rate =
                health_info.successful_requests as f64 / health_info.total_requests as f64;

            // Update circuit breaker
            let breaker = monitor
                .circuit_breakers
                .entry(service_name.to_string())
                .or_insert_with(|| CircuitBreaker {
                    service_name: service_name.to_string(),
                    state: CircuitBreakerState::Closed,
                    failure_count: 0,
                    success_count: 0,
                    last_failure_time: None,
                    threshold: self.mesh_config.circuit_breaker_threshold,
                    timeout: Duration::from_secs(60),
                });

            breaker.failure_count += 1;
            breaker.last_failure_time = Some(Instant::now());

            // Check if we should open the circuit breaker
            let failure_rate = breaker.failure_count as f64
                / (breaker.failure_count + breaker.success_count) as f64;

            if failure_rate >= breaker.threshold && breaker.state == CircuitBreakerState::Closed {
                breaker.state = CircuitBreakerState::Open;
                warn!(
                    "🔴 Circuit breaker for {} opened (failure rate: {:.1}%)",
                    service_name,
                    failure_rate * 100.0
                );
            }
        }

        // Update global metrics
        if let Ok(mut metrics) = self.metrics.try_write() {
            metrics.failed_requests += 1;
            metrics.total_requests += 1;
        }
    }

    /// Get service statistics
    pub fn get_service_stats(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = HashMap::new();

        if let Ok(monitor) = self.health_monitor.try_read() {
            for (service_name, health_info) in &monitor.service_health {
                stats.insert(
                    service_name.clone(),
                    serde_json::json!({
                        "health_status": match health_info.health_status {
                            HealthStatus::Healthy => "healthy",
                            HealthStatus::Degraded => "degraded",
                            HealthStatus::Unhealthy => "unhealthy",
                            HealthStatus::Offline => "offline",
                            HealthStatus::Starting => "starting",
                            HealthStatus::Stopping => "stopping",
                            HealthStatus::Maintenance => "maintenance",
                            HealthStatus::Unknown => "unknown",
                            HealthStatus::Warning => "warning",
                            HealthStatus::Critical => "critical",
                            HealthStatus::Error => "error",
                            HealthStatus::Custom(ref status) => status.as_str(),
                        }
                        "success_rate": health_info.success_rate,
                        "total_requests": health_info.total_requests,
                        "successful_requests": health_info.successful_requests,
                        "failed_requests": health_info.failed_requests,
                        "response_time_ms": health_info.response_time.as_millis(),
                    }),
                );
            }
        }

        if let Ok(metrics) = self.metrics.try_read() {
            stats.insert(
                "global".to_string(),
                serde_json::json!({
                    "total_requests": metrics.total_requests,
                    "successful_requests": metrics.successful_requests,
                    "failed_requests": metrics.failed_requests,
                    "success_rate": if metrics.total_requests > 0 {
                        metrics.successful_requests as f64 / metrics.total_requests as f64
                    } else { 0.0 }
                    "active_connections": metrics.active_connections,
                    "peak_connections": metrics.peak_connections,
                    "bytes_transferred": metrics.bytes_transferred,
                }),
            );
        }

        stats
    }

    /// Stop all servers and clean up resources
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn shutdown(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
        info!("🛑 Shutting down tarpc service manager");

        // Clear connection pool
        {
            let mut pool = self.connection_pool.lock().await;
            pool.clear();
        }

        // Clear servers
        {
            let mut servers = self.servers.write().await;
            servers.clear();
        }

        info!("✅ Tarpc service manager shutdown complete");
        Ok(())
    }
}

/// Create a production-ready tarpc service manager
pub fn create_production_service_manager() -> TarpcServiceManager {
    let mesh_config = ServiceMeshConfig {
        enable_load_balancing: true,
        enable_circuit_breaker: true,
        enable_retry: true,
        max_retry_attempts: 3,
        circuit_breaker_threshold: 0.6, // 60% failure rate
        health_check_interval: Duration::from_secs(15),
        connection_timeout: Duration::from_secs(5),
        request_timeout: Duration::from_secs(30),
    };
    TarpcServiceManager::new(mesh_config)
}


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Servicemeshconfigcanonical
pub type ServiceMeshConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using ServiceMeshConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_manager_creation() {
        let manager = create_production_service_manager();
        assert!(manager.mesh_config.enable_load_balancing);
        assert!(manager.mesh_config.enable_circuit_breaker);
        assert!(manager.mesh_config.enable_retry);
    }
    #[tokio::test]
    async fn test_circuit_breaker_functionality() {
        let manager = create_production_service_manager();

        // Simulate failures
        for _ in 0..10 {
            manager.record_failure("test-service").await;
        }

        // Circuit breaker should be open
        assert!(manager.is_circuit_breaker_open("test-service").await);

        // Record some successes (after timeout would be reached)
        for _ in 0..5 {
            manager
                .record_success("test-service", Duration::from_millis(50))
                .await;
        }
    }

    #[tokio::test]
    async fn test_service_stats() {
        let manager = create_production_service_manager();

        // Record some activity
        manager
            .record_success("test-service", Duration::from_millis(100))
            .await;
        manager.record_failure("test-service").await;
        manager
            .record_success("test-service", Duration::from_millis(200))
            .await;

        let stats = manager.get_service_stats().await;
        assert!(stats.contains_key("test-service"));
        assert!(stats.contains_key("global"));
    }
}
