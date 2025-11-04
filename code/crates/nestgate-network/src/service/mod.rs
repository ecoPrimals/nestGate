//! **NETWORK SERVICE IMPLEMENTATION**
//!
//! This module provides the main network service implementation,
//! managing connections, services, and network operations.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use super::types::{
    ConnectionDetails, ConnectionInfo, NetworkConfig, NetworkStatistics, ServiceDetails,
    ServiceInfo, ServiceStatus,
};
use nestgate_core::NestGateError;

// Type aliases for complex types to improve readability and reduce warnings
type ConnectionMap = Arc<RwLock<HashMap<String, ConnectionInfo>>>;
type PortMap = Arc<RwLock<HashMap<u16, String>>>;
type ServiceMap = Arc<RwLock<HashMap<String, ServiceInfo>>>;

/// Real network service implementation
#[derive(Debug)]
pub struct RealNetworkService {
    /// Configuration
    config: NetworkConfig,
    /// Active connections tracking
    connections: ConnectionMap,
    /// Port allocation tracker
    allocated_ports: PortMap,
    /// Service registry
    services: ServiceMap,
}
impl RealNetworkService {
    /// Create a new real network service
    #[must_use]
    pub fn new(config: NetworkConfig) -> Self {
        Self {
            config,
            connections: Arc::new(RwLock::new(HashMap::new())),
            allocated_ports: Arc::new(RwLock::new(HashMap::new())),
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get network statistics
    pub async fn get_network_statistics(&self) -> nestgate_core::Result<NetworkStatistics> {
        let connections = self.connections.read().await;
        let services = self.services.read().await;
        let allocated_ports = self.allocated_ports.read().await;

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
            allocated_ports: allocated_ports.len() as u32,
            total_bytes_sent,
            total_bytes_received,
        })
    }

    /// Start the network service
    pub async fn start(&self) -> nestgate_core::Result<()> {
        info!(
            "Starting real network service on {}:{}",
            self.config.network.api.bind_address, self.config.network.api.port
        );

        let addr = format!(
            "{}:{}",
            std::env::var("NESTGATE_BIND_ADDRESS").unwrap_or_else(|_| {
                use nestgate_core::constants::hardcoding::addresses;
                addresses::LOCALHOST_NAME.to_string()
            }),
            std::env::var("NESTGATE_API_PORT").unwrap_or_else(|_| {
                use nestgate_core::constants::hardcoding::ports;
                ports::HTTP_DEFAULT.to_string()
            })
        );
        let listener = TcpListener::bind(&addr).await.map_err(|_| {
            NestGateError::network_error(&format!("Failed to bind to endpoint: {addr}"))
        })?;

        info!("Network service listening on {}", addr);

        // Start accepting connections
        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, peer_addr)) => {
                        debug!("Accepted connection from {}", peer_addr);
                        tokio::spawn(Self::handle_connection(stream, peer_addr));
                    }
                    Err(e) => {
                        error!("Failed to accept connection: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    /// Stop the network service
    pub async fn stop(&self) -> nestgate_core::Result<()> {
        info!("Stopping network service");

        // Close all active connections
        let mut connections = self.connections.write().await;
        connections.clear();

        // Release all allocated ports
        let mut allocated_ports = self.allocated_ports.write().await;
        allocated_ports.clear();

        info!("Network service stopped");
        Ok(())
    }

    /// Handle individual connection
    async fn handle_connection(_stream: TcpStream, peer_addr: SocketAddr) {
        debug!("Handling connection from {}", peer_addr);

        // Connection handling logic would go here
        // For now, we'll just log and close

        debug!("Connection from {} handled", peer_addr);
    }

    /// Allocate port for service
    pub async fn allocate_port_for_service(
        &self,
        service_name: &str,
    ) -> nestgate_core::Result<u16> {
        let mut allocated_ports = self.allocated_ports.write().await;

        // Find an available port in the configured range
        for port in self.config.extensions.port_range_start..=self.config.extensions.port_range_end
        {
            if let std::collections::hash_map::Entry::Vacant(e) = allocated_ports.entry(port) {
                e.insert(service_name.to_string());
                info!("Allocated port {} for service {}", port, service_name);
                return Ok(port);
            }
        }

        Err(NestGateError::network_error(
            "No available ports for service",
        ))
    }

    /// Release service port
    pub async fn release_service_port(&self, port: u16) -> nestgate_core::Result<()> {
        let mut allocated_ports = self.allocated_ports.write().await;

        if let Some(service_name) = allocated_ports.remove(&port) {
            info!("Released port {} from service {}", port, service_name);
        } else {
            warn!("Attempted to release unallocated port {}", port);
        }

        Ok(())
    }

    /// Register a service
    pub async fn register_service(&self, service: ServiceInfo) -> nestgate_core::Result<()> {
        let service_id = service.id().to_string();
        let mut services = self.services.write().await;

        services.insert(service_id.clone(), service);
        info!("Registered service {}", service_id);

        Ok(())
    }

    /// Unregister a service
    pub async fn unregister_service(&self, service_id: &str) -> nestgate_core::Result<()> {
        let mut services = self.services.write().await;

        if services.remove(service_id).is_some() {
            info!("Unregistered service {}", service_id);
        } else {
            warn!("Attempted to unregister unknown service {}", service_id);
        }

        Ok(())
    }

    /// Get service status
    pub async fn get_service_status(&self) -> nestgate_core::Result<ServiceStatus> {
        let connections = self.connections.read().await;
        let _services = self.services.read().await;

        let _active_connections =
            connections.values().filter(|conn| conn.is_active()).count() as u32;

        Ok(crate::types::ServiceStatus::Running)
    }

    /// Health check for the network service
    pub async fn health_check(&self) -> nestgate_core::Result<bool> {
        // Basic health check - could be expanded
        let stats = self.get_network_statistics().await?;

        // Consider healthy if we have reasonable resource usage
        let healthy = (stats.active_connections as usize)
            < self.config.network.api.max_connections as usize
            && stats.allocated_ports
                < u32::from(
                    self.config.extensions.port_range_end - self.config.extensions.port_range_start,
                );

        if healthy {
            debug!("Network service health check: OK");
        } else {
            warn!("Network service health check: DEGRADED");
        }
        Ok(healthy)
    }

    /// Get connection details
    pub async fn get_connection_details(&self, connection_id: &str) -> Option<ConnectionDetails> {
        let connections = self.connections.read().await;
        connections
            .get(connection_id)
            .map(|conn| ConnectionDetails {
                id: conn.id().to_string(),
                endpoint: conn.address(),
                age: conn.age(),
                is_active: conn.is_active(),
                status: "active".to_string(),
            })
    }

    /// Get service details
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
}
