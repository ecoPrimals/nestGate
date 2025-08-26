use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use uuid::Uuid;

/// Zero-cost service instance representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostServiceInstance {
    /// Service instance ID
    pub id: Uuid,
    /// Service name
    pub name: String,
    /// Service endpoint URL
    pub endpoint: String,
    /// Service port
    pub port: u16,
    /// Service metadata
    pub metadata: HashMap<String, String>,
    /// Service tags
    pub tags: Vec<String>,
    /// Registration timestamp
    pub registered_at: SystemTime,
    /// Last seen timestamp
    pub last_seen: SystemTime,
    /// Service health status
    pub health_status: String,
    /// Service version
    pub version: Option<String>,
}

impl ZeroCostServiceInstance {
    /// Create new service instance
    pub fn new(name: String, endpoint: String, port: u16) -> Self {
        let now = SystemTime::now();
        Self {
            id: Uuid::new_v4(),
            name,
            endpoint,
            port,
            metadata: HashMap::new(),
            tags: Vec::new(),
            registered_at: now,
            last_seen: now,
            health_status: "unknown".to_string(),
            version: None,
        }
    }

    /// Update last seen timestamp
    pub fn update_last_seen(&mut self) {
        self.last_seen = SystemTime::now();
    }

    /// Set health status
    pub fn set_health_status(&mut self, status: String) {
        self.health_status = status;
        self.update_last_seen();
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Add tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Check if service instance is stale
    pub fn is_stale(&self, max_age: std::time::Duration) -> bool {
        self.last_seen.elapsed().unwrap_or_default() > max_age
    }

    /// Get service address
    pub fn address(&self) -> String {
        format!("{}:{}", self.endpoint, self.port)
    }
}

/// Service registry for managing service instances
pub struct ServiceRegistry {
    /// Registered services
    services: Arc<RwLock<HashMap<String, ZeroCostServiceInstance>>>,
    /// Registry configuration
    config: ServiceRegistryConfig,
}

/// Service registry configuration
#[derive(Debug, Clone)]
pub struct ServiceRegistryConfig {
    /// Maximum service age before considered stale
    pub max_service_age: std::time::Duration,
    /// Cleanup interval for stale services
    pub cleanup_interval: std::time::Duration,
    /// Maximum number of services
    pub max_services: usize,
}

impl Default for ServiceRegistryConfig {
    fn default() -> Self {
        Self {
            max_service_age: std::time::Duration::from_secs(300), // 5 minutes
            cleanup_interval: std::time::Duration::from_secs(60), // 1 minute
            max_services: 1000,
        }
    }
}

impl ServiceRegistry {
    /// Create new service registry
    pub fn new(config: ServiceRegistryConfig) -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Register a service instance
    pub fn register_service(&self, service: ZeroCostServiceInstance) -> Result<()> {
        let mut services = self.services.write().map_err(|_| {
            NestGateError::simple(
                "Failed to acquire write lock on service registry".to_string(),
                "service_registry_register".to_string(),
            )
        })?;

        // Check if we're at capacity
        if services.len() >= self.config.max_services {
            return Err(NestGateError::invalid_input(
                "registry_capacity".to_string(),
                format!("Registry at capacity: {}", self.config.max_services),
            ));
        }

        let service_key = format!("{}:{}", service.name, service.id);
        services.insert(service_key, service);

        Ok(())
    }

    /// Unregister a service instance
    pub fn unregister_service(&self, service_name: &str, service_id: Uuid) -> Result<bool> {
        let mut services = self.services.write().map_err(|_| {
            NestGateError::simple(
                "Failed to acquire write lock on service registry".to_string(),
                "service_registry_unregister".to_string(),
            )
        })?;

        let service_key = format!("{}:{}", service_name, service_id);
        Ok(services.remove(&service_key).is_some())
    }

    /// Find services by name
    pub fn find_services(&self, service_name: &str) -> Result<Vec<ZeroCostServiceInstance>> {
        let services = self.services.read().map_err(|_| {
            NestGateError::simple(
                "Failed to acquire read lock on service registry".to_string(),
                "service_registry_find_services".to_string(),
            )
        })?;

        let matching_services: Vec<ZeroCostServiceInstance> = services
            .values()
            .filter(|service| service.name == service_name)
            .cloned()
            .collect();

        Ok(matching_services)
    }

    /// Find service by ID
    pub fn find_service_by_id(&self, service_id: Uuid) -> Result<Option<ZeroCostServiceInstance>> {
        let services = self.services.read().map_err(|_| {
            NestGateError::simple(
                "Failed to acquire read lock on service registry".to_string(),
                "service_registry_find_by_id".to_string(),
            )
        })?;

        let service = services
            .values()
            .find(|service| service.id == service_id)
            .cloned();

        Ok(service)
    }

    /// Find services by tag (using metadata instead of tags field)
    pub fn find_services_by_tag(&self, tag: &str) -> Result<Vec<ZeroCostServiceInstance>> {
        let services = self.services.read().map_err(|_| {
            NestGateError::simple(
                "Failed to acquire read lock on service registry".to_string(),
                "service_registry_find_by_tag".to_string(),
            )
        })?;

        let matching_services: Vec<ZeroCostServiceInstance> = services
            .values()
            .filter(|service| service.metadata.values().any(|v| v.contains(tag)))
            .cloned()
            .collect();

        Ok(matching_services)
    }

    /// Get all registered services
    pub fn get_all_services(&self) -> Result<Vec<ZeroCostServiceInstance>> {
        let services = self.services.read().map_err(|_| {
            NestGateError::simple(
                "Failed to acquire read lock on service registry".to_string(),
                "service_registry_get_all".to_string(),
            )
        })?;

        Ok(services.values().cloned().collect())
    }

    /// Update service health status
    pub fn update_service_health(&self, service_id: Uuid, health_status: String) -> Result<bool> {
        let mut services = self.services.write().map_err(|_| {
            NestGateError::simple(
                "Failed to acquire write lock on service registry".to_string(),
                "service_registry_update_health".to_string(),
            )
        })?;

        for service in services.values_mut() {
            if service.id == service_id {
                service.health_status = health_status;
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Cleanup stale services
    pub fn cleanup_stale_services(&self) -> Result<usize> {
        let mut services = self.services.write().map_err(|_| {
            NestGateError::simple(
                "Failed to acquire write lock on service registry".to_string(),
                "service_registry_cleanup".to_string(),
            )
        })?;

        let initial_count = services.len();
        let stale_threshold = SystemTime::now()
            .checked_sub(self.config.max_service_age)
            .unwrap_or(SystemTime::UNIX_EPOCH);

        services.retain(|_, service| service.last_seen > stale_threshold);

        let removed_count = initial_count - services.len();
        Ok(removed_count)
    }

    /// Get registry statistics
    pub fn get_statistics(&self) -> Result<RegistryStatistics> {
        let services = self.services.read().map_err(|_| {
            NestGateError::simple(
                "Failed to acquire read lock on service registry".to_string(),
                "service_registry_get_stats".to_string(),
            )
        })?;

        let total_services = services.len();
        let mut service_names = std::collections::HashSet::new();
        let mut healthy_services = 0;
        let mut unhealthy_services = 0;

        for service in services.values() {
            service_names.insert(service.name.clone());
            match service.health_status.as_str() {
                "healthy" => healthy_services += 1,
                "unhealthy" => unhealthy_services += 1,
                _ => {}
            }
        }

        Ok(RegistryStatistics {
            total_services,
            unique_service_names: service_names.len(),
            healthy_services,
            unhealthy_services,
            capacity_used: (total_services as f64 / self.config.max_services as f64) * 100.0,
        })
    }

    /// Get registry capacity
    pub fn capacity(&self) -> usize {
        self.config.max_services
    }

    /// Get current service count
    pub fn service_count(&self) -> usize {
        match self.services.read() {
            Ok(services) => services.len(),
            Err(e) => {
                tracing::error!("RwLock read poisoned - returning 0 for service count: {:?}", e);
                0
            }
        }
    }
}

/// Registry statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryStatistics {
    /// Total number of registered services
    pub total_services: usize,
    /// Number of unique service names
    pub unique_service_names: usize,
    /// Number of healthy services
    pub healthy_services: usize,
    /// Number of unhealthy services
    pub unhealthy_services: usize,
    /// Capacity used (percentage)
    pub capacity_used: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_instance_creation() {
        let service =
            ZeroCostServiceInstance::new("test-service".to_string(), "localhost".to_string(), 8080);

        assert_eq!(service.name, "test-service");
        assert_eq!(service.endpoint, "localhost");
        assert_eq!(service.port, 8080);
        assert_eq!(service.address(), "localhost:8080");
    }

    #[test]
    fn test_service_registry_operations() {
        let config = ServiceRegistryConfig::default();
        let registry = ServiceRegistry::new(config);

        let service =
            ZeroCostServiceInstance::new("test-service".to_string(), "localhost".to_string(), 8080);
        let service_id = service.id;

        // Register service
        assert!(registry.register_service(service).is_ok());
        assert_eq!(registry.service_count(), 1);

        // Find service
        let found_services = registry.find_services("test-service").unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed - {}: {:?}", "unable to continue", e),
            )
            .into());
        });
        assert_eq!(found_services.len(), 1);

        // Unregister service
        assert!(registry
            .unregister_service("test-service", service_id)
            .unwrap_or_else(|e| {
                tracing::error!("Unwrap failed: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed - {}: {:?}", "unable to continue", e),
                )
                .into());
            }));
        assert_eq!(registry.service_count(), 0);
    }
}
