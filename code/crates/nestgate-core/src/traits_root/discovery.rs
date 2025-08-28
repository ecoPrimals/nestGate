// Removed unused error imports
/// Service discovery traits and types
/// 
/// **MIGRATION NOTE**: This module has been updated to use canonical trait types
use futures_util::stream::Stream;
use std::collections::HashMap;

use crate::Result;
use crate::unified_enums::service_types::UnifiedServiceState as HealthStatus;
use crate::traits::ServiceRegistration as ServiceInfo;

/// Service query for filtering discovered services
#[derive(Debug, Clone)]
pub struct ServiceQuery {
    pub service_name: Option<String>,
    pub tags: Vec<String>,
    pub namespace: Option<String>,
    pub healthy_only: bool,
}

impl ServiceQuery {
    pub fn new() -> Self {
        Self {
            service_name: None,
            tags: Vec::new(),
            namespace: None,
            healthy_only: true,
        }
    }

    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.service_name = Some(name.into());
        self
    }

    pub fn with_tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn in_namespace<S: Into<String>>(mut self, namespace: S) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    pub fn include_unhealthy(mut self) -> Self {
        self.healthy_only = false;
        self
    }
}

impl Default for ServiceQuery {
    fn default() -> Self {
        Self::new()
    }
}

/// Service discovery trait for finding and managing services
pub trait ServiceDiscovery: Send + Sync {
    /// Register a service with the discovery system
    fn register(&self, service: ServiceInfo) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Deregister a service from the discovery system
    fn deregister(&self, service_id: &str) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Discover services by name
    fn discover(&self, service_name: &str) -> impl std::future::Future<Output = Result<Vec<ServiceInfo>> + Send;

    /// Watch for service changes
    fn watch(&self) -> impl std::future::Future<Output = Result<impl Stream<Item = ServiceEvent>> + Send;

    /// Update health status for a service
    fn health_update(&self, service_id: &str, status: HealthStatus) -> impl std::future::Future<Output = Result<()>> + Send;

    /// List all registered services
    fn list_all(&self) -> impl std::future::Future<Output = Result<Vec<ServiceInfo>> + Send;

    /// Check if a service exists
    fn exists(&self, service_id: &str) -> impl std::future::Future<Output = Result<bool>> + Send;

    /// Update service metadata
    async fn update_metadata(
        &self,
        service_id: &str,
        metadata: HashMap<String, String>,
    ) -> Result<()>;
}

/// Service event types for discovery notifications
pub enum ServiceEvent {
    /// Service was registered
    Registered(Box<ServiceInfo>),

    /// Service was deregistered
    Deregistered(String),

    /// Service was updated
    Updated {
        service_id: String,
        old_info: Box<ServiceInfo>,
        new_info: Box<ServiceInfo>,
    },

    /// Service health status changed
    HealthChanged {
        service_id: String,
        old_status: HealthStatus,
        new_status: HealthStatus,
    },

    /// Service became available
    Available(String),

    /// Service became unavailable
    Unavailable(String),

    /// Discovery system lost connection
    ConnectionLost,

    /// Discovery system restored connection
    ConnectionRestored,
}
