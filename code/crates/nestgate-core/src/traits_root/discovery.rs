// Removed unused error imports
/// Service discovery traits and types
use async_trait::async_trait;
use futures_util::stream::Stream;
use std::collections::HashMap;

use crate::errors::Result;
use crate::traits_root::health::HealthStatus;
use crate::traits_root::service::ServiceInfo;

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
#[async_trait]
pub trait ServiceDiscovery: Send + Sync {
    /// Register a service with the discovery system
    async fn register(&self, service: ServiceInfo) -> Result<()>;

    /// Deregister a service from the discovery system
    async fn deregister(&self, service_id: &str) -> Result<()>;

    /// Discover services by name
    async fn discover(&self, service_name: &str) -> Result<Vec<ServiceInfo>>;

    /// Watch for service changes
    async fn watch(&self) -> Result<impl Stream<Item = ServiceEvent> + Send>;

    /// Update health status for a service
    async fn health_update(&self, service_id: &str, status: HealthStatus) -> Result<()>;

    /// List all registered services
    async fn list_all(&self) -> Result<Vec<ServiceInfo>>;

    /// Check if a service exists
    async fn exists(&self, service_id: &str) -> Result<bool>;

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
