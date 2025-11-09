//! **UNIVERSAL ADAPTER**
//!
//! O(1) connection management for capability-based architecture.

use super::capability_scanner::CapabilityInfo;
use crate::error::NestGateError;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, info, warn};

/// Generic request type for capability communication with zero-copy optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub body: Option<Bytes>,
}

/// Generic response type for capability communication with zero-copy optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub body: Option<Bytes>,
}

/// Health status for connections
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    fn send_request(&self, request: Request) -> impl Future<Output = Result<Response, NestGateError>> + Send;

    /// Check health of the connection - native async
    fn health_check(&self) -> impl Future<Output = Result<HealthStatus, NestGateError>> + Send;

    /// Get connection metadata - native async
    fn get_metadata(&self) -> impl Future<Output = Result<ConnectionMetadata, NestGateError>> + Send;

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
pub enum ConnectionImpl {
    /// HTTP-based connection
    Http(HttpConnection),
}

impl Connection for ConnectionImpl {
    fn send_request(&self, request: Request) -> impl Future<Output = Result<Response, NestGateError>> + Send {
        async move {
            match self {
                Self::Http(conn) => conn.send_request(request).await,
            }
        }
    }

    fn health_check(&self) -> impl Future<Output = Result<HealthStatus, NestGateError>> + Send {
        async move {
            match self {
                Self::Http(conn) => conn.health_check().await,
            }
        }
    }

    fn get_metadata(&self) -> impl Future<Output = Result<ConnectionMetadata, NestGateError>> + Send {
        async move {
            match self {
                Self::Http(conn) => conn.get_metadata().await,
            }
        }
    }

    fn connection_type(&self) -> &str {
        match self {
            Self::Http(conn) => conn.connection_type(),
        }
    }

    fn endpoint(&self) -> &str {
        match self {
            Self::Http(conn) => conn.endpoint(),
        }
    }
}

/// HTTP-based connection implementation
#[derive(Clone)]
pub struct HttpConnection {
    /// Capability information
    capability_info: CapabilityInfo,
    /// HTTP client
    client: reqwest::Client,
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
    /// - Network or I/O errors occur
    pub fn new(capability_info: CapabilityInfo) -> Result<Self, NestGateError> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| {
                NestGateError::Internal(Box::new(
                    crate::error::variants::core_errors::InternalErrorDetails {
                        message: format!("Failed to create HTTP client: {e}"),
                        component: "universal_adapter".to_string(),
                        location: Some(format!("{}:{}", file!(), line!())),
                        is_bug: false,
                        context: None,
                    },
                ))
            })?;

        let metadata = ConnectionMetadata {
            connection_type: "http".to_string(),
            endpoint: capability_info.endpoint.clone(),
            created_at: std::time::SystemTime::now(),
            last_health_check: None,
            metadata: capability_info.metadata.clone(),
        };

        Ok(Self {
            capability_info,
            client,
            metadata,
        })
    }
}

impl Connection for HttpConnection {
    fn send_request(&self, request: Request) -> impl Future<Output = Result<Response, NestGateError>> + Send {
        async move {
        debug!(
            "Sending request {} to {}",
            request.id, self.capability_info.endpoint
        );

        let response = self
            .client
            .post(&self.capability_info.endpoint)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                NestGateError::Internal(Box::new(
                    crate::error::variants::core_errors::InternalErrorDetails {
                        message: format!("HTTP request failed: {e}"),
                        component: "universal_adapter".to_string(),
                        location: Some(format!("{}:{}", file!(), line!())),
                        is_bug: false,
                        context: None,
                    },
                ))
            })?;

        if response.status().is_success() {
            let response_data: Response = response.json().await.map_err(|e| {
                NestGateError::Internal(Box::new(
                    crate::error::variants::core_errors::InternalErrorDetails {
                        message: format!("Failed to parse response: {e}"),
                        component: "universal_adapter".to_string(),
                        location: Some(format!("{}:{}", file!(), line!())),
                        is_bug: false,
                        context: None,
                    },
                ))
            })?;

            Ok(response_data)
        } else {
            Err(NestGateError::Internal(Box::new(
                crate::error::variants::core_errors::InternalErrorDetails {
                    message: format!("HTTP request failed with status: {}", response.status()),
                    component: "universal_adapter".to_string(),
                    location: Some(format!("{}:{}", file!(), line!())),
                    is_bug: false,
                    context: None,
                },
            )))
        }
        }
    }

    fn health_check(&self) -> impl Future<Output = Result<HealthStatus, NestGateError>> + Send {
        async move {
        debug!("Health checking {}", self.capability_info.endpoint);

        let health_url = format!("{}/health", self.capability_info.endpoint);

        match self.client.get(&health_url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(HealthStatus::Healthy)
                } else {
                    Ok(HealthStatus::Degraded)
                }
            }
            Err(_) => Ok(HealthStatus::Unhealthy),
        }
        }
    }

    fn get_metadata(&self) -> impl Future<Output = Result<ConnectionMetadata, NestGateError>> + Send {
        async move {
            Ok(self.metadata.clone())
        }
    }

    fn connection_type(&self) -> &str {
        "http"
    }

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
            let connection = self.create_connection(capability).await?;

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
                crate::error::variants::core_errors::InternalErrorDetails {
                    message: format!("Capability {capability_type} not discovered"),
                    component: "universal_adapter".to_string(),
                    location: Some(format!("{}:{}", file!(), line!())),
                    is_bug: false,
                    context: None,
                },
            )))
        }
    }

    /// Create a connection for a capability
    async fn create_connection(
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

        let capability = CapabilityInfo {
            capability_type: "test".to_string(),
            endpoint: "http://localhost:8080".to_string(),
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
}
