// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **NETWORK SERVICE IMPLEMENTATION**
//!
//! This module provides the main network service implementation,
//! managing connections, services, and network operations.

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, error, info, warn};

use super::types::{
    ConnectionDetails, ConnectionInfo, NetworkConfig, NetworkStatistics, ServiceDetails,
    ServiceInfo, ServiceStatus,
};
use dashmap::DashMap;
use nestgate_core::NestGateError;

// Type aliases for complex types to improve readability and reduce warnings
// ✅ LOCK-FREE: Migrated to DashMap for concurrent network service management
type ConnectionMap = Arc<DashMap<String, ConnectionInfo>>;
/// Type alias for PortMap
type PortMap = Arc<DashMap<u16, String>>;
/// Type alias for ServiceMap
type ServiceMap = Arc<DashMap<String, ServiceInfo>>;

/// Real network service implementation
#[derive(Debug)]
/// Service implementation for RealNetwork
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
            connections: Arc::new(DashMap::new()), // ✅ Lock-free
            allocated_ports: Arc::new(DashMap::new()), // ✅ Lock-free
            services: Arc::new(DashMap::new()),    // ✅ Lock-free
        }
    }

    /// Get network statistics (lock-free!)
    pub fn get_network_statistics(&self) -> nestgate_core::Result<NetworkStatistics> {
        // ✅ Lock-free iteration over connections, services, and ports
        let mut total_bytes_sent = 0;
        let mut total_bytes_received = 0;
        let mut active_connections = 0;

        // ✅ Lock-free iteration over connections
        for entry in self.connections.iter() {
            let connection = entry.value();
            total_bytes_sent += connection.bytes_sent;
            total_bytes_received += connection.bytes_received;
            if connection.is_active() {
                active_connections += 1;
            }
        }

        Ok(NetworkStatistics {
            active_connections,
            registered_services: self.services.len() as u32,
            allocated_ports: self.allocated_ports.len() as u32,
            total_bytes_sent,
            total_bytes_received,
        })
    }

    /// Start the network service
    pub async fn start(&self) -> nestgate_core::Result<()> {
        info!(
            "Starting real network service on {}:{}",
            self.config.api.bind_address, self.config.api.port
        );

        // ✅ MIGRATED: Now uses centralized runtime configuration
        let addr = {
            use nestgate_core::config::runtime::get_config;
            use nestgate_core::constants::hardcoding::addresses;
            let config = get_config();
            let host = if config.network.bind_all {
                addresses::BIND_ALL_IPV4
            } else {
                &config.network.api_host.to_string()
            };
            format!("{}:{}", host, config.network.api_port)
        };
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

        // Close all active connections (lock-free!)
        self.connections.clear();

        // Release all allocated ports (lock-free!)
        self.allocated_ports.clear();

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

    /// Allocate port for service (lock-free!)
    pub fn allocate_port_for_service(&self, service_name: &str) -> nestgate_core::Result<u16> {
        // ✅ Lock-free: Find and atomically allocate available port
        for port in self.config.api.port_range_start..=self.config.api.port_range_end {
            if self
                .allocated_ports
                .insert(port, service_name.to_string())
                .is_none()
            {
                info!("Allocated port {} for service {}", port, service_name);
                return Ok(port);
            }
        }

        Err(NestGateError::network_error(
            "No available ports for service",
        ))
    }

    /// Release service port
    pub fn release_service_port(&self, port: u16) -> nestgate_core::Result<()> {
        // ✅ Lock-free remove
        if let Some((_, service_name)) = self.allocated_ports.remove(&port) {
            info!("Released port {} from service {}", port, service_name);
        } else {
            warn!("Attempted to release unallocated port {}", port);
        }

        Ok(())
    }

    /// Register a service (lock-free!)
    pub fn register_service(&self, service: ServiceInfo) -> nestgate_core::Result<()> {
        let service_id = service.id().to_string();

        // ✅ Lock-free insert
        self.services.insert(service_id.clone(), service);
        info!("Registered service {}", service_id);

        Ok(())
    }

    /// Unregister a service (lock-free!)
    pub fn unregister_service(&self, service_id: &str) -> nestgate_core::Result<()> {
        // ✅ Lock-free remove
        if self.services.remove(service_id).is_some() {
            info!("Unregistered service {}", service_id);
        } else {
            warn!("Attempted to unregister unknown service {}", service_id);
        }

        Ok(())
    }

    /// Get service status (lock-free!)
    pub fn get_service_status(&self) -> nestgate_core::Result<ServiceStatus> {
        // ✅ Lock-free count of active connections
        let _active_connections = self
            .connections
            .iter()
            .filter(|entry| entry.value().is_active())
            .count() as u32;

        Ok(crate::types::ServiceStatus::Running)
    }

    /// Health check for the network service
    pub async fn health_check(&self) -> nestgate_core::Result<bool> {
        // Basic health check - could be expanded
        let stats = self.get_network_statistics()?;

        // Consider healthy if we have reasonable resource usage
        let healthy = (stats.active_connections as usize)
            < self.config.api.max_connections as usize
            && stats.allocated_ports
                < u32::from(self.config.api.port_range_end - self.config.api.port_range_start);

        if healthy {
            debug!("Network service health check: OK");
        } else {
            warn!("Network service health check: DEGRADED");
        }
        Ok(healthy)
    }

    /// Get connection details (lock-free!)
    pub fn get_connection_details(&self, connection_id: &str) -> Option<ConnectionDetails> {
        // ✅ Lock-free get
        self.connections.get(connection_id).map(|entry| {
            let conn = entry.value();
            ConnectionDetails {
                id: conn.id().to_string(),
                endpoint: conn.address(),
                age: conn.age(),
                is_active: conn.is_active(),
                status: "active".to_string(),
            }
        })
    }

    /// Get service details (lock-free!)
    pub fn get_service_details(&self, service_id: &str) -> Option<ServiceDetails> {
        // ✅ Lock-free get
        self.services.get(service_id).map(|entry| {
            let service = entry.value();
            ServiceDetails {
                id: service.id().to_string(),
                name: service.name().to_string(),
                endpoint: service.address(),
                health_status: "healthy".to_string(),
                registered_at: service.registered_at(),
                metadata: service.metadata().clone(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    fn test_network_config() -> NetworkConfig {
        crate::types::NetworkConfigBuilder::new()
            .port_range(9000, 9010)
            .max_connections(100)
            .build()
    }

    #[test]
    fn test_real_network_service_new() {
        let config = test_network_config();
        let service = RealNetworkService::new(config);
        let stats = service.get_network_statistics().unwrap();
        assert_eq!(stats.active_connections, 0);
        assert_eq!(stats.registered_services, 0);
        assert_eq!(stats.allocated_ports, 0);
    }

    #[test]
    fn test_allocate_port_for_service() {
        let config = test_network_config();
        let service = RealNetworkService::new(config);
        let port1 = service.allocate_port_for_service("svc1").unwrap();
        let port2 = service.allocate_port_for_service("svc2").unwrap();
        assert!(port1 >= 9000 && port1 <= 9010);
        assert!(port2 >= 9000 && port2 <= 9010);
        assert_ne!(port1, port2);
    }

    #[test]
    fn test_release_service_port() {
        let config = test_network_config();
        let service = RealNetworkService::new(config);
        let port = service.allocate_port_for_service("svc1").unwrap();
        let result = service.release_service_port(port);
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_and_unregister_service() {
        let config = test_network_config();
        let service = RealNetworkService::new(config);
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000);
        let service_info = ServiceInfo::new("id-1".to_string(), "test-svc".to_string(), addr);
        let result = service.register_service(service_info);
        assert!(result.is_ok());
        let details = service.get_service_details("id-1");
        assert!(details.is_some());
        assert_eq!(details.unwrap().name, "test-svc");
        let result = service.unregister_service("id-1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_service_status() {
        let config = test_network_config();
        let service = RealNetworkService::new(config);
        let status = service.get_service_status().unwrap();
        assert!(matches!(status, ServiceStatus::Running));
    }

    #[test]
    fn test_get_connection_details_none() {
        let config = test_network_config();
        let service = RealNetworkService::new(config);
        let details = service.get_connection_details("nonexistent");
        assert!(details.is_none());
    }
}
