// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **TRUE PRIMAL TRANSPORT LAYER**
//!
//! Unix socket + JSON-RPC 2.0 transport for TRUE PRIMAL architecture.
//!
//! ## Architecture
//!
//! This module implements the TRUE PRIMAL transport pattern:
//! - **Primary**: Unix sockets (port-free, 100x faster than HTTP)
//! - **Secondary**: HTTP/REST (optional fallback for debugging)
//! - **Protocol**: JSON-RPC 2.0 (universal, simple, compatible)
//! - **Security**: Capability-based security provider (hardware-backed, sovereign)
//!
//! ## Key Principles
//!
//! 1. **Primal Self-Knowledge**: Only knows NestGate identity
//! 2. **Runtime Discovery**: Discovers security provider via capability scan
//! 3. **Capability-Based**: No hardcoded endpoints or ports
//! 4. **Agnostic**: Works with any security provider
//!
//! ## Usage
//!
//! ```rust,ignore
//! use nestgate_api::transport::{TransportServer, TransportConfig};
//!
//! // Primary mode: Unix socket only
//! let config = TransportConfig::from_env()?;
//! let server = TransportServer::new(config)?;
//! server.start().await?;
//!
//! // Dual mode: Unix socket + HTTP fallback
//! let config = TransportConfig::from_env()?.with_http_fallback(8080);
//! let server = TransportServer::new(config)?;
//! server.start().await?;
//! ```

pub mod config;
pub mod handlers;
pub mod jsonrpc;
pub mod security;
pub mod server;
pub mod unix_socket;

pub use config::TransportConfig;
pub use handlers::{NestGateRpcHandler, StorageBackend};
pub use jsonrpc::{
    JsonRpcError, JsonRpcHandler, JsonRpcRequest, JsonRpcResponse, RpcMethodHandler,
};
pub use security::SecurityProviderClient;
pub use server::TransportServer;
pub use unix_socket::UnixSocketListener;
