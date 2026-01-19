//! **TRANSPORT SERVER**
//!
//! Main server implementation for Unix socket + optional HTTP transport.

use super::{
    config::TransportConfig,
    jsonrpc::{JsonRpcHandler, RpcMethodHandler},
    unix_socket::UnixSocketListener,
};
use nestgate_core::error::{NestGateError, Result};
use std::sync::Arc;
use tokio::sync::Notify;
use tracing::{error, info, warn};

/// **TRANSPORT SERVER**
///
/// Dual-mode server supporting Unix sockets (primary) and HTTP (optional fallback).
#[derive(Clone)]
pub struct TransportServer<H> {
    config: TransportConfig,
    handler: Arc<H>,
    shutdown: Arc<Notify>,
}

impl<H> TransportServer<H>
where
    H: RpcMethodHandler + Send + Sync + 'static,
{
    /// Create new transport server
    ///
    /// # Errors
    ///
    /// Returns error if configuration is invalid
    pub fn new(config: TransportConfig, handler: H) -> Result<Self> {
        config.validate()?;

        Ok(Self {
            config,
            handler: Arc::new(handler),
            shutdown: Arc::new(Notify::new()),
        })
    }

    /// Start the server
    ///
    /// # Errors
    ///
    /// Returns error if server startup fails
    pub async fn start(&self) -> Result<()> {
        info!("🚀 Starting NestGate Transport Server");
        info!("   Family ID: {}", self.config.family_id);
        info!("   Socket: {}", self.config.socket_path.display());

        if let Some(port) = self.config.http_port {
            warn!("⚠️  HTTP fallback enabled on port {}", port);
            warn!("   This is for debugging only - production should use Unix sockets");
        }

        // Start Unix socket listener
        let unix_handle = self.start_unix_socket().await?;

        // Start HTTP fallback if configured
        let http_handle = if self.config.http_port.is_some() {
            Some(self.start_http_fallback().await?)
        } else {
            None
        };

        // Wait for shutdown signal
        self.shutdown.notified().await;

        info!("📡 Shutting down gracefully...");

        // Wait for tasks to complete
        let _ = tokio::join!(unix_handle);
        if let Some(handle) = http_handle {
            let _ = handle.await;
        }

        info!("✅ NestGate Transport Server stopped");
        Ok(())
    }

    /// Start Unix socket listener
    async fn start_unix_socket(&self) -> Result<tokio::task::JoinHandle<()>> {
        let mut listener = UnixSocketListener::new(&self.config.socket_path)?;
        listener.bind()?;

        let handler = Arc::clone(&self.handler);
        let shutdown = Arc::clone(&self.shutdown);

        let handle = tokio::spawn(async move {
            let jsonrpc = JsonRpcHandler {
                handler: Arc::clone(&handler),
            };

            loop {
                tokio::select! {
                    _ = shutdown.notified() => {
                        info!("Unix socket listener received shutdown signal");
                        break;
                    }
                    result = listener.accept() => {
                        match result {
                            Ok(stream) => {
                                let jsonrpc = jsonrpc.clone();
                                tokio::spawn(async move {
                                    if let Err(e) = jsonrpc.handle_connection(stream).await {
                                        error!("Connection handler error: {}", e);
                                    }
                                });
                            }
                            Err(e) => {
                                error!("Failed to accept connection: {}", e);
                            }
                        }
                    }
                }
            }
        });

        Ok(handle)
    }

    /// Start HTTP fallback server
    async fn start_http_fallback(&self) -> Result<tokio::task::JoinHandle<()>> {
        let port = self
            .config
            .http_port
            .ok_or_else(|| NestGateError::api_error("HTTP port not configured"))?;

        let shutdown = Arc::clone(&self.shutdown);

        let handle = tokio::spawn(async move {
            // TODO: Implement HTTP fallback in Phase 4
            warn!("HTTP fallback not yet implemented - use Unix sockets");
            shutdown.notified().await;
        });

        info!("📡 HTTP fallback placeholder started on port {}", port);

        Ok(handle)
    }

    /// Signal graceful shutdown
    pub fn shutdown(&self) {
        info!("Shutdown signal received");
        self.shutdown.notify_waiters();
    }

    /// Get server configuration
    #[must_use]
    pub fn config(&self) -> &TransportConfig {
        &self.config
    }
}

// Clone implementation for JsonRpcHandler to support spawning
impl<H> Clone for JsonRpcHandler<H> {
    fn clone(&self) -> Self {
        Self {
            handler: Arc::clone(&self.handler),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    struct TestHandler;

    #[async_trait::async_trait]
    impl RpcMethodHandler for TestHandler {
        async fn handle_method(&self, method: &str, _params: Value) -> Result<Value> {
            match method {
                "test.ping" => Ok(Value::String("pong".to_string())),
                _ => Err(NestGateError::api_error("Unknown method")),
            }
        }
    }

    #[test]
    fn test_server_creation() {
        let config = TransportConfig::new("test");
        let handler = TestHandler;
        let server = TransportServer::new(config, handler);
        assert!(server.is_ok());
    }

    #[test]
    fn test_server_config() {
        let config = TransportConfig::new("test");
        let handler = TestHandler;
        let server = TransportServer::new(config, handler).unwrap();
        assert_eq!(server.config().family_id, "test");
    }
}
