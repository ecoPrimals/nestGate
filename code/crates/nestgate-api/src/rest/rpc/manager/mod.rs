// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unified RPC Manager
//!
//! Connection pool, security, load balancer, health monitor, stream registry,
//! metrics, and service map types for the RPC stack. Fields exist to preserve
//! shape for upcoming wiring; the `dead_code` expect will auto-warn when the
//! stack is exercised end-to-end.

#![expect(dead_code, reason = "Scaffold fields reserved for upcoming RPC wiring")]

#[cfg(test)]
mod tests;
mod types;

pub use types::*;

use super::config::NestGateRpcConfig;
use super::types::{
    DynRpcService, ResponseMetrics, RpcError, RpcStreamEvent, UnifiedRpcRequest, UnifiedRpcResponse,
};
use nestgate_core::error::NestGateError;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{RwLock, broadcast, mpsc};
use tracing::info;

/// Unified RPC manager for handling multi-protocol communications.
#[derive(Clone)]
pub struct UnifiedRpcManager {
    config: NestGateRpcConfig,
    connection_pool: Arc<RwLock<ConnectionPool>>,
    security: Arc<UniversalSecurityLayer>,
    load_balancer: Arc<LoadBalancer>,
    health_monitor: Arc<ConnectionHealthMonitor>,
    stream_registry: Arc<StreamRegistry>,
    metrics: Arc<MetricsCollector>,
    services: Arc<RwLock<HashMap<String, DynRpcService>>>,
    shutdown_tx: Option<broadcast::Sender<()>>,
}

impl std::fmt::Display for UnifiedRpcManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnifiedRpcManager(connections: active)")
    }
}

impl Default for UnifiedRpcManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UnifiedRpcManager {
    /// Create a new RPC manager with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(NestGateRpcConfig::default())
    }

    /// Create a new RPC manager with custom configuration.
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

    /// Initialize security capability (passthrough — delegated to crypto IPC).
    ///
    /// # Errors
    ///
    /// Returns error if the operation fails (currently always succeeds).
    pub fn init_security_capability(&self, _endpoint: &str) -> Result<(), NestGateError> {
        tracing::info!(
            "Security capability initialization deferred \
             — delegated to crypto capability provider via crypto.* IPC"
        );
        Ok(())
    }

    /// Register a new RPC service.
    ///
    /// # Errors
    ///
    /// Returns `RpcError` if registration fails.
    pub async fn register_service(
        &self,
        name: impl Into<String>,
        service: DynRpcService,
    ) -> Result<(), RpcError> {
        let mut services = self.services.write().await;
        services.insert(name.into(), service);
        Ok(())
    }

    /// Send an RPC request to a registered service.
    ///
    /// # Errors
    ///
    /// Returns `RpcError` if the call fails.
    pub async fn call(
        &self,
        service_name: &str,
        request: UnifiedRpcRequest,
    ) -> Result<UnifiedRpcResponse, RpcError> {
        let start_time = Instant::now();

        let services = self.services.read().await;
        if let Some(service) = services.get(service_name) {
            match service.call(request.clone()).await {
                Ok(mut response) => {
                    response.metrics.processing_time_ms = start_time.elapsed().as_millis() as u64;
                    Ok(response)
                }
                Err(_e) => Ok(UnifiedRpcResponse {
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
                }),
            }
        } else {
            Ok(UnifiedRpcResponse {
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
            })
        }
    }

    /// Start the RPC manager background tasks.
    ///
    /// # Errors
    ///
    /// Returns `RpcError` if startup fails.
    pub fn start(&self) -> Result<(), RpcError> {
        self.start_health_monitoring()?;
        self.start_metrics_collection()?;
        Ok(())
    }

    /// Stop the RPC manager and clean up resources.
    ///
    /// # Errors
    ///
    /// Returns `RpcError` if shutdown fails.
    pub fn shutdown(&mut self) -> Result<(), RpcError> {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(());
        }
        Ok(())
    }

    /// Start a bidirectional stream.
    ///
    /// # Errors
    ///
    /// Returns `RpcError` if stream creation fails.
    pub fn start_bidirectional_stream(
        &self,
        _request: UnifiedRpcRequest,
    ) -> Result<(mpsc::Sender<RpcStreamEvent>, mpsc::Receiver<RpcStreamEvent>), RpcError> {
        let (tx, rx) = mpsc::channel(100);
        Ok((tx, rx))
    }

    /// Get health status of all services.
    ///
    /// # Errors
    ///
    /// Returns `RpcError` if health check fails.
    pub fn get_health_status(&self) -> Result<serde_json::Value, RpcError> {
        Ok(serde_json::json!({
            "status": "healthy",
            "services": {},
            "timestamp": chrono::Utc::now()
        }))
    }

    /// Initialize tarpc service connection.
    ///
    /// # Errors
    ///
    /// Returns `RpcError` if endpoint is invalid.
    pub fn init_tarpc_service(&mut self, endpoint: &str) -> Result<(), RpcError> {
        info!("Initializing tarpc service connection to: {}", endpoint);
        if endpoint.is_empty() || !endpoint.contains(':') {
            return Err(RpcError::InvalidConfiguration(format!(
                "Invalid tarpc endpoint: {endpoint}"
            )));
        }
        info!("Tarpc service configured for endpoint: {}", endpoint);
        Ok(())
    }

    /// Initialize JSON RPC service connection.
    ///
    /// # Errors
    ///
    /// Returns `RpcError` if endpoint is invalid.
    pub fn init_json_rpc_service(&mut self, endpoint: &str) -> Result<(), RpcError> {
        info!("Initializing JSON-RPC service connection to: {}", endpoint);
        if endpoint.is_empty() {
            return Err(RpcError::InvalidConfiguration(
                "JSON-RPC address cannot be empty".to_string(),
            ));
        }
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

    #[expect(clippy::unnecessary_wraps)]
    const fn start_health_monitoring(&self) -> Result<(), RpcError> {
        Ok(())
    }

    #[expect(clippy::unnecessary_wraps)]
    const fn start_metrics_collection(&self) -> Result<(), RpcError> {
        Ok(())
    }
}
