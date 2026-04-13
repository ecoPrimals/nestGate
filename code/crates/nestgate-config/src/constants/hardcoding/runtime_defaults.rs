// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Environment-backed [`RuntimeDefaults`] and `NESTGATE_*` accessor functions.
#![expect(deprecated)]

use std::env;
use std::sync::OnceLock;

use super::{addresses, discovery, ports};

/// Cache for bind address from environment
static BIND_ADDRESS: OnceLock<String> = OnceLock::new();

/// Cache for API port from environment
static API_PORT: OnceLock<u16> = OnceLock::new();

/// Environment-first network defaults with deprecated constant fallbacks.
///
/// Use this for bootstrap and tests. In production, **resolve ports and peer URLs via capability
/// discovery**; primal code should only encode self-knowledge, not fixed peers.
pub struct RuntimeDefaults;

impl RuntimeDefaults {
    /// `NESTGATE_BIND_ADDRESS`, else [`addresses::BIND_ALL_IPV4`].
    #[must_use]
    pub fn bind_address() -> &'static str {
        BIND_ADDRESS.get_or_init(|| {
            env::var("NESTGATE_BIND_ADDRESS")
                .unwrap_or_else(|_| addresses::BIND_ALL_IPV4.to_string())
        })
    }

    /// `NESTGATE_API_PORT`, else [`ports::API_DEFAULT`].
    #[must_use]
    pub fn api_port() -> u16 {
        *API_PORT.get_or_init(|| {
            env::var("NESTGATE_API_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(ports::API_DEFAULT)
        })
    }

    /// `NESTGATE_METRICS_PORT`, else [`ports::METRICS_DEFAULT`].
    #[must_use]
    pub fn metrics_port() -> u16 {
        env::var("NESTGATE_METRICS_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(ports::METRICS_DEFAULT)
    }

    /// `NESTGATE_HEALTH_PORT`, else [`ports::HEALTH_CHECK`].
    #[must_use]
    pub fn health_port() -> u16 {
        env::var("NESTGATE_HEALTH_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(ports::HEALTH_CHECK)
    }

    /// `NESTGATE_ORCHESTRATOR_ADDR`, else `localhost`:[`ports::HTTP_DEFAULT`]. See
    /// [`get_orchestrator_fallback_addr`].
    #[deprecated(
        since = "0.4.0",
        note = "use capability discovery (ServicesConfig::resolve_by_capability) instead of hardcoded orchestrator addresses"
    )]
    #[must_use]
    pub fn orchestrator_fallback_addr() -> String {
        match env::var("NESTGATE_ORCHESTRATOR_ADDR") {
            Ok(s) if s.trim().is_empty() => String::new(),
            Ok(s) => s,
            Err(_) => format!("{}:{}", addresses::LOCALHOST_NAME, ports::HTTP_DEFAULT),
        }
    }

    /// `NESTGATE_ORCHESTRATOR_URL` if set; otherwise a URL derived from
    /// [`Self::orchestrator_fallback_addr`].
    #[deprecated(
        since = "0.4.0",
        note = "use capability discovery (ServicesConfig::resolve_by_capability) instead of hardcoded orchestrator URLs"
    )]
    #[must_use]
    pub fn orchestrator_url() -> String {
        if let Some(url) = env::var("NESTGATE_ORCHESTRATOR_URL")
            .ok()
            .filter(|s| !s.trim().is_empty())
        {
            return url.trim().to_string();
        }
        let addr = Self::orchestrator_fallback_addr();
        if addr.is_empty() {
            return format!(
                "http://{}:{}",
                addresses::LOCALHOST_NAME,
                ports::HTTP_DEFAULT
            );
        }
        if addr.starts_with("http://") || addr.starts_with("https://") {
            return addr;
        }
        format!("http://{addr}")
    }

    /// `NESTGATE_WEBSOCKET_PORT`, else [`ports::WEBSOCKET_DEFAULT`].
    #[must_use]
    pub fn websocket_port() -> u16 {
        env::var("NESTGATE_WEBSOCKET_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(ports::WEBSOCKET_DEFAULT)
    }

    /// `NESTGATE_RPC_PORT`, else [`ports::GRPC_DEFAULT`].
    #[must_use]
    pub fn grpc_port() -> u16 {
        env::var("NESTGATE_RPC_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(ports::GRPC_DEFAULT)
    }

    /// `NESTGATE_MQ_PORT`, else [`crate::constants::port_defaults::DEFAULT_RABBITMQ_PORT`].
    #[must_use]
    pub fn message_queue_port() -> u16 {
        env::var("NESTGATE_MQ_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(crate::constants::port_defaults::DEFAULT_RABBITMQ_PORT)
    }

    /// `NESTGATE_ORCHESTRATION_PORT`, else [`ports::ORCHESTRATION_DEFAULT`].
    #[must_use]
    pub fn orchestration_service_port() -> u16 {
        env::var("NESTGATE_ORCHESTRATION_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(ports::ORCHESTRATION_DEFAULT)
    }

    /// `NESTGATE_DISCOVERY_TIMEOUT_MS`, else [`discovery::TIMEOUT_MS`].
    #[must_use]
    pub fn discovery_timeout_ms() -> u64 {
        env::var("NESTGATE_DISCOVERY_TIMEOUT_MS")
            .ok()
            .and_then(|t| t.parse().ok())
            .unwrap_or(discovery::TIMEOUT_MS)
    }

    /// `NESTGATE_ZFS_BIND_PORT`, else [`ports::COMPUTE_DEFAULT`] (legacy ZFS standalone lane).
    #[must_use]
    pub fn zfs_bind_port() -> u16 {
        env::var("NESTGATE_ZFS_BIND_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(ports::COMPUTE_DEFAULT)
    }
}

/// Get the bind address from environment or use default
///
/// Checks `NESTGATE_BIND_ADDRESS` environment variable.
/// Falls back to `0.0.0.0` if not set.
#[must_use]
pub fn get_bind_address() -> &'static str {
    RuntimeDefaults::bind_address()
}

/// Get the API port from environment or use default
///
/// Checks `NESTGATE_API_PORT` environment variable.
/// Falls back to 3000 if not set or invalid.
#[must_use]
pub fn get_api_port() -> u16 {
    RuntimeDefaults::api_port()
}

/// Get the metrics port from environment or use default
///
/// Checks `NESTGATE_METRICS_PORT` environment variable.
/// Falls back to 9090 if not set or invalid.
#[must_use]
pub fn get_metrics_port() -> u16 {
    RuntimeDefaults::metrics_port()
}

/// Get the health check port from environment or use default
///
/// Checks `NESTGATE_HEALTH_PORT` environment variable.
/// Falls back to 8081 if not set or invalid.
#[must_use]
pub fn get_health_port() -> u16 {
    RuntimeDefaults::health_port()
}

/// Fallback orchestrator peer address when capability discovery finds none.
///
/// Checks `NESTGATE_ORCHESTRATOR_ADDR` (host:port, unix path, or `unix://…`).
/// Defaults to `localhost` and [`ports::HTTP_DEFAULT`] when unset.
/// If the variable is set to an empty string (after trim), returns empty — callers treat that as
/// "no configured orchestrator".
#[deprecated(
    since = "0.4.0",
    note = "use capability discovery (ServicesConfig::resolve_by_capability) instead"
)]
#[must_use]
pub fn get_orchestrator_fallback_addr() -> String {
    RuntimeDefaults::orchestrator_fallback_addr()
}

/// WebSocket port from environment or [`ports::WEBSOCKET_DEFAULT`].
#[must_use]
pub fn get_websocket_port() -> u16 {
    RuntimeDefaults::websocket_port()
}

/// gRPC / RPC port from `NESTGATE_RPC_PORT` or [`ports::GRPC_DEFAULT`].
#[must_use]
pub fn get_grpc_port() -> u16 {
    RuntimeDefaults::grpc_port()
}

/// Message queue (e.g. `RabbitMQ`) port from `NESTGATE_MQ_PORT` or crate default.
#[must_use]
pub fn get_message_queue_port() -> u16 {
    RuntimeDefaults::message_queue_port()
}

/// Standalone orchestration service port from `NESTGATE_ORCHESTRATION_PORT` or default.
#[must_use]
pub fn get_orchestration_service_port() -> u16 {
    RuntimeDefaults::orchestration_service_port()
}

/// ZFS primal bind port from `NESTGATE_ZFS_BIND_PORT` or [`ports::COMPUTE_DEFAULT`].
#[must_use]
pub fn get_zfs_bind_port() -> u16 {
    RuntimeDefaults::zfs_bind_port()
}
