// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Service Discovery Trait for Finding and Managing Services
//!
//! **MIGRATED FROM**: `traits::service_discovery` (November 7, 2025)
//! **CANONICAL**: This is the single source of truth for service discovery
//! **STATUS**: Production-ready, native async

use crate::Result;
use crate::service_discovery::types::ServiceInfo;
use crate::unified_enums::service_types::UnifiedServiceState as HealthStatus;
use futures_util::stream::Stream;
use std::collections::HashMap;

/// Service query for filtering discovered services
#[derive(Debug, Clone)]
/// Servicequery
pub struct ServiceQuery {
    /// Service name
    pub service_name: Option<String>,
    /// Tags
    pub tags: Vec<String>,
    /// Namespace
    pub namespace: Option<String>,
    /// Healthy Only
    pub healthy_only: bool,
}

impl ServiceQuery {
    /// Creates a new service query with default settings.
    ///
    /// By default, queries return only healthy services without filtering
    /// by name, tags, or namespace.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            service_name: None,
            tags: Vec::new(),
            namespace: None,
            healthy_only: true,
        }
    }

    /// Builder method to set Name
    #[must_use]
    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.service_name = Some(name.into());
        self
    }

    /// Builder method to set Tag
    #[must_use]
    pub fn with_tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// In Namespace
    #[must_use]
    pub fn in_namespace<S: Into<String>>(mut self, namespace: S) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    /// Modifies the query to include unhealthy services in results.
    ///
    /// By default, only healthy services are returned. Use this method
    /// to include services in any health state.
    #[must_use]
    pub const fn include_unhealthy(mut self) -> Self {
        self.healthy_only = false;
        self
    }
}

impl Default for ServiceQuery {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

/// Service discovery trait for finding and managing services
///
/// This trait provides the interface for service registration, discovery,
/// and health monitoring in the NestGate ecosystem.
pub trait ServiceDiscovery: Send + Sync {
    /// Register a service with the discovery system
    fn register(
        &self,
        service: ServiceInfo,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Deregister a service from the discovery system
    fn deregister(&self, service_id: &str) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Discover services by name
    fn discover(
        &self,
        service_name: &str,
    ) -> impl std::future::Future<Output = Result<Vec<ServiceInfo>>> + Send;

    /// Watch for service changes
    fn watch(
        &self,
    ) -> impl std::future::Future<Output = Result<impl Stream<Item = ServiceEvent>>> + Send;

    /// Update health status for a service
    fn health_update(
        &self,
        service_id: &str,
        status: HealthStatus,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// List all registered services
    fn list_all(&self) -> impl std::future::Future<Output = Result<Vec<ServiceInfo>>> + Send;

    /// Check if a service exists
    fn exists(&self, service_id: &str) -> impl std::future::Future<Output = Result<bool>> + Send;

    /// Update service metadata
    fn update_metadata(
        &self,
        service_id: &str,
        metadata: HashMap<String, String>,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

/// Service event types for discovery notifications
pub enum ServiceEvent {
    /// Service was registered
    Registered(Box<ServiceInfo>),

    /// Service was deregistered
    Deregistered(String),

    /// Service was updated
    Updated {
        /// The unique identifier of the updated service
        service_id: String,
        /// The previous service information before the update
        old_info: Box<ServiceInfo>,
        /// The new service information after the update
        new_info: Box<ServiceInfo>,
    },

    /// Service health status changed
    HealthChanged {
        /// The unique identifier of the service with changed health
        service_id: String,
        /// The previous health status before the change
        old_status: HealthStatus,
        /// The new health status after the change
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
