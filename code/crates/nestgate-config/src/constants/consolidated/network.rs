// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Network hosts, ports, and bind addresses (environment-driven).
//!
//! # Configuration hierarchy
//!
//! Resolution order for effective network settings:
//!
//! 1. **Environment variables** (`NESTGATE_API_PORT`, `NESTGATE_BIND_ADDRESS`, etc.) — always win when set.
//! 2. **Capability config** — next (runtime discovery and advertised endpoints).
//! 3. **Numeric / string defaults in this module** — last resort for local development only; production should set env or capability config.
//!
//! Primal code should not assume fixed ports on other nodes; peers are discovered at runtime.

use std::sync::{Arc, OnceLock};

use super::defaults::{env_or_from_source, env_or_parse_from_source};
use nestgate_types::{EnvSource, ProcessEnv};

/// Network configuration constants with environment override support
#[derive(Debug, Clone)]
/// Networkconstants
pub struct NetworkConstants {
    // Hosts
    api_host: String,
    metrics_host: String,
    health_host: String,
    admin_host: String,

    // Ports
    api_port: u16,
    http_port: u16,
    https_port: u16,
    websocket_port: u16,
    grpc_port: u16,
    metrics_port: u16,
    prometheus_port: u16,
    health_port: u16,
    admin_port: u16,

    // Addresses
    bind_address: String,
    localhost_ipv4: String,
    localhost_ipv6: String,
    bind_all_ipv4: String,
    bind_all_ipv6: String,
}

impl Default for NetworkConstants {
    /// Returns the default instance.
    ///
    /// See the module-level documentation for the env → capability → default hierarchy.
    fn default() -> Self {
        Self::from_env_source(&ProcessEnv)
    }
}

impl NetworkConstants {
    /// Build network constants from an injectable environment source (use [`MapEnv`](nestgate_types::MapEnv) in tests).
    #[must_use]
    pub fn from_env_source(env: &dyn EnvSource) -> Self {
        Self {
            // Hosts (default to localhost for security)
            api_host: env_or_from_source(env, "NESTGATE_API_HOST", "127.0.0.1"),
            metrics_host: env_or_from_source(env, "NESTGATE_METRICS_HOST", "127.0.0.1"),
            health_host: env_or_from_source(env, "NESTGATE_HEALTH_HOST", "127.0.0.1"),
            admin_host: env_or_from_source(env, "NESTGATE_ADMIN_HOST", "127.0.0.1"),

            // Ports (defaults: development last resort; see module docs)
            api_port: env_or_parse_from_source(env, "NESTGATE_API_PORT", 8080),
            http_port: env_or_parse_from_source(env, "NESTGATE_HTTP_PORT", 8080),
            https_port: env_or_parse_from_source(env, "NESTGATE_HTTPS_PORT", 8443),
            websocket_port: env_or_parse_from_source(env, "NESTGATE_WS_PORT", 8082),
            grpc_port: env_or_parse_from_source(env, "NESTGATE_GRPC_PORT", 50051),
            metrics_port: env_or_parse_from_source(env, "NESTGATE_METRICS_PORT", 9090),
            prometheus_port: env_or_parse_from_source(env, "NESTGATE_PROMETHEUS_PORT", 9090),
            health_port: env_or_parse_from_source(env, "NESTGATE_HEALTH_PORT", 8081),
            admin_port: env_or_parse_from_source(env, "NESTGATE_ADMIN_PORT", 9000),

            // Addresses (bind to loopback by default; expose publicly via env/config)
            bind_address: env_or_from_source(env, "NESTGATE_BIND_ADDRESS", "127.0.0.1"),
            localhost_ipv4: "127.0.0.1".to_string(),
            localhost_ipv6: "::1".to_string(),
            bind_all_ipv4: "0.0.0.0".to_string(),
            bind_all_ipv6: "::".to_string(),
        }
    }

    /// Get or initialize the global network constants
    pub fn get() -> Arc<Self> {
        static INSTANCE: OnceLock<Arc<NetworkConstants>> = OnceLock::new();
        INSTANCE.get_or_init(|| Arc::new(Self::default())).clone()
    }

    // Host getters

    /// Returns the API host address (e.g., "localhost" or "0.0.0.0")
    #[must_use]
    pub fn api_host(&self) -> &str {
        &self.api_host
    }
    /// Metrics Host
    #[must_use]
    pub fn metrics_host(&self) -> &str {
        &self.metrics_host
    }
    /// Health Host
    #[must_use]
    pub fn health_host(&self) -> &str {
        &self.health_host
    }
    /// Admin Host
    #[must_use]
    pub fn admin_host(&self) -> &str {
        &self.admin_host
    }

    // Port getters

    /// Returns the API port number (read from `NESTGATE_API_PORT` or default 8080)
    #[must_use]
    pub const fn api_port(&self) -> u16 {
        self.api_port
    }
    /// Http Port
    #[must_use]
    pub const fn http_port(&self) -> u16 {
        self.http_port
    }
    /// Https Port
    #[must_use]
    pub const fn https_port(&self) -> u16 {
        self.https_port
    }
    /// Websocket Port
    #[must_use]
    pub const fn websocket_port(&self) -> u16 {
        self.websocket_port
    }
    /// Grpc Port
    #[must_use]
    pub const fn grpc_port(&self) -> u16 {
        self.grpc_port
    }
    /// Metrics Port
    #[must_use]
    pub const fn metrics_port(&self) -> u16 {
        self.metrics_port
    }
    /// Prometheus Port
    #[must_use]
    pub const fn prometheus_port(&self) -> u16 {
        self.prometheus_port
    }
    /// Health Port
    #[must_use]
    pub const fn health_port(&self) -> u16 {
        self.health_port
    }
    /// Admin Port
    #[must_use]
    pub const fn admin_port(&self) -> u16 {
        self.admin_port
    }

    // Address getters

    /// Returns the bind address for server sockets (read from `NESTGATE_BIND_ADDRESS` or default `127.0.0.1`)
    #[must_use]
    pub fn bind_address(&self) -> &str {
        &self.bind_address
    }
    /// Localhost Ipv4
    #[must_use]
    pub fn localhost_ipv4(&self) -> &str {
        &self.localhost_ipv4
    }
    /// Localhost Ipv6
    #[must_use]
    pub fn localhost_ipv6(&self) -> &str {
        &self.localhost_ipv6
    }
    /// Bind All Ipv4
    #[must_use]
    pub fn bind_all_ipv4(&self) -> &str {
        &self.bind_all_ipv4
    }
    /// Bind All Ipv6
    #[must_use]
    pub fn bind_all_ipv6(&self) -> &str {
        &self.bind_all_ipv6
    }

    // Convenience methods for full URLs

    /// Returns the full API URL (e.g., "<http://localhost:8080>")
    #[must_use]
    pub fn api_url(&self) -> String {
        format!("http://{}:{}", self.api_host, self.api_port)
    }

    /// Api Bind Address
    #[must_use]
    pub fn api_bind_address(&self) -> String {
        format!("{}:{}", self.bind_address, self.api_port)
    }

    /// Health Url
    #[must_use]
    pub fn health_url(&self) -> String {
        format!("http://{}:{}", self.health_host, self.health_port)
    }

    /// Metrics Url
    #[must_use]
    pub fn metrics_url(&self) -> String {
        format!("http://{}:{}", self.metrics_host, self.metrics_port)
    }

    /// Websocket Url
    #[must_use]
    pub fn websocket_url(&self) -> String {
        format!("ws://{}:{}/ws", self.api_host, self.websocket_port)
    }
}
