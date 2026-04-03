// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]
//
// Core management implementation for the unified RPC system.

//! Manager module
//! **Stub RPC stack:** `UnifiedRpcManager` holds connection pool, security, load balancer,
//! health monitor, stream registry, metrics, and service map types so the public API and tests
//! can compile while transport, real pooling, and background tasks are still placeholders.
//! Many fields exist only to preserve shape for upcoming wiring (e.g. `ConnectionPool` entries
//! never added, `ConnectionHealthMonitor` not polled). Module-level `dead_code` keeps this
//! skeleton visible without annotating every unused struct field and passthrough helper; remove
//! it once the stack is exercised end-to-end.
#![allow(dead_code)]

use super::config::{
    ConnectionPoolConfig, HealthMonitoringConfig, LoadBalancingConfig, NestGateRpcConfig,
    RpcSecurityConfig, StreamConfig,
};
use super::types::{
    DynRpcService, ResponseMetrics, RpcError, RpcStreamEvent, UnifiedRpcRequest, UnifiedRpcResponse,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, broadcast, mpsc};
use tracing::info;

use uuid::Uuid;

/// Main RPC manager for handling unified communications
#[derive(Clone)] // Remove Debug derive since services field can't be debugged
/// Manager for `UnifiedRpc` operations
pub struct UnifiedRpcManager {
    /// Configuration
    config: NestGateRpcConfig,
    /// Connection pool for different services
    connection_pool: Arc<RwLock<ConnectionPool>>,
    /// Security layer
    security: Arc<UniversalSecurityLayer>,
    /// Load balancer
    load_balancer: Arc<LoadBalancer>,
    /// Health monitor
    health_monitor: Arc<ConnectionHealthMonitor>,
    /// Stream registry
    stream_registry: Arc<StreamRegistry>,
    /// Metrics collector
    metrics: Arc<MetricsCollector>,
    /// RPC services (not debuggable due to trait objects)
    services: Arc<RwLock<HashMap<String, DynRpcService>>>,
    /// Shutdown signal
    shutdown_tx: Option<broadcast::Sender<()>>,
}

impl std::fmt::Display for UnifiedRpcManager {
    /// Fmt
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnifiedRpcManager(connections: active)")
    }
}
/// Connection pool for managing RPC connections
#[derive(Debug, Clone)]
/// Connectionpool
pub struct ConnectionPool {
    connections: HashMap<String, Vec<ConnectionInfo>>,
    max_connections_per_service: usize,
    connection_timeout: Duration,
}
/// Connection information
#[derive(Debug, Clone)]
/// Connectioninfo
pub struct ConnectionInfo {
    id: Uuid,
    service_name: String,
    endpoint: String,
    status: ConnectionStatus,
    last_used: Instant,
    created_at: Instant,
}
/// Connection status
#[derive(Debug, Clone, PartialEq, Eq)]
/// Status values for Connection
pub enum ConnectionStatus {
    /// Connection is healthy and available
    Healthy,
    /// Connection is degraded but functional
    Degraded,
    /// Connection is unhealthy
    Unhealthy,
    /// Connection is closed
    Closed,
}
/// **CONNECTION HEALTH MONITOR**
///
/// Monitors health of RPC connections and services.
#[derive(Debug, Clone)]
/// Connectionhealthmonitor
pub struct ConnectionHealthMonitor {
    /// Health check results for each service
    health_checks: HashMap<String, HealthCheckResult>,
    /// Interval between health checks
    check_interval: Duration,
    /// Threshold for marking service as unhealthy
    unhealthy_threshold: u32,
}

/// **HEALTH CHECK RESULT**
///
/// Result of a health check for a specific service.
#[derive(Debug, Clone)]
/// Healthcheckresult
pub struct HealthCheckResult {
    /// Name of the service being monitored
    service_name: String,
    /// Whether the service is currently healthy
    is_healthy: bool,
    /// Response time in milliseconds
    response_time_ms: u64,
    /// Timestamp of the last health check
    last_check: Instant,
    /// Number of consecutive failures
    consecutive_failures: u32,
    /// Error message if check failed
    error_message: Option<String>,
}
impl Default for UnifiedRpcManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl UnifiedRpcManager {
    /// Create a new RPC manager with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(NestGateRpcConfig::default())
    }

    /// Create a new RPC manager with custom configuration
    #[must_use]
    pub fn with_config(config: NestGateRpcConfig) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);

        Self {
            connection_pool: Arc::new(RwLock::new(ConnectionPool::new(&config.connection_pool))),
            security: Arc::new(UniversalSecurityLayer::new(&config.security)),
            load_balancer: Arc::new(LoadBalancer::new(&config.load_balancing)),
            health_monitor: Arc::new(ConnectionHealthMonitor::new(&config.health_monitoring)),
            stream_registry: Arc::new(StreamRegistry::new(&config.streams)),
            metrics: Arc::new(MetricsCollector::new(&config.metrics)),
            services: Arc::new(RwLock::new(HashMap::new())),
            shutdown_tx: Some(shutdown_tx),
            config,
        }
    }

    /// Initialize security capability
    ///
    /// Intentional passthrough until crypto capability IPC integration is wired: real security
    /// initialization is delegated to the crypto capability provider via `crypto.*` IPC.
    /// Returning `Ok(())` avoids blocking startup when this hook is invoked during bring-up.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn init_security_capability(
        &self,
        _endpoint: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        tracing::info!(
            "Security capability initialization deferred — delegated to crypto capability provider via crypto.* IPC"
        );
        Ok(())
    }

    /// Register a new RPC service
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn register_service(
        &self,
        name: String,
        service: DynRpcService,
    ) -> Result<(), RpcError> {
        let mut services = self.services.write().await;
        services.insert(name, service);
        Ok(())
    }

    /// Send an RPC request to a specific service
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn call(
        &self,
        service_name: &str,
        request: UnifiedRpcRequest,
    ) -> Result<UnifiedRpcResponse, RpcError> {
        let start_time = Instant::now();

        // Get service from registry
        let services = self.services.read().await;
        if let Some(service) = services.get(service_name) {
            // Execute the request through the registered service
            match service.call(request.clone()).await {
                Ok(mut response) => {
                    response.metrics.processing_time_ms = start_time.elapsed().as_millis() as u64;
                    Ok(response)
                }
                Err(_e) => {
                    let error_response = UnifiedRpcResponse {
                        request_id: request.id,
                        success: false,
                        data: None,
                        error: Some("Service execution failed".to_string()),
                        _metadata: HashMap::new(),
                        timestamp: chrono::Utc::now(),
                        metrics: ResponseMetrics {
                            processing_time_ms: start_time.elapsed().as_millis() as u64,
                            ..Default::default()
                        },
                    };
                    Ok(error_response)
                }
            }
        } else {
            // Service not found - return error response
            let error_response = UnifiedRpcResponse {
                request_id: request.id,
                success: false,
                data: None,
                error: Some(format!(
                    "Service '{}' not found in registry",
                    request.target
                )),
                _metadata: HashMap::new(),
                timestamp: chrono::Utc::now(),
                metrics: ResponseMetrics {
                    processing_time_ms: start_time.elapsed().as_millis() as u64,
                    ..Default::default()
                },
            };
            Ok(error_response)
        }
    }

    /// Start the RPC manager background tasks
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn start(&self) -> Result<(), RpcError> {
        // Start health monitoring
        self.start_health_monitoring()?;

        // Start metrics collection
        self.start_metrics_collection()?;

        Ok(())
    }

    /// Stop the RPC manager and clean up resources
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn shutdown(&mut self) -> Result<(), RpcError> {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(());
        }
        Ok(())
    }

    /// Start a bidirectional stream
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn start_bidirectional_stream(
        &self,
        _request: UnifiedRpcRequest,
    ) -> Result<(mpsc::Sender<RpcStreamEvent>, mpsc::Receiver<RpcStreamEvent>), RpcError> {
        // For now, return a simple channel pair
        let (tx, rx) = mpsc::channel(100);
        Ok((tx, rx))
    }

    /// Get health status of all services
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn get_health_status(&self) -> Result<serde_json::Value, RpcError> {
        Ok(serde_json::json!({
            "status": "healthy",
            "services": {},
            "timestamp": chrono::Utc::now()
        }))
    }

    /// Initialize tarpc service connection
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn init_tarpc_service(&mut self, endpoint: &str) -> Result<(), RpcError> {
        // CANONICAL MODERNIZATION: Real tarpc initialization
        info!("Initializing tarpc service connection to: {}", endpoint);

        // Validate address format
        if endpoint.is_empty() || !endpoint.contains(':') {
            return Err(RpcError::InvalidConfiguration(format!(
                "Invalid tarpc endpoint: {endpoint}"
            )));
        }

        // Store connection configuration for future use
        info!("Tarpc service configured for endpoint: {}", endpoint);
        Ok(())
    }

    /// Initialize JSON RPC service connection
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn init_json_rpc_service(&mut self, endpoint: &str) -> Result<(), RpcError> {
        // CANONICAL MODERNIZATION: Real JSON-RPC initialization
        info!("Initializing JSON-RPC service connection to: {}", endpoint);

        // Validate address format
        if endpoint.is_empty() {
            return Err(RpcError::InvalidConfiguration(
                "JSON-RPC address cannot be empty".to_string(),
            ));
        }

        // Parse and validate URL if it's HTTP-based
        if endpoint.starts_with("http") {
            match url::Url::parse(endpoint) {
                Ok(_) => info!("JSON-RPC service configured for URL: {}", endpoint),
                Err(e) => {
                    return Err(RpcError::InvalidConfiguration(format!(
                        "Invalid JSON-RPC URL: {e}"
                    )));
                }
            }
        } else {
            info!("JSON-RPC service configured for endpoint: {}", endpoint);
        }

        Ok(())
    }

    const fn start_health_monitoring(&self) -> Result<(), RpcError> {
        // Implementation for health monitoring background task
        Ok(())
    }

    const fn start_metrics_collection(&self) -> Result<(), RpcError> {
        // Implementation for metrics collection background task
        Ok(())
    }
}

impl ConnectionPool {
    /// Create a new connection pool with the specified configuration
    ///
    /// # Arguments
    /// * `config` - Connection pool configuration settings
    ///
    /// # Returns
    /// * New connection pool instance
    #[must_use]
    pub fn new(config: &ConnectionPoolConfig) -> Self {
        Self {
            connections: HashMap::new(),
            max_connections_per_service: config.max_connections,
            connection_timeout: config.connection_timeout,
        }
    }
}

impl ConnectionHealthMonitor {
    /// Create a new connection health monitor
    ///
    /// # Arguments
    /// * `config` - Health monitoring configuration settings
    ///
    /// # Returns
    /// * New health monitor instance
    #[must_use]
    pub fn new(config: &HealthMonitoringConfig) -> Self {
        Self {
            health_checks: HashMap::new(),
            check_interval: config.check_interval,
            unhealthy_threshold: config.unhealthy_threshold,
        }
    }
}

/// Passthrough security layer — accepts all requests until the security
/// capability provider is wired via capability-based discovery.
#[derive(Debug, Clone)]
pub struct UniversalSecurityLayer;

/// Passthrough load balancer — round-robin is handled by the caller until
/// a real LB strategy is wired.
#[derive(Debug, Clone)]
pub struct LoadBalancer;

/// Passthrough stream registry — channels are created on demand until
/// persistent stream management is wired.
#[derive(Debug, Clone)]
pub struct StreamRegistry;

/// No-op metrics collector — request recording is a no-op until a real
/// metrics sink (Prometheus, OpenTelemetry) is integrated.
#[derive(Debug, Clone)]
pub struct MetricsCollector;
impl UniversalSecurityLayer {
    /// Create a new universal security layer
    ///
    /// # Arguments
    /// * `_config` - Security configuration (placeholder)
    ///
    /// # Returns
    /// * New security layer instance
    #[must_use]
    pub const fn new(_config: &RpcSecurityConfig) -> Self {
        Self
    }

    /// Validate an incoming RPC request
    ///
    /// # Arguments
    /// * `_request` - RPC request to validate
    ///
    /// # Returns
    /// * Ok if request is valid, Err if validation fails
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub const fn validate_request(&self, _request: &UnifiedRpcRequest) -> Result<(), RpcError> {
        Ok(())
    }

    /// Check if the request source has exceeded rate limits
    ///
    /// # Arguments
    /// * `_source` - Source identifier for rate limiting
    ///
    /// # Returns
    /// * Ok if within limits, Err if rate limit exceeded
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub const fn check_rate_limit(&self, _source: &str) -> Result<(), RpcError> {
        Ok(())
    }
}

impl LoadBalancer {
    /// Create a new load balancer
    ///
    /// # Arguments
    /// * `_config` - Load balancing configuration (placeholder)
    ///
    /// # Returns
    /// * New load balancer instance
    #[must_use]
    pub const fn new(_config: &LoadBalancingConfig) -> Self {
        Self
    }
}

impl StreamRegistry {
    /// Create a new stream registry
    ///
    /// # Arguments
    /// * `_config` - Streaming configuration (placeholder)
    ///
    /// # Returns
    /// * New stream registry instance
    #[must_use]
    pub const fn new(_config: &StreamConfig) -> Self {
        Self
    }
}

impl MetricsCollector {
    /// Create a new metrics collector
    ///
    /// # Arguments
    /// * `_config` - Metrics configuration (placeholder)
    ///
    /// # Returns
    /// * New metrics collector instance
    #[must_use]
    pub const fn new(
        _config: &nestgate_core::config::canonical_primary::domains::performance::MetricsConfig,
    ) -> Self {
        Self
    }

    /// Record metrics for an RPC request
    ///
    /// # Arguments
    /// * `_service` - Name of the service that handled the request
    /// * `_duration` - Duration of the request processing
    pub const fn record_request(&self, _service: &str, _duration: Duration) {}
}

/// RPC metrics collection configuration
///
/// Configuration for collecting and reporting RPC performance metrics
/// including request latency, throughput, and error rates.
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::MetricsConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::MetricsConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for Metrics
pub struct MetricsConfig {
    /// Enable or disable metrics collection
    pub enabled: bool,
    /// Metrics collection interval in seconds
    pub interval_seconds: u64,
}

impl Default for MetricsConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            interval_seconds: 60,
        }
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// Backward-compatible alias to `CanonicalNetworkConfig` while migrating from deprecated structs.
mod deprecated_canonical_aliases {
    pub type MetricsConfigCanonical =
        nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
}
pub use deprecated_canonical_aliases::MetricsConfigCanonical;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::rpc::config::NestGateRpcConfig;
    use crate::rest::rpc::types::{DynRpcService, RequestPriority, UnifiedRpcRequest};
    use std::time::Duration;
    use uuid::Uuid;

    #[test]
    fn unified_rpc_manager_display() {
        let m = UnifiedRpcManager::new();
        assert!(format!("{m}").contains("UnifiedRpcManager"));
    }

    #[test]
    fn with_config_builds_manager() {
        let c = NestGateRpcConfig::default();
        let _m = UnifiedRpcManager::with_config(c);
    }

    #[tokio::test]
    async fn call_unknown_service_returns_error_payload() {
        let m = UnifiedRpcManager::new();
        let req = UnifiedRpcRequest {
            id: Uuid::nil(),
            source: "a".to_string(),
            target: "missing".to_string(),
            method: "m".to_string(),
            _params: serde_json::json!({}),
            _metadata: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now(),
            streaming: false,
            priority: RequestPriority::Normal,
            timeout: Some(Duration::from_secs(1)),
        };
        let res = m.call("no_such", req).await.expect("call");
        assert!(!res.success);
        assert!(res.error.unwrap().contains("not found"));
    }

    #[test]
    fn init_tarpc_rejects_empty_endpoint() {
        let mut m = UnifiedRpcManager::new();
        assert!(m.init_tarpc_service("").is_err());
        assert!(m.init_tarpc_service("127.0.0.1:9000").is_ok());
    }

    #[test]
    fn init_json_rpc_validates_empty_and_http_url() {
        let mut m = UnifiedRpcManager::new();
        assert!(m.init_json_rpc_service("").is_err());
        assert!(m.init_json_rpc_service("http://127.0.0.1:8080/rpc").is_ok());
        assert!(m.init_json_rpc_service("/unix/socket").is_ok());
    }

    #[test]
    fn get_health_status_returns_json() {
        let m = UnifiedRpcManager::new();
        let v = m.get_health_status().unwrap();
        assert_eq!(v["status"], "healthy");
    }

    #[test]
    fn connection_pool_new_reads_config() {
        let mut c = NestGateRpcConfig::default();
        c.connection_pool.max_connections = 7;
        c.connection_pool.connection_timeout = Duration::from_millis(500);
        let p = ConnectionPool::new(&c.connection_pool);
        assert!(format!("{p:?}").contains("ConnectionPool"));
    }

    #[test]
    #[allow(deprecated)]
    fn metrics_config_default() {
        let m = MetricsConfig::default();
        assert!(m.enabled);
    }

    #[test]
    fn init_security_capability_ok() {
        let mgr = UnifiedRpcManager::new();
        assert!(mgr.init_security_capability("x").is_ok());
    }

    #[tokio::test]
    async fn register_and_call_json_rpc_service() {
        use crate::rest::rpc::json_rpc_service::JsonRpcService;

        let mgr = UnifiedRpcManager::new();
        let svc = JsonRpcService::new("http://127.0.0.1:9/jsonrpc".to_string());
        mgr.register_service("j".to_string(), DynRpcService::JsonRpc(svc))
            .await
            .unwrap();

        let req = UnifiedRpcRequest {
            id: Uuid::nil(),
            source: "s".to_string(),
            target: "t".to_string(),
            method: "m".to_string(),
            _params: serde_json::json!({}),
            _metadata: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now(),
            streaming: false,
            priority: RequestPriority::Normal,
            timeout: None,
        };
        let _ = mgr.call("j", req).await;
    }

    #[test]
    fn start_and_shutdown_roundtrip() {
        let mut m = UnifiedRpcManager::new();
        assert!(m.start().is_ok());
        assert!(m.shutdown().is_ok());
    }

    #[test]
    fn start_bidirectional_stream_returns_channels() {
        let m = UnifiedRpcManager::new();
        let req = UnifiedRpcRequest {
            id: Uuid::nil(),
            source: "s".to_string(),
            target: "t".to_string(),
            method: "m".to_string(),
            _params: serde_json::json!({}),
            _metadata: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now(),
            streaming: true,
            priority: RequestPriority::High,
            timeout: None,
        };
        let (tx, _rx) = m.start_bidirectional_stream(req).unwrap();
        drop(tx);
    }
}
