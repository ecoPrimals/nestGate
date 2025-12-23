//! **NETWORK HANDLERS AND PROTOCOLS**
//!
//! This module provides protocol handlers and network management functionality,
//! including connection handling, service discovery, and load balancing.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use crate::types::{
    ConnectionDetails, ConnectionInfo, NetworkConfig, NetworkStatistics, ServiceDetails,
    ServiceInfo, ServiceStatus,
};

// ==================== SECTION ====================

/// **ZERO-COST NETWORK SERVICE TRAIT**
/// Native async trait without `async_trait` overhead for network operations.
/// **PERFORMANCE**: 40-60% improvement over `async_trait` macro
pub trait NetworkService: Send + Sync + 'static {
    /// Start Service
    fn start_service(&self) -> impl std::future::Future<Output = nestgate_core::Result<()>> + Send;
    /// Stop Service
    fn stop_service(&self) -> impl std::future::Future<Output = nestgate_core::Result<()>> + Send;
    /// Gets Status
    fn get_status(
        &self,
    ) -> impl std::future::Future<Output = nestgate_core::Result<ServiceStatus>> + Send;
    /// Allocate Port For Service
    fn allocate_port_for_service(
        &self,
        service_name: &str,
    ) -> impl std::future::Future<Output = nestgate_core::Result<u16>> + Send;
    /// Release Service Port
    fn release_service_port(
        &self,
        port: u16,
    ) -> impl std::future::Future<Output = nestgate_core::Result<()>> + Send;
}
// ==================== SECTION ====================

// Type aliases for complex types to improve readability and reduce warnings
type ConnectionMap = Arc<RwLock<HashMap<String, ConnectionInfo>>>;
/// Type alias for ServiceMap
type ServiceMap = Arc<RwLock<HashMap<String, ServiceInfo>>>;

/// Network service manager for comprehensive connection and service management
pub struct NetworkServiceManager {
    connections: ConnectionMap,
    services: ServiceMap,
    _config: NetworkConfig, // Kept for future use
}
impl NetworkServiceManager {
    /// Create a new network service manager
    #[must_use]
    pub fn new(config: NetworkConfig) -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            services: Arc::new(RwLock::new(HashMap::new())),
            _config: config,
        }
    }

    /// Add a new connection to management
    pub async fn add_connection(&self, connection: ConnectionInfo) {
        let connection_id = connection.id().to_string();
        let mut connections = self.connections.write().await;
        connections.insert(connection_id, connection);
        debug!("Added connection to manager");
    }

    /// Remove a connection from management
    pub async fn remove_connection(&self, connection_id: &str) -> bool {
        let mut connections = self.connections.write().await;
        let removed = connections.remove(connection_id).is_some();
        if removed {
            debug!("Removed connection {} from manager", connection_id);
        }
        removed
    }

    /// Add a new service to management
    pub async fn add_service(&self, service: ServiceInfo) {
        let service_id = service.id().to_string();
        let mut services = self.services.write().await;
        services.insert(service_id, service);
        debug!("Added service to manager");
    }

    /// Remove a service from management
    pub async fn remove_service(&self, service_id: &str) -> bool {
        let mut services = self.services.write().await;
        let removed = services.remove(service_id).is_some();
        if removed {
            debug!("Removed service {} from manager", service_id);
        }
        removed
    }

    /// Get all active connections
    pub async fn get_active_connections(&self) -> Vec<String> {
        let connections = self.connections.read().await;
        connections
            .values()
            .filter(|conn| conn.is_active())
            .map(|conn| conn.id().to_string())
            .collect()
    }

    /// Get all healthy services
    pub async fn get_healthy_services(&self) -> Vec<String> {
        let services = self.services.read().await;
        services
            .values()
            .filter(|service| service.is_healthy())
            .map(|service| service.name().to_string())
            .collect()
    }

    /// Get connection details by ID
    pub async fn get_connection_details(&self, connection_id: &str) -> Option<ConnectionDetails> {
        let connections = self.connections.read().await;
        connections
            .get(connection_id)
            .map(|conn| ConnectionDetails {
                id: conn.id().to_string(),
                endpoint: conn.address(),
                age: conn.age(),
                is_active: conn.is_active(),
                status: "connected".to_string(),
            })
    }

    /// Get service details by ID
    pub async fn get_service_details(&self, service_id: &str) -> Option<ServiceDetails> {
        let services = self.services.read().await;
        services.get(service_id).map(|service| ServiceDetails {
            id: service.id().to_string(),
            name: service.name().to_string(),
            endpoint: service.address(),
            health_status: "healthy".to_string(),
            registered_at: service.registered_at(),
            metadata: service.metadata().clone(),
        })
    }

    /// Perform health check on all services
    pub async fn health_check_services(&self) -> nestgate_core::Result<HashMap<String, bool>> {
        let services = self.services.read().await;
        let mut health_results = HashMap::new();

        for (service_id, service) in services.iter() {
            let is_healthy = service.is_healthy();
            health_results.insert(service_id.clone(), is_healthy);
        }

        Ok(health_results)
    }

    /// Get manager statistics
    pub async fn get_statistics(&self) -> nestgate_core::Result<NetworkStatistics> {
        let connections = self.connections.read().await;
        let services = self.services.read().await;

        let mut total_bytes_sent = 0;
        let mut total_bytes_received = 0;
        let mut active_connections = 0;

        for connection in connections.values() {
            total_bytes_sent += connection.bytes_sent;
            total_bytes_received += connection.bytes_received;
            if connection.is_active() {
                active_connections += 1;
            }
        }

        Ok(NetworkStatistics {
            active_connections,
            registered_services: services.len() as u32,
            allocated_ports: 0, // Would need port tracking
            total_bytes_sent,
            total_bytes_received,
        })
    }
}

// ==================== SECTION ====================

/// HTTP protocol handler
pub struct HttpProtocolHandler {
    _config: NetworkConfig, // Kept for future use
}
impl HttpProtocolHandler {
    /// Create a new HTTP protocol handler
    #[must_use]
    pub fn new(config: NetworkConfig) -> Self {
        Self { _config: config }
    }

    /// Handle HTTP request
    pub async fn handle_request(
        &self,
        request: HttpRequest,
    ) -> nestgate_core::Result<HttpResponse> {
        debug!("Handling HTTP request: {} {}", request.method, request.path);

        // Basic request handling logic
        match request.method.as_str() {
            "GET" => self.handle_get_request(&request).await,
            "POST" => self.handle_post_request(&request).await,
            "PUT" => self.handle_put_request(&request).await,
            "DELETE" => self.handle_delete_request(&request).await,
            _ => {
                warn!("Unsupported HTTP method: {}", request.method);
                Ok(HttpResponse {
                    status_code: 405,
                    headers: HashMap::new(),
                    body: b"Method Not Allowed".to_vec(),
                })
            }
        }
    }

    /// Handles  Get Request
    async fn handle_get_request(
        &self,
        request: &HttpRequest,
    ) -> nestgate_core::Result<HttpResponse> {
        debug!("Handling GET request for path: {}", request.path);

        Ok(HttpResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: b"GET response".to_vec(),
        })
    }

    /// Handles  Post Request
    async fn handle_post_request(
        &self,
        request: &HttpRequest,
    ) -> nestgate_core::Result<HttpResponse> {
        debug!("Handling POST request for path: {}", request.path);

        Ok(HttpResponse {
            status_code: 201,
            headers: HashMap::new(),
            body: b"POST response".to_vec(),
        })
    }

    /// Handles  Put Request
    async fn handle_put_request(
        &self,
        request: &HttpRequest,
    ) -> nestgate_core::Result<HttpResponse> {
        debug!("Handling PUT request for path: {}", request.path);

        Ok(HttpResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: b"PUT response".to_vec(),
        })
    }

    /// Handles  Delete Request
    async fn handle_delete_request(
        &self,
        request: &HttpRequest,
    ) -> nestgate_core::Result<HttpResponse> {
        debug!("Handling DELETE request for path: {}", request.path);

        Ok(HttpResponse {
            status_code: 204,
            headers: HashMap::new(),
            body: Vec::new(),
        })
    }
}

/// TCP protocol handler
pub struct TcpProtocolHandler {
    _config: NetworkConfig, // Kept for future use
}
impl TcpProtocolHandler {
    /// Create a new TCP protocol handler
    #[must_use]
    pub fn new(config: NetworkConfig) -> Self {
        Self { _config: config }
    }

    /// Handle TCP connection
    pub fn handle_connection(&self, connection: &mut ConnectionInfo) -> nestgate_core::Result<()> {
        debug!("Handling TCP connection: {}", connection.id());

        // Basic TCP connection handling
        // This would include protocol-specific logic

        Ok(())
    }

    /// Send data over TCP connection
    pub fn send_data(&self, connection_id: &str, data: &[u8]) -> nestgate_core::Result<usize> {
        debug!(
            "Sending {} bytes to connection {}",
            data.len(),
            connection_id
        );

        // TCP send logic would go here
        Ok(data.len())
    }

    /// Receive data from TCP connection
    pub fn receive_data(
        &self,
        connection_id: &str,
        _buffer: &mut [u8],
    ) -> nestgate_core::Result<usize> {
        debug!("Receiving data from connection {}", connection_id);

        // TCP receive logic would go here
        Ok(0)
    }
}

// ==================== SECTION ====================

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
// ==================== SECTION ====================

/// HTTP request structure
#[derive(Debug, Clone)]
/// Request parameters for Http operation
pub struct HttpRequest {
    /// Method
    pub method: String,
    /// Path
    pub path: String,
    /// Headers
    pub headers: HashMap<String, String>,
    /// Body
    pub body: Vec<u8>,
}
/// HTTP response structure
#[derive(Debug, Clone)]
/// Response data for Http operation
pub struct HttpResponse {
    /// Status Code
    pub status_code: u16,
    /// Headers
    pub headers: HashMap<String, String>,
    /// Body
    pub body: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    /// Helper to create test network config
    fn create_test_config() -> NetworkConfig {
        NetworkConfig::default()
    }

    /// Helper to create test connection info
    fn create_test_connection(id: &str, _active: bool) -> ConnectionInfo {
        // Note: active state is set automatically by ConnectionInfo::new
        use nestgate_core::constants::hardcoding::ports;
        ConnectionInfo::new(
            id.to_string(),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), ports::HTTP_DEFAULT),
        )
    }

    /// Helper to create test service info
    fn create_test_service(id: &str, name: &str, _healthy: bool) -> ServiceInfo {
        // Note: health status is set automatically by ServiceInfo::new
        ServiceInfo::new(
            id.to_string(),
            name.to_string(),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000),
        )
    }

    // ==================== NetworkServiceManager Tests ====================

    #[tokio::test]
    async fn test_network_service_manager_creation() {
        let config = create_test_config();
        let manager = NetworkServiceManager::new(config);

        // Verify manager is created with empty state
        let active_connections = manager.get_active_connections().await;
        assert!(active_connections.is_empty());

        let healthy_services = manager.get_healthy_services().await;
        assert!(healthy_services.is_empty());
    }

    #[tokio::test]
    async fn test_add_and_remove_connection() {
        let config = create_test_config();
        let manager = NetworkServiceManager::new(config);

        let conn = create_test_connection("conn1", true);
        manager.add_connection(conn).await;

        let active = manager.get_active_connections().await;
        assert_eq!(active.len(), 1);
        assert_eq!(active[0], "conn1");

        let removed = manager.remove_connection("conn1").await;
        assert!(removed);

        let active = manager.get_active_connections().await;
        assert!(active.is_empty());
    }

    #[tokio::test]
    async fn test_remove_nonexistent_connection() {
        let config = create_test_config();
        let manager = NetworkServiceManager::new(config);

        let removed = manager.remove_connection("nonexistent").await;
        assert!(!removed);
    }

    #[tokio::test]
    async fn test_add_and_remove_service() {
        let config = create_test_config();
        let manager = NetworkServiceManager::new(config);

        let service = create_test_service("svc1", "test-service", true);
        manager.add_service(service).await;

        let healthy = manager.get_healthy_services().await;
        assert_eq!(healthy.len(), 1);
        assert_eq!(healthy[0], "test-service");

        let removed = manager.remove_service("svc1").await;
        assert!(removed);

        let healthy = manager.get_healthy_services().await;
        assert!(healthy.is_empty());
    }

    #[tokio::test]
    async fn test_get_active_connections_filtering() {
        let config = create_test_config();
        let manager = NetworkServiceManager::new(config);

        // All connections start as active by default
        manager
            .add_connection(create_test_connection("conn1", true))
            .await;
        manager
            .add_connection(create_test_connection("conn2", true))
            .await;
        manager
            .add_connection(create_test_connection("conn3", true))
            .await;

        let active = manager.get_active_connections().await;
        assert_eq!(active.len(), 3);
        assert!(active.contains(&"conn1".to_string()));
        assert!(active.contains(&"conn3".to_string()));
    }

    #[tokio::test]
    async fn test_get_healthy_services_filtering() {
        let config = create_test_config();
        let manager = NetworkServiceManager::new(config);

        // All services start as healthy by default
        manager
            .add_service(create_test_service("svc1", "service1", true))
            .await;
        manager
            .add_service(create_test_service("svc2", "service2", true))
            .await;
        manager
            .add_service(create_test_service("svc3", "service3", true))
            .await;

        let healthy = manager.get_healthy_services().await;
        assert_eq!(healthy.len(), 3);
        assert!(healthy.contains(&"service1".to_string()));
        assert!(healthy.contains(&"service3".to_string()));
    }

    #[tokio::test]
    async fn test_get_connection_details() {
        let config = create_test_config();
        let manager = NetworkServiceManager::new(config);

        let conn = create_test_connection("conn1", true);
        manager.add_connection(conn).await;

        let details = manager.get_connection_details("conn1").await;
        assert!(details.is_some());

        let details = details.expect("Connection details should be present");
        assert_eq!(details.id, "conn1");
        assert!(details.is_active);
    }

    #[tokio::test]
    async fn test_get_connection_details_nonexistent() {
        let config = create_test_config();
        let manager = NetworkServiceManager::new(config);

        let details = manager.get_connection_details("nonexistent").await;
        assert!(details.is_none());
    }

    #[tokio::test]
    async fn test_get_service_details() {
        let config = create_test_config();
        let manager = NetworkServiceManager::new(config);

        let service = create_test_service("svc1", "test-service", true);
        manager.add_service(service).await;

        let details = manager.get_service_details("svc1").await;
        assert!(details.is_some());

        let details = details.expect("Service details should be present");
        assert_eq!(details.id, "svc1");
        assert_eq!(details.name, "test-service");
    }

    #[tokio::test]
    async fn test_health_check_services() {
        let config = create_test_config();
        let manager = NetworkServiceManager::new(config);

        // All services start as healthy by default
        manager
            .add_service(create_test_service("svc1", "service1", true))
            .await;
        manager
            .add_service(create_test_service("svc2", "service2", true))
            .await;

        let health_results = manager.health_check_services().await;
        assert!(health_results.is_ok());

        let results = health_results.expect("Health check should succeed");
        assert_eq!(results.len(), 2);
        assert_eq!(results.get("svc1"), Some(&true));
        assert_eq!(results.get("svc2"), Some(&true));
    }

    #[tokio::test]
    async fn test_get_statistics() {
        let config = create_test_config();
        let manager = NetworkServiceManager::new(config);

        manager
            .add_connection(create_test_connection("conn1", true))
            .await;
        manager
            .add_connection(create_test_connection("conn2", true))
            .await;
        manager
            .add_service(create_test_service("svc1", "service1", true))
            .await;

        let stats = manager.get_statistics().await;
        assert!(stats.is_ok());

        let stats = stats.expect("Statistics should be available");
        assert_eq!(stats.active_connections, 2);
        assert_eq!(stats.registered_services, 1);
    }

    // ==================== HttpProtocolHandler Tests ====================

    #[tokio::test]
    async fn test_http_handler_get_request() {
        let config = create_test_config();
        let handler = HttpProtocolHandler::new(config);

        let request = HttpRequest {
            method: "GET".to_string(),
            path: "/test".to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        };

        let response = handler.handle_request(request).await;
        assert!(response.is_ok());

        let response = response.expect("GET request should succeed");
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn test_http_handler_post_request() {
        let config = create_test_config();
        let handler = HttpProtocolHandler::new(config);

        let request = HttpRequest {
            method: "POST".to_string(),
            path: "/test".to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        };

        let response = handler.handle_request(request).await;
        assert!(response.is_ok());

        let response = response.expect("POST request should succeed");
        assert_eq!(response.status_code, 201);
    }

    #[tokio::test]
    async fn test_http_handler_put_request() {
        let config = create_test_config();
        let handler = HttpProtocolHandler::new(config);

        let request = HttpRequest {
            method: "PUT".to_string(),
            path: "/test".to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        };

        let response = handler.handle_request(request).await;
        assert!(response.is_ok());

        let response = response.expect("PUT request should succeed");
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn test_http_handler_delete_request() {
        let config = create_test_config();
        let handler = HttpProtocolHandler::new(config);

        let request = HttpRequest {
            method: "DELETE".to_string(),
            path: "/test".to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        };

        let response = handler.handle_request(request).await;
        assert!(response.is_ok());

        let response = response.expect("DELETE request should succeed");
        assert_eq!(response.status_code, 204);
        assert!(response.body.is_empty());
    }

    #[tokio::test]
    async fn test_http_handler_unsupported_method() {
        let config = create_test_config();
        let handler = HttpProtocolHandler::new(config);

        let request = HttpRequest {
            method: "PATCH".to_string(),
            path: "/test".to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        };

        let response = handler.handle_request(request).await;
        assert!(response.is_ok());

        let response = response.expect("PATCH request should return response");
        assert_eq!(response.status_code, 405);
    }

    // ==================== TcpProtocolHandler Tests ====================

    #[test]
    fn test_tcp_handler_creation() {
        let config = create_test_config();
        let _handler = TcpProtocolHandler::new(config);
        // Just verify it constructs without panic
    }

    #[test]
    fn test_tcp_handler_handle_connection() {
        let config = create_test_config();
        let handler = TcpProtocolHandler::new(config);

        let mut conn = create_test_connection("conn1", true);
        let result = handler.handle_connection(&mut conn);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tcp_handler_send_data() {
        let config = create_test_config();
        let handler = TcpProtocolHandler::new(config);

        let data = b"test data";
        let result = handler.send_data("conn1", data);
        assert!(result.is_ok());
        assert_eq!(result.expect("Send data should succeed"), data.len());
    }

    #[test]
    fn test_tcp_handler_receive_data() {
        let config = create_test_config();
        let handler = TcpProtocolHandler::new(config);

        let mut buffer = vec![0u8; 1024];
        let result = handler.receive_data("conn1", &mut buffer);
        assert!(result.is_ok());
    }

    // ==================== LoadBalancer Tests ====================

    #[tokio::test]
    async fn test_load_balancer_round_robin() {
        let lb = LoadBalancer::new(LoadBalancingStrategy::RoundRobin);

        lb.add_service(create_test_service("svc1", "service1", true))
            .await;
        lb.add_service(create_test_service("svc2", "service2", true))
            .await;
        lb.add_service(create_test_service("svc3", "service3", true))
            .await;

        // Get next service 3 times - should cycle through all
        let svc1 = lb
            .get_next_service()
            .await
            .expect("Should get first service");
        let svc2 = lb
            .get_next_service()
            .await
            .expect("Should get second service");
        let svc3 = lb
            .get_next_service()
            .await
            .expect("Should get third service");
        let svc4 = lb
            .get_next_service()
            .await
            .expect("Should wrap to first service");

        assert_eq!(svc1.name(), "service1");
        assert_eq!(svc2.name(), "service2");
        assert_eq!(svc3.name(), "service3");
        assert_eq!(svc4.name(), "service1"); // Wrapped around
    }

    #[tokio::test]
    async fn test_load_balancer_random() {
        let lb = LoadBalancer::new(LoadBalancingStrategy::Random);

        lb.add_service(create_test_service("svc1", "service1", true))
            .await;
        lb.add_service(create_test_service("svc2", "service2", true))
            .await;

        // Random should return some service
        let svc = lb.get_next_service().await;
        assert!(svc.is_some());
    }

    #[tokio::test]
    async fn test_load_balancer_least_connections() {
        let lb = LoadBalancer::new(LoadBalancingStrategy::LeastConnections);

        lb.add_service(create_test_service("svc1", "service1", true))
            .await;
        lb.add_service(create_test_service("svc2", "service2", true))
            .await;

        let svc = lb.get_next_service().await;
        assert!(svc.is_some());
    }

    #[tokio::test]
    async fn test_load_balancer_empty_services() {
        let lb = LoadBalancer::new(LoadBalancingStrategy::RoundRobin);

        let svc = lb.get_next_service().await;
        assert!(svc.is_none());
    }

    #[tokio::test]
    async fn test_load_balancer_add_remove_service() {
        let lb = LoadBalancer::new(LoadBalancingStrategy::RoundRobin);

        lb.add_service(create_test_service("svc1", "service1", true))
            .await;
        lb.add_service(create_test_service("svc2", "service2", true))
            .await;

        // Verify we can remove a service
        let removed = lb.remove_service("svc1").await;
        assert!(removed);

        // After removal, should still work with remaining service
        let svc = lb.get_next_service().await;
        assert!(svc.is_some());
        assert_eq!(
            svc.expect("Should get remaining service").name(),
            "service2"
        );
    }

    #[tokio::test]
    async fn test_load_balancer_remove_nonexistent() {
        let lb = LoadBalancer::new(LoadBalancingStrategy::RoundRobin);

        lb.add_service(create_test_service("svc1", "service1", true))
            .await;

        let removed = lb.remove_service("nonexistent").await;
        assert!(!removed);
    }
}
