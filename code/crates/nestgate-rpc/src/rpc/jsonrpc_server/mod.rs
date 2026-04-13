// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! JSON-RPC 2.0 Server for `NestGate` Storage (built on jsonrpsee)
//!
//! Provides universal, language-agnostic RPC access to `NestGate` storage
//! capabilities over HTTP. Built on the `jsonrpsee` library — this module adds
//! NestGate-specific method registration, not a custom transport.
//!
//! **Note**: The standalone `JsonRpcServer::start()` binds its own TCP port.
//! Production HTTP JSON-RPC currently routes through the Axum `POST /jsonrpc`
//! handler in `nestgate-api` (`NestGateJsonRpcHandler`). This module is
//! available for dedicated-port deployments or future consolidation.
//!
//! ## Philosophy
//!
//! - **Universal Access**: JSON-RPC works with ANY language
//! - **Self-Knowledge**: Exposes only storage capabilities
//! - **Runtime Discovery**: Capability-based service finding
//! - **Same Operations**: 18 operations (same as tarpc)
//!
//! ## Supported Methods (wateringHole `domain.operation` standard)
//!
//! Storage Dataset Operations (4):
//! - `storage.dataset.create` - Create a new dataset
//! - `storage.dataset.list` - List all datasets
//! - `storage.dataset.get` - Get dataset info
//! - `storage.dataset.delete` - Delete dataset
//!
//! Storage Object Operations (5):
//! - `storage.object.store` - Store object in dataset
//! - `storage.object.retrieve` - Retrieve object data
//! - `storage.object.metadata` - Get object metadata
//! - `storage.object.list` - List objects in dataset
//! - `storage.object.delete` - Delete object
//!
//! Discovery Operations (2):
//! - `discovery.capability.register` - Register service capability
//! - `discovery.capability.query` - Discover services by capability
//!
//! Health Operations (6):
//! - `health.check` - Service health status
//! - `health.liveness` - Process alive (minimal probe)
//! - `health.readiness` - Ready to serve traffic
//! - `health.metrics` - Storage metrics
//! - `health.info` - Service version and build metadata
//! - `health.protocols` - Supported protocols
//!
//! Capabilities (1):
//! - `capabilities.list` - Supported JSON-RPC method names

use std::net::SocketAddr;

use jsonrpsee::{
    RpcModule,
    server::{Server, ServerHandle},
};
use tracing::info;

use nestgate_types::NestGateError;

use super::storage_backend::StorageBackend;
use super::tarpc_server::NestGateRpcService;

mod capability_methods;
mod monitoring_methods;
mod storage_dataset_methods;
mod storage_methods;
mod storage_object_methods;

/// Maps jsonrpsee method registration failures (`RegisterMethodError`) into [`NestGateError`].
/// jsonrpsee does not use our error type for `RpcModule::register_*`; we normalize at this boundary.
#[inline]
pub(super) fn map_jsonrpc_registration<T>(
    result: std::result::Result<T, impl std::fmt::Display>,
) -> std::result::Result<T, NestGateError> {
    result.map_err(|e| NestGateError::internal(format!("JSON-RPC registration: {e}")))
}

/// JSON-RPC server configuration
#[derive(Debug, Clone)]
pub struct JsonRpcConfig {
    /// Bind address
    pub addr: SocketAddr,
    /// Enable request logging
    pub log_requests: bool,
    /// Maximum request size (bytes)
    pub max_request_size: u32,
    /// Maximum response size (bytes)
    pub max_response_size: u32,
}

impl Default for JsonRpcConfig {
    fn default() -> Self {
        use std::net::{IpAddr, Ipv6Addr};
        Self {
            addr: SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 8092),
            log_requests: true,
            max_request_size: 100 * 1024 * 1024, // 100 MB for large objects
            max_response_size: 100 * 1024 * 1024, // 100 MB
        }
    }
}

/// Shared state for JSON-RPC methods
pub struct JsonRpcState<S: StorageBackend = crate::rpc::storage_backend::InMemoryStorageBackend> {
    /// RPC service instance
    pub service: NestGateRpcService<S>,
    /// Server start time for uptime tracking
    pub start_time: std::time::Instant,
}

impl<S: StorageBackend> Clone for JsonRpcState<S> {
    fn clone(&self) -> Self {
        Self {
            service: self.service.clone(),
            start_time: self.start_time,
        }
    }
}

/// JSON-RPC 2.0 server for `NestGate` storage
pub struct JsonRpcServer<S: StorageBackend = crate::rpc::storage_backend::InMemoryStorageBackend> {
    config: JsonRpcConfig,
    state: JsonRpcState<S>,
}

impl<S: StorageBackend + 'static> JsonRpcServer<S> {
    /// Create a new JSON-RPC server
    #[must_use]
    pub fn new(service: NestGateRpcService<S>, config: JsonRpcConfig) -> Self {
        Self {
            config,
            state: JsonRpcState {
                service,
                start_time: std::time::Instant::now(),
            },
        }
    }

    /// Build RPC module with all methods registered (used by `start()` and tests)
    pub(crate) fn build_module(
        state: JsonRpcState<S>,
    ) -> Result<RpcModule<JsonRpcState<S>>, NestGateError> {
        let mut module = RpcModule::new(state);
        storage_methods::register_storage_methods(&mut module)?;
        capability_methods::register_capability_methods(&mut module)?;
        monitoring_methods::register_monitoring_methods(&mut module)?;
        Ok(module)
    }

    /// Build and start the JSON-RPC server
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP server fails to bind, the listening address cannot be read,
    /// or JSON-RPC method registration fails.
    pub async fn start(self) -> Result<(ServerHandle, SocketAddr), NestGateError> {
        info!(
            "Starting NestGate JSON-RPC 2.0 server on {}",
            self.config.addr
        );

        // Build server
        let server = Server::builder().build(self.config.addr).await?;

        let addr = server.local_addr()?;

        // Create RPC module with all methods registered
        let module = Self::build_module(self.state)?;

        // Start server
        let handle = server.start(module);

        info!("NestGate JSON-RPC 2.0 server listening on {}", addr);
        info!("   Endpoint: http://{}/jsonrpc", addr);
        info!("   Methods: 18 registered");
        info!("   Protocol: Primary=tarpc, Secondary=JSON-RPC");

        Ok((handle, addr))
    }
}

#[cfg(test)]
mod tests;
