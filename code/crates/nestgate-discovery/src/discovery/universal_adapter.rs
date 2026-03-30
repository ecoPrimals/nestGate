// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **UNIVERSAL ADAPTER**
//!
//! O(1) connection management for capability-based architecture.

use super::capability_scanner::CapabilityInfo;
use bytes::Bytes;
use nestgate_types::error::NestGateError;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, info, warn};

/// Generic request type for capability communication with zero-copy optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for  operation
pub struct Request {
    /// Request ID for tracking (zero-copy string)
    pub id: Cow<'static, str>,
    /// Request method/action (zero-copy string)
    pub method: Cow<'static, str>,
    /// Request parameters
    pub params: serde_json::Value,
    /// Request headers (zero-copy keys and values)
    pub headers: HashMap<Cow<'static, str>, Cow<'static, str>>,
    /// Raw body for zero-copy operations
    #[serde(skip)]
    /// Body
    pub body: Option<Bytes>,
}

/// Generic response type for capability communication with zero-copy optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for  operation
pub struct Response {
    /// Response ID matching request (zero-copy string)
    pub id: Cow<'static, str>,
    /// Success status
    pub success: bool,
    /// Response data
    pub data: serde_json::Value,
    /// Error message if failed (zero-copy string)
    pub error: Option<Cow<'static, str>>,
    /// Response headers (zero-copy keys and values)
    pub headers: HashMap<Cow<'static, str>, Cow<'static, str>>,
    /// Raw body for zero-copy operations
    #[serde(skip)]
    /// Body
    pub body: Option<Bytes>,
}

/// Health status for connections
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Health
pub enum HealthStatus {
    /// Connection is healthy and responsive
    Healthy,
    /// Connection has warnings but functional
    Warning,
    /// Connection is degraded but operational
    Degraded,
    /// Connection is unhealthy/failed
    Unhealthy,
}

/// Connection metadata
#[derive(Debug, Clone)]
/// Connectionmetadata
pub struct ConnectionMetadata {
    /// Connection type identifier
    pub connection_type: String,
    /// Endpoint URL
    pub endpoint: String,
    /// Connection creation timestamp
    pub created_at: std::time::SystemTime,
    /// Last health check timestamp
    pub last_health_check: Option<std::time::SystemTime>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Trait for connections to capabilities
///
/// **NATIVE ASYNC**: Uses `impl Future` for zero-cost abstractions.
/// Uses enum dispatch pattern for polymorphism without trait objects.
pub trait Connection: Send + Sync {
    /// Send a request to the capability - native async, no boxing
    fn send_request(
        &self,
        request: Request,
    ) -> impl Future<Output = Result<Response, NestGateError>> + Send;

    /// Check health of the connection - native async
    fn health_check(&self) -> impl Future<Output = Result<HealthStatus, NestGateError>> + Send;

    /// Get connection metadata - native async
    fn get_metadata(
        &self,
    ) -> impl Future<Output = Result<ConnectionMetadata, NestGateError>> + Send;

    /// Get connection type
    fn connection_type(&self) -> &str;

    /// Get endpoint
    fn endpoint(&self) -> &str;
}

/// Enum wrapper for Connection implementations (enum dispatch pattern)
///
/// This enables use of Connection trait without trait objects while maintaining
/// zero-cost abstractions. Add new variants here as new connection types are implemented.
#[derive(Clone)]
/// Connectionimpl
pub enum ConnectionImpl {
    /// HTTP-based connection
    Http(HttpConnection),
}

impl Connection for ConnectionImpl {
    /// Send Request
    async fn send_request(&self, request: Request) -> Result<Response, NestGateError> {
        match self {
            Self::Http(conn) => conn.send_request(request).await,
        }
    }

    /// Health Check
    async fn health_check(&self) -> Result<HealthStatus, NestGateError> {
        match self {
            Self::Http(conn) => conn.health_check().await,
        }
    }

    /// Gets Metadata
    async fn get_metadata(&self) -> Result<ConnectionMetadata, NestGateError> {
        match self {
            Self::Http(conn) => conn.get_metadata().await,
        }
    }

    /// Connection Type
    fn connection_type(&self) -> &str {
        match self {
            Self::Http(conn) => conn.connection_type(),
        }
    }

    /// Endpoint
    fn endpoint(&self) -> &str {
        match self {
            Self::Http(conn) => conn.endpoint(),
        }
    }
}

/// HTTP-based connection implementation
#[derive(Clone)]
/// HTTP connection for external HTTP delegation to Songbird
///
/// **`BiomeOS` Architecture**: `NestGate` does NOT make external HTTP calls directly.
/// All external HTTP is delegated to Songbird primal via JSON-RPC over Unix sockets.
/// This struct exists for capability routing, NOT for actual HTTP client usage.
pub struct HttpConnection {
    /// Capability information
    capability_info: CapabilityInfo,
    /// Connection metadata
    metadata: ConnectionMetadata,
}

impl HttpConnection {
    /// Create a new HTTP connection
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    pub fn new(capability_info: CapabilityInfo) -> Result<Self, NestGateError> {
        let metadata = ConnectionMetadata {
            connection_type: "http".to_string(),
            endpoint: capability_info.endpoint.clone(),
            created_at: std::time::SystemTime::now(),
            last_health_check: None,
            metadata: capability_info.metadata.clone(),
        };

        Ok(Self {
            capability_info,
            metadata,
        })
    }
}

impl Connection for HttpConnection {
    /// Send Request
    async fn send_request(&self, request: Request) -> Result<Response, NestGateError> {
        debug!(
            "Request {} for {} - using tarpc for primal communication",
            request.id, self.capability_info.endpoint
        );

        // BiomeOS Concentrated Gap: External HTTP removed
        // For primal-to-primal: Use tarpc (Unix sockets)
        // For external HTTP: Use Songbird via discover_capability("external-http")
        Err(NestGateError::api_error(
            "HTTP adapter deprecated. Use tarpc for primal-to-primal or Songbird RPC for external HTTP",
        ))

        // REMOVED: HTTP client implementation (BiomeOS evolution)
        // Previous implementation: ~45 lines of HTTP request/response handling
        // Migration: Use tarpc for primal-to-primal, Songbird RPC for external HTTP
    }

    /// Health Check
    async fn health_check(&self) -> Result<HealthStatus, NestGateError> {
        debug!(
            "Health checking {} - using tarpc health API",
            self.capability_info.endpoint
        );

        // BiomeOS Concentrated Gap: Use tarpc for health checks
        // HTTP health checks removed - primals use Unix socket health APIs
        // For external services: Use Songbird's health proxy

        // Return degraded status (HTTP deprecated)
        // Real implementation should use tarpc health RPC
        Ok(HealthStatus::Degraded)

        // REMOVED: HTTP health check implementation
        /* Previous HTTP-based health check removed
        match self.client.get(&health_url).send().await { ... }
        */
    }

    /// Gets Metadata
    async fn get_metadata(&self) -> Result<ConnectionMetadata, NestGateError> {
        Ok(self.metadata.clone())
    }

    /// Connection Type
    fn connection_type(&self) -> &'static str {
        "http"
    }

    /// Endpoint
    fn endpoint(&self) -> &str {
        &self.capability_info.endpoint
    }
}

/// Universal adapter for O(1) capability connections
pub struct UniversalAdapter {
    /// Registry of discovered capabilities
    capability_registry: Arc<RwLock<HashMap<String, CapabilityInfo>>>,
    /// Pool of active connections (using enum dispatch, not trait objects)
    connection_pool: Arc<Mutex<HashMap<String, Arc<ConnectionImpl>>>>,
    /// Adapter metrics
    metrics: Arc<Mutex<AdapterMetrics>>,
}

/// Metrics for the universal adapter
#[derive(Debug, Default)]
/// Adaptermetrics
pub struct AdapterMetrics {
    /// Total requests processed
    pub total_requests: u64,
    /// Total connections created
    pub total_connections: u64,
    /// Cache hit rate
    pub cache_hits: u64,
    /// Cache misses
    pub cache_misses: u64,
    /// Failed connections
    pub failed_connections: u64,
}

impl UniversalAdapter {
    /// Create a new universal adapter
    #[must_use]
    pub fn new() -> Self {
        Self {
            capability_registry: Arc::new(RwLock::new(HashMap::new())),
            connection_pool: Arc::new(Mutex::new(HashMap::new())),
            metrics: Arc::new(Mutex::new(AdapterMetrics::default())),
        }
    }

    /// Register a discovered capability
    pub async fn register_capability(&self, capability: CapabilityInfo) {
        let mut registry = self.capability_registry.write().await;
        info!(
            "Registering capability: {} -> {}",
            capability.capability_type, capability.endpoint
        );
        registry.insert(capability.capability_type.clone(), capability);
    }

    /// Get capability with O(1) complexity
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_capability(
        &self,
        capability_type: &str,
    ) -> Result<Arc<ConnectionImpl>, NestGateError> {
        // Check connection pool first (O(1))
        {
            let pool = self.connection_pool.lock().await;
            if let Some(connection) = pool.get(capability_type) {
                let mut metrics = self.metrics.lock().await;
                metrics.cache_hits += 1;
                debug!("Using cached connection for {}", capability_type);
                return Ok(connection.clone());
            }
        }

        // Get capability info and create connection (O(1))
        let capability = {
            let registry = self.capability_registry.read().await;
            registry.get(capability_type).cloned()
        };

        if let Some(capability) = capability {
            let connection = self.create_connection(capability)?;

            // Cache connection for future use
            let mut pool = self.connection_pool.lock().await;
            pool.insert(capability_type.to_string(), connection.clone());

            let mut metrics = self.metrics.lock().await;
            metrics.cache_misses += 1;
            metrics.total_connections += 1;

            info!("Created new connection for {}", capability_type);
            Ok(connection)
        } else {
            warn!("Capability {} not found in registry", capability_type);
            Err(NestGateError::Internal(Box::new(
                nestgate_types::error::variants::core_errors::InternalErrorDetails {
                    message: format!("Capability {capability_type} not discovered").into(),
                    component: "universal_adapter".into(),
                    location: Some(format!("{}:{}", file!(), line!()).into()),
                    is_bug: false,
                    context: None,
                },
            )))
        }
    }

    /// Create a connection for a capability
    fn create_connection(
        &self,
        capability: CapabilityInfo,
    ) -> Result<Arc<ConnectionImpl>, NestGateError> {
        debug!(
            "Creating connection for capability: {}",
            capability.capability_type
        );

        // For now, default to HTTP connections
        // In the future, this could be extended to support other protocols
        let connection = HttpConnection::new(capability)?;
        Ok(Arc::new(ConnectionImpl::Http(connection)))
    }

    /// Get adapter metrics
    pub async fn get_metrics(&self) -> AdapterMetrics {
        let metrics = self.metrics.lock().await;
        AdapterMetrics {
            total_requests: metrics.total_requests,
            total_connections: metrics.total_connections,
            cache_hits: metrics.cache_hits,
            cache_misses: metrics.cache_misses,
            failed_connections: metrics.failed_connections,
        }
    }

    /// Clear connection pool (force reconnection)
    pub async fn clear_connections(&self) {
        let mut pool = self.connection_pool.lock().await;
        pool.clear();
        info!("Cleared all connections from pool");
    }

    /// Health check all connections
    pub async fn health_check_all(&self) -> HashMap<String, HealthStatus> {
        let pool = self.connection_pool.lock().await;
        let mut results = HashMap::new();

        for (capability_type, connection) in pool.iter() {
            match connection.health_check().await {
                Ok(status) => {
                    results.insert(capability_type.clone(), status);
                }
                Err(_) => {
                    results.insert(capability_type.clone(), HealthStatus::Unhealthy);
                }
            }
        }

        results
    }
}

impl Default for UniversalAdapter {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_universal_adapter() {
        let adapter = UniversalAdapter::new();

        // Register a test capability
        let mut metadata = HashMap::new();
        metadata.insert("test".to_string(), "true".to_string());

        use nestgate_config::constants::{
            network_defaults::LOCALHOST_NAME, port_defaults::get_admin_port,
        };
        let endpoint = format!("http://{}:{}", LOCALHOST_NAME, get_admin_port());
        let capability = CapabilityInfo {
            capability_type: "test".to_string(),
            endpoint,
            confidence: 1.0,
            metadata,
        };

        adapter.register_capability(capability).await;

        // Test that capability is registered
        let registry = adapter.capability_registry.read().await;
        assert!(registry.contains_key("test"));
    }

    #[tokio::test]
    async fn test_adapter_metrics() {
        let adapter = UniversalAdapter::new();
        let metrics = adapter.get_metrics().await;

        assert_eq!(metrics.total_requests, 0);
        assert_eq!(metrics.total_connections, 0);
        assert_eq!(metrics.cache_hits, 0);
        assert_eq!(metrics.cache_misses, 0);
    }

    #[tokio::test]
    async fn get_capability_miss_then_hit_cache() {
        use nestgate_config::constants::{
            network_defaults::LOCALHOST_NAME, port_defaults::get_admin_port,
        };

        let adapter = UniversalAdapter::new();
        let endpoint = format!("http://{}:{}", LOCALHOST_NAME, get_admin_port());
        let capability = CapabilityInfo {
            capability_type: "alpha".to_string(),
            endpoint: endpoint.clone(),
            confidence: 1.0,
            metadata: HashMap::new(),
        };
        adapter.register_capability(capability).await;

        let c1 = adapter.get_capability("alpha").await.expect("first");
        let c2 = adapter.get_capability("alpha").await.expect("cached");
        assert_eq!(c1.endpoint(), c2.endpoint());

        let m = adapter.get_metrics().await;
        assert_eq!(m.cache_misses, 1);
        assert_eq!(m.cache_hits, 1);
        assert_eq!(m.total_connections, 1);
    }

    #[tokio::test]
    async fn get_capability_unknown_errors() {
        let adapter = UniversalAdapter::new();
        let err = adapter.get_capability("nope").await.err().expect("err");
        assert!(err.to_string().contains("nope") || err.to_string().contains("Capability"));
    }

    #[tokio::test]
    async fn clear_connections_and_health_check_all() {
        use nestgate_config::constants::{
            network_defaults::LOCALHOST_NAME, port_defaults::get_admin_port,
        };

        let adapter = UniversalAdapter::new();
        let endpoint = format!("http://{}:{}", LOCALHOST_NAME, get_admin_port());
        adapter
            .register_capability(CapabilityInfo {
                capability_type: "h".to_string(),
                endpoint,
                confidence: 1.0,
                metadata: HashMap::new(),
            })
            .await;
        let _ = adapter.get_capability("h").await.unwrap();
        adapter.clear_connections().await;
        let m = adapter.get_metrics().await;
        assert_eq!(m.total_connections, 1);

        adapter
            .register_capability(CapabilityInfo {
                capability_type: "h2".to_string(),
                endpoint: format!("http://{}:{}", LOCALHOST_NAME, get_admin_port()),
                confidence: 1.0,
                metadata: HashMap::new(),
            })
            .await;
        let _ = adapter.get_capability("h2").await.unwrap();
        let health = adapter.health_check_all().await;
        assert!(health.contains_key("h2"));
    }

    #[tokio::test]
    async fn http_connection_trait_methods() {
        use nestgate_config::constants::{
            network_defaults::LOCALHOST_NAME, port_defaults::get_admin_port,
        };

        let endpoint = format!("http://{}:{}", LOCALHOST_NAME, get_admin_port());
        let info = CapabilityInfo {
            capability_type: "x".to_string(),
            endpoint: endpoint.clone(),
            confidence: 0.5,
            metadata: HashMap::from([("k".to_string(), "v".to_string())]),
        };
        let http = HttpConnection::new(info).expect("http conn");
        assert_eq!(http.connection_type(), "http");
        assert_eq!(http.endpoint(), endpoint.as_str());

        let meta = http.get_metadata().await.expect("meta");
        assert_eq!(meta.connection_type, "http");
        assert_eq!(meta.endpoint, endpoint);

        let hc = http.health_check().await.expect("health");
        assert!(matches!(hc, HealthStatus::Degraded));

        let err = http
            .send_request(Request {
                id: std::borrow::Cow::Borrowed("1"),
                method: std::borrow::Cow::Borrowed("m"),
                params: serde_json::json!({}),
                headers: HashMap::new(),
                body: None,
            })
            .await
            .expect_err("deprecated http");
        assert!(err.to_string().contains("HTTP") || err.to_string().contains("deprecated"));

        let conn = ConnectionImpl::Http(http);
        let _: &str = conn.connection_type();
        let _: &str = conn.endpoint();
    }

    #[test]
    fn request_response_serde_roundtrip() {
        let req = Request {
            id: std::borrow::Cow::Borrowed("rid"),
            method: std::borrow::Cow::Borrowed("m"),
            params: serde_json::json!({"a": 1}),
            headers: HashMap::new(),
            body: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        let back: Request = serde_json::from_str(&s).unwrap();
        assert_eq!(back.id, req.id);
        assert_eq!(back.method, req.method);

        let resp = Response {
            id: std::borrow::Cow::Borrowed("rid"),
            success: true,
            data: serde_json::json!({"ok": true}),
            error: None,
            headers: HashMap::new(),
            body: None,
        };
        let s2 = serde_json::to_string(&resp).unwrap();
        let back2: Response = serde_json::from_str(&s2).unwrap();
        assert!(back2.success);
    }

    #[tokio::test]
    async fn universal_adapter_default_matches_new() {
        let a = UniversalAdapter::default();
        let b = UniversalAdapter::new();
        let ma = a.get_metrics().await;
        let mb = b.get_metrics().await;
        assert_eq!(ma.total_requests, mb.total_requests);
        assert_eq!(ma.failed_connections, mb.failed_connections);
    }
}
