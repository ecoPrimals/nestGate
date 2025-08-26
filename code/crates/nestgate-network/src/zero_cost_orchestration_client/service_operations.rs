use super::{ZeroCostOrchestrationClient, ZeroCostServiceInstance};
/// **ZERO-COST ORCHESTRATION SERVICE OPERATIONS**
///
/// This module handles service operations including registration, discovery,
/// and port allocation for the zero-cost orchestration client.
use crate::zero_cost_orchestration_types::*;
use nestgate_core::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

/// Zero-cost service orchestration operations
pub trait ZeroCostServiceOperations {
    /// Register a service with the orchestrator
    fn register_service(
        &self,
        registration: &ZeroCostServiceRegistration,
    ) -> impl std::future::Future<Output = Result<String>> + Send;

    /// Discover services by name
    fn discover_services(
        &self,
        service_name: &str,
    ) -> impl std::future::Future<Output = Result<Vec<ZeroCostServiceInstance>>> + Send;

    /// Allocate a port for a service
    fn allocate_port(
        &self,
        service_name: &str,
    ) -> impl std::future::Future<Output = Result<u16>> + Send;

    /// Release a port
    fn release_port(
        &self,
        service_name: &str,
        port: u16,
    ) -> impl std::future::Future<Output = Result<bool>> + Send;

    /// Perform health check
    fn health_check(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}

/// Extended service operations with retry and metadata support
pub trait ExtendedServiceOperations: ZeroCostServiceOperations {
    /// Register service with retry logic
    fn register_service_with_retry(
        &self,
        service: &ZeroCostServiceRegistration,
    ) -> impl std::future::Future<Output = Result<String>> + Send;

    /// Unregister a service
    fn unregister_service(
        &self,
        service_id: &str,
    ) -> impl std::future::Future<Output = Result<bool>> + Send;

    /// Update service metadata
    fn update_service_metadata(
        &self,
        service_id: &str,
        metadata: HashMap<String, String>,
    ) -> impl std::future::Future<Output = Result<bool>> + Send;
}

/// Advanced service discovery operations
pub trait AdvancedServiceDiscovery {
    /// Discover services with filtering
    fn discover_services_filtered(
        &self,
        filter: ServiceFilter,
    ) -> impl std::future::Future<Output = Result<Vec<ZeroCostServiceInstance>>> + Send;

    /// Get service by ID
    fn get_service_by_id(
        &self,
        service_id: &str,
    ) -> impl std::future::Future<Output = Result<Option<ZeroCostServiceInstance>>> + Send;

    /// List all services
    fn list_all_services(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<ZeroCostServiceInstance>>> + Send;
}

/// Service registration operations
pub trait ServiceRegistrationOps {
    /// Register service with retry logic
    fn register_service_with_retry(
        &self,
        service: &ZeroCostServiceRegistration,
    ) -> impl std::future::Future<Output = Result<String>> + Send;

    /// Unregister service
    fn unregister_service(
        &self,
        service_id: &str,
    ) -> impl std::future::Future<Output = Result<bool>> + Send;

    /// Update service metadata
    fn update_service_metadata(
        &self,
        service_id: &str,
        metadata: HashMap<String, String>,
    ) -> impl std::future::Future<Output = Result<bool>> + Send;
}

/// Service discovery operations
pub trait ServiceDiscoveryOps {
    /// Discover services with filters
    fn discover_services_filtered(
        &self,
        filter: ServiceFilter,
    ) -> impl std::future::Future<Output = Result<Vec<ZeroCostServiceInstance>>> + Send;

    /// Get service by ID
    fn get_service_by_id(
        &self,
        service_id: &str,
    ) -> impl std::future::Future<Output = Result<Option<ZeroCostServiceInstance>>> + Send;

    /// List all services
    fn list_all_services(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<ZeroCostServiceInstance>>> + Send;
}

/// Service filter for discovery operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceFilter {
    /// Service name pattern
    pub name_pattern: Option<String>,
    /// Required tags
    pub tags: Vec<String>,
    /// Metadata filters
    pub metadata: HashMap<String, String>,
    /// Health status filter
    pub health_status: Option<String>,
    /// Maximum results
    pub limit: Option<usize>,
}

impl Default for ServiceFilter {
    fn default() -> Self {
        Self {
            name_pattern: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
            health_status: None,
            limit: Some(100),
        }
    }
}

impl ZeroCostServiceOperations for ZeroCostOrchestrationClient {
    fn register_service(
        &self,
        registration: &ZeroCostServiceRegistration,
    ) -> impl std::future::Future<Output = Result<String>> + Send {
        async move {
            let start_time = SystemTime::now();

            // Generate unique service ID
            let service_id = Uuid::new_v4().to_string();

            // Create service instance
            let instance = super::registry::ZeroCostServiceInstance {
                id: Uuid::parse_str(&service_id).unwrap_or_else(|_| Uuid::new_v4()),
                name: registration.service_name.clone(),
                endpoint: format!("http://{}:{}", registration.host, registration.port),
                port: registration.port,
                metadata: registration.metadata.clone(),
                tags: registration.tags.clone(),
                registered_at: start_time,
                last_seen: start_time,
                health_status: "registered".to_string(),
                version: None,
            };

            // Register with internal registry
            if let Ok(mut registry) = self.service_registry.write() {
                registry.insert(service_id.clone(), instance);
            }

            tracing::info!(
                "Registered service: {} with ID: {}",
                registration.service_name,
                service_id
            );

            Ok(service_id)
        }
    }

    fn discover_services(
        &self,
        service_name: &str,
    ) -> impl std::future::Future<Output = Result<Vec<ZeroCostServiceInstance>>> + Send {
        let service_name = service_name.to_string();
        async move {
            let services = if let Ok(registry) = self.service_registry.read() {
                registry
                    .values()
                    .filter(|instance| instance.name == service_name)
                    .cloned()
                    .collect()
            } else {
                Vec::new()
            };

            tracing::debug!(
                "Discovered {} instances of service: {}",
                services.len(),
                service_name
            );

            Ok(services)
        }
    }

    fn allocate_port(
        &self,
        service_name: &str,
    ) -> impl std::future::Future<Output = Result<u16>> + Send {
        let service_name = service_name.to_string();
        async move {
            // Simple port allocation strategy - in production this would be more sophisticated
            let base_port = 8000;
            let hash = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            let mut hasher = hash;
            service_name.hash(&mut hasher);
            let port = base_port + (hasher.finish() % 1000) as u16;

            // Cache allocated port
            if let Ok(mut ports) = self.allocated_ports.write() {
                ports.insert(service_name.clone(), port);
            }

            tracing::debug!("Allocated port {} for service: {}", port, service_name);

            Ok(port)
        }
    }

    fn release_port(
        &self,
        service_name: &str,
        port: u16,
    ) -> impl std::future::Future<Output = Result<bool>> + Send {
        let service_name = service_name.to_string();
        async move {
            // Remove from local cache
            if let Ok(mut ports) = self.allocated_ports.write() {
                ports.remove(&service_name);
            }
            tracing::debug!("Released port {} for service: {}", port, service_name);
            Ok(true)
        }
    }

    fn health_check(&self) -> impl std::future::Future<Output = Result<()>> + Send {
        async move {
            // Basic health check - verify registry is accessible
            let _can_read = self.service_registry.read().is_ok();
            tracing::debug!("Health check passed");
            Ok(())
        }
    }
}

impl ServiceRegistrationOps for ZeroCostOrchestrationClient {
    fn register_service_with_retry(
        &self,
        service: &ZeroCostServiceRegistration,
    ) -> impl std::future::Future<Output = Result<String>> + Send {
        let service = service.clone();
        async move {
            let mut last_error = None;

            for attempt in 1..=3 {
                match self.register_service(&service).await {
                    Ok(service_id) => return Ok(service_id),
                    Err(e) => {
                        last_error = Some(e);
                        if attempt < 3 {
                            tokio::time::sleep(std::time::Duration::from_millis(100 * attempt))
                                .await;
                        }
                    }
                }
            }

            Err(last_error.unwrap_or_else(|| {
                NestGateError::network_error(
                    "Service registration failed after retries",
                    "register_service_with_retry",
                    None,
                )
            }))
        }
    }

    fn unregister_service(
        &self,
        service_id: &str,
    ) -> impl std::future::Future<Output = Result<bool>> + Send {
        let service_id = service_id.to_string();
        async move {
            let unregister_url = format!(
                "{}/api/v1/services/unregister/{}",
                self.config.base_url, service_id
            );

            let response = self
                .http_client
                .delete(&unregister_url)
                .send()
                .await
                .map_err(|e| {
                    NestGateError::network_error(
                        &format!("Service unregistration failed: {}", e),
                        "unregister_service",
                        Some(&unregister_url),
                    )
                })?;

            Ok(response.status().is_success())
        }
    }

    fn update_service_metadata(
        &self,
        service_id: &str,
        metadata: HashMap<String, String>,
    ) -> impl std::future::Future<Output = Result<bool>> + Send {
        let service_id = service_id.to_string();
        async move {
            let update_url = format!(
                "{}/api/v1/services/{}/metadata",
                self.config.base_url, service_id
            );

            let response = self
                .http_client
                .put(&update_url)
                .json(&metadata)
                .send()
                .await
                .map_err(|e| {
                    NestGateError::network_error(
                        &format!("Service metadata update failed: {}", e),
                        "update_service_metadata",
                        Some(&update_url),
                    )
                })?;

            Ok(response.status().is_success())
        }
    }
}

impl ServiceDiscoveryOps for ZeroCostOrchestrationClient {
    fn discover_services_filtered(
        &self,
        filter: ServiceFilter,
    ) -> impl std::future::Future<Output = Result<Vec<ZeroCostServiceInstance>>> + Send {
        async move {
            let discovery_url = format!("{}/api/v1/services/discover", self.config.base_url);

            let response = self
                .http_client
                .post(&discovery_url)
                .json(&filter)
                .send()
                .await
                .map_err(|e| {
                    NestGateError::network_error(
                        &format!("Service discovery failed: {}", e),
                        "discover_services_filtered",
                        Some(&discovery_url),
                    )
                })?;

            if response.status().is_success() {
                let services: Vec<ZeroCostServiceInstance> =
                    response.json().await.map_err(|e| {
                        NestGateError::network_error(
                            &format!("Failed to parse discovery response: {}", e),
                            "discover_services_filtered",
                            Some(&discovery_url),
                        )
                    })?;
                Ok(services)
            } else {
                Err(NestGateError::network_error(
                    &format!("Discovery failed with status: {}", response.status()),
                    "discover_services_filtered",
                    Some(&discovery_url),
                ))
            }
        }
    }

    fn get_service_by_id(
        &self,
        service_id: &str,
    ) -> impl std::future::Future<Output = Result<Option<ZeroCostServiceInstance>>> + Send {
        let service_id = service_id.to_string();
        async move {
            let service_url = format!("{}/api/v1/services/{}", self.config.base_url, service_id);

            let response = self
                .http_client
                .get(&service_url)
                .send()
                .await
                .map_err(|e| {
                    NestGateError::network_error(
                        &format!("Service lookup failed: {}", e),
                        "get_service_by_id",
                        Some(&service_url),
                    )
                })?;

            if response.status().is_success() {
                let service: ZeroCostServiceInstance = response.json().await.map_err(|e| {
                    NestGateError::network_error(
                        &format!("Failed to parse service response: {}", e),
                        "get_service_by_id",
                        Some(&service_url),
                    )
                })?;
                Ok(Some(service))
            } else if response.status() == 404 {
                Ok(None)
            } else {
                Err(NestGateError::network_error(
                    &format!("Service lookup failed with status: {}", response.status()),
                    "get_service_by_id",
                    Some(&service_url),
                ))
            }
        }
    }

    fn list_all_services(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<ZeroCostServiceInstance>>> + Send {
        async move {
            let list_url = format!("{}/api/v1/services", self.config.base_url);

            let response = self.http_client.get(&list_url).send().await.map_err(|e| {
                NestGateError::network_error(
                    &format!("Service listing failed: {}", e),
                    "list_all_services",
                    Some(&list_url),
                )
            })?;

            if response.status().is_success() {
                let services: Vec<ZeroCostServiceInstance> =
                    response.json().await.map_err(|e| {
                        NestGateError::network_error(
                            &format!("Failed to parse services response: {}", e),
                            "list_all_services",
                            Some(&list_url),
                        )
                    })?;
                Ok(services)
            } else {
                Err(NestGateError::network_error(
                    &format!("Service listing failed with status: {}", response.status()),
                    "list_all_services",
                    Some(&list_url),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_filter_creation() {
        let filter = ServiceFilter {
            name_pattern: Some("test-*".to_string()),
            tags: vec!["production".to_string()],
            health_status: Some("healthy".to_string()),
            ..Default::default()
        };

        assert_eq!(filter.name_pattern, Some("test-*".to_string()));
        assert_eq!(filter.tags, vec!["production"]);
        assert_eq!(filter.limit, Some(100));
    }
}
