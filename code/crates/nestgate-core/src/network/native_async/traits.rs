// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use crate::Result;
/// Native Async Network Traits - Zero-Cost Abstractions
/// Extracted from `native_async_network.rs` to maintain file size compliance
/// Contains trait definitions for service discovery, protocol handling, and load balancing
use std::collections::HashMap;
use std::future::Future;
use std::time::Duration;

/// Native async service discovery trait - replaces #\[async_trait\] `ServiceDiscovery`
pub trait NativeAsyncServiceDiscovery<
    const MAX_SERVICES: usize = 10000,
    const DISCOVERY_TIMEOUT_SECS: u64 = 30,
    const WATCH_BUFFER_SIZE: usize = 1000,
    const HEALTH_UPDATE_INTERVAL_SECS: u64 = 60,
>: Send + Sync
{
    /// Type alias for ServiceInfo
    type ServiceInfo: Clone + Send + Sync + 'static;
    /// Type alias for ServiceEvent
    type ServiceEvent: Clone + Send + Sync + 'static;
    /// Type alias for HealthStatus
    type HealthStatus: Clone + Send + Sync + 'static;
    /// Type alias for Query
    type Query: Clone + Send + Sync + 'static;
    /// Register service - native async, no Future boxing
    fn register(&self, service: Self::ServiceInfo) -> impl Future<Output = Result<()>> + Send;

    /// Deregister service - direct async method
    fn deregister(&self, service_id: &str) -> impl Future<Output = Result<()>> + Send;

    /// Discover services - no Future boxing
    fn discover(
        &self,
        service_name: &str,
    ) -> impl Future<Output = Result<Vec<Self::ServiceInfo>>> + Send;

    /// Watches for service changes and returns a stream of events
    fn watch(&self) -> impl Future<Output = Result<Vec<Self::ServiceEvent>>> + Send;

    /// Update health status - compile-time optimization
    fn health_update(
        &self,
        service_id: &str,
        status: Self::HealthStatus,
    ) -> impl Future<Output = Result<()>> + Send;

    /// List all services - direct async method
    fn list_all(&self) -> impl Future<Output = Result<Vec<Self::ServiceInfo>>> + Send;

    /// Check service existence - native async
    fn exists(&self, service_id: &str) -> impl Future<Output = Result<bool>> + Send;

    /// Query services with filters - no Future boxing
    fn query(
        &self,
        query: Self::Query,
    ) -> impl Future<Output = Result<Vec<Self::ServiceInfo>>> + Send;

    /// Get service by ID - direct async method
    fn get_service(
        &self,
        service_id: &str,
    ) -> impl Future<Output = Result<Option<Self::ServiceInfo>>> + Send;

    /// Update service metadata - native async
    fn update_service(
        &self,
        service_id: &str,
        metadata: HashMap<String, String>,
    ) -> impl Future<Output = Result<()>> + Send;

    /// Returns the maximum number of services
    #[must_use]
    fn max_services() -> usize {
        MAX_SERVICES
    }

    /// Returns the discovery timeout in seconds
    #[must_use]
    fn discovery_timeout_seconds() -> u64 {
        DISCOVERY_TIMEOUT_SECS
    }

    /// Returns the watch buffer size
    #[must_use]
    fn watch_buffer_size() -> usize {
        WATCH_BUFFER_SIZE
    }
}

/// Native async network protocol handler trait - replaces #\[async_trait\] `ProtocolHandler`
pub trait NativeAsyncProtocolHandler<
    const MAX_CONNECTIONS: usize = 1000,
    const CONNECTION_TIMEOUT_SECS: u64 = 30,
    const MAX_RETRIES: u32 = 3,
    const BUFFER_SIZE: usize = 8192,
>: Send + Sync
{
    /// Type alias for Connection
    type Connection: Clone + Send + Sync + 'static;
    /// Type alias for Request
    type Request: Clone + Send + Sync + 'static;
    /// Type alias for Response
    type Response: Clone + Send + Sync + 'static;
    /// Type alias for Config
    type Config: Clone + Send + Sync + 'static;
    /// Establish connection - native async, no Future boxing
    fn connect(
        &self,
        config: &Self::Config,
    ) -> impl Future<Output = Result<Self::Connection>> + Send;

    /// Send request - direct async method
    fn send_request(
        &self,
        connection: &Self::Connection,
        request: Self::Request,
    ) -> impl Future<Output = Result<Self::Response>> + Send;

    /// Close connection - no Future boxing
    fn disconnect(&self, connection: &Self::Connection) -> impl Future<Output = Result<()>> + Send;

    /// Handle incoming connection - native async
    fn handle_connection(
        &self,
        connection: Self::Connection,
    ) -> impl Future<Output = Result<()>> + Send;

    /// Get connection status - compile-time optimization
    fn connection_status(
        &self,
        connection: &Self::Connection,
    ) -> impl Future<Output = Result<String>> + Send;

    /// Ping connection - direct async method
    fn ping(&self, _connection: &Self::Connection)
    -> impl Future<Output = Result<Duration>> + Send;

    /// Max connections at compile-time
    /// Returns the maximum number of connections.
    #[must_use]
    fn max_connections() -> usize {
        MAX_CONNECTIONS
    }

    /// Returns the connection timeout in seconds.
    #[must_use]
    fn connection_timeout_seconds() -> u64 {
        CONNECTION_TIMEOUT_SECS
    }

    /// Max retries at compile-time
    #[must_use]
    fn max_retries() -> u32 {
        MAX_RETRIES
    }
}

/// Native async unified service interface trait - replaces #\[async_trait\] `UnifiedServiceInterface`
pub trait NativeAsyncUnifiedServiceInterface<
    const MAX_REQUESTS_PER_SEC: usize = 1000,
    const HEALTH_CHECK_INTERVAL_SECS: u64 = 30,
    const METRICS_BUFFER_SIZE: usize = 10000,
    const SERVICE_TIMEOUT_SECS: u64 = 60,
>: Send + Sync
{
    /// Type alias for HealthStatus
    type HealthStatus: Clone + Send + Sync + 'static;
    /// Type alias for Metrics
    type Metrics: Clone + Send + Sync + 'static;
    /// Type alias for ServiceInfo
    type ServiceInfo: Clone + Send + Sync + 'static;
    /// Type alias for Configuration
    type Configuration: Clone + Send + Sync + 'static;
    /// Get service health - native async, no Future boxing
    fn health(&self) -> impl Future<Output = Result<Self::HealthStatus>> + Send;

    /// Get service metrics - direct async method
    fn metrics(&self) -> impl Future<Output = Result<Self::Metrics>> + Send;

    /// Get service info - no Future boxing
    fn info(&self) -> impl Future<Output = Self::ServiceInfo> + Send;

    /// Start service - native async
    fn start(&self) -> impl Future<Output = Result<()>> + Send;

    /// Stop service - compile-time optimization
    fn stop(&self) -> impl Future<Output = Result<()>> + Send;

    /// Restart service - direct async method
    fn restart(&self) -> impl Future<Output = Result<()>> + Send;

    /// Configure service - native async
    fn configure(&self, config: Self::Configuration) -> impl Future<Output = Result<()>> + Send;

    /// Get configuration - no Future boxing
    fn get_configuration(&self) -> impl Future<Output = Result<Self::Configuration>> + Send;

    /// Validate configuration - direct async method
    fn validate_configuration(
        &self,
        config: &Self::Configuration,
    ) -> impl Future<Output = Result<bool>> + Send;

    /// Max requests per second at compile-time
    #[must_use]
    fn max_requests_per_second() -> usize {
        MAX_REQUESTS_PER_SEC
    }

    /// Returns the health check interval in seconds.
    #[must_use]
    fn health_check_interval_seconds() -> u64 {
        HEALTH_CHECK_INTERVAL_SECS
    }
}

/// Native async load balancer trait - replaces #\[async_trait\] `LoadBalancer`
pub trait NativeAsyncLoadBalancer<
    const MAX_BACKENDS: usize = 100,
    const HEALTH_CHECK_INTERVAL_SECS: u64 = 30,
    const MAX_REQUESTS_PER_BACKEND: usize = 1000,
>: Send + Sync
{
    /// Type alias for Backend
    type Backend: Clone + Send + Sync + 'static;
    /// Type alias for Request
    type Request: Clone + Send + Sync + 'static;
    /// Type alias for Response
    type Response: Clone + Send + Sync + 'static;
    /// Type alias for HealthCheck
    type HealthCheck: Clone + Send + Sync + 'static;
    /// Select backend - native async, no Future boxing
    fn select_backend(
        &self,
        request: &Self::Request,
    ) -> impl Future<Output = Result<Self::Backend>> + Send;

    /// Add backend - direct async method
    fn add_backend(&self, backend: Self::Backend) -> impl Future<Output = Result<()>> + Send;

    /// Remove backend - no Future boxing
    fn remove_backend(&self, backend_id: &str) -> impl Future<Output = Result<()>> + Send;

    /// Get backend health - native async
    fn backend_health(
        &self,
        backend_id: &str,
    ) -> impl Future<Output = Result<Self::HealthCheck>> + Send;

    /// List all backends - compile-time optimization
    fn list_backends(&self) -> impl Future<Output = Result<Vec<Self::Backend>>> + Send;

    /// Update backend weight - direct async method
    fn update_backend_weight(
        &self,
        backend_id: &str,
        weight: u32,
    ) -> impl Future<Output = Result<()>> + Send;

    /// Max backends at compile-time
    #[must_use]
    fn max_backends() -> usize {
        MAX_BACKENDS
    }

    /// Returns the health check interval in seconds.
    #[must_use]
    fn health_check_interval_seconds() -> u64 {
        HEALTH_CHECK_INTERVAL_SECS
    }
}
