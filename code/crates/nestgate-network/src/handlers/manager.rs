// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;

use crate::types::{
    ConnectionDetails, ConnectionInfo, NetworkConfig, NetworkStatistics, ServiceDetails,
    ServiceInfo,
};

// Type aliases for complex types to improve readability and reduce warnings
type ConnectionMap = Arc<RwLock<HashMap<String, ConnectionInfo>>>;
/// Type alias for `ServiceMap`
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
