// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! ⚠️ **DEPRECATED**: This module is being phased out in favor of capability-based configuration
//!
//! # Primal model: self-knowledge vs peers
//!
//! Primal code carries **self-knowledge** only: this process’s identity, capabilities, and own
//! listen endpoints. **Other primals** (orchestrator, storage peers, etc.) are **not** baked in at
//! compile time; their host/port (or URL) must come from **capability discovery at runtime**
//! (service registry, mDNS, mesh, etc.). The numeric port constants in [`ports`] exist solely as
//! **fallback defaults** for bootstrap, tests, and legacy paths—prefer `RuntimeDefaults` (env then
//! fallback) or your discovery layer in production.
//!
//! # Migration Path
//!
//! Instead of using hardcoded constants, use `CapabilityConfig` for runtime discovery:
//!
//! ```rust,ignore
//! # use nestgate_core::capability_config::CapabilityConfig;
//! # use anyhow::Result;
//! # fn example() -> Result<()> {
//! // ❌ OLD: Hardcoded
//! // const API_PORT: u16 = 8080;
//!
//! // ✅ NEW: Capability-based
//! let config = CapabilityConfig::from_env()?;
//! let api_endpoint = config.get_endpoint("api")?;
//! # Ok(())
//! # }
//! ```
//!
//! See `PHASE2_HARDCODING_ELIMINATION_PLAN.md` for full migration guide.
//!
//! This module will be removed in v0.3.0.
//!
//! ## Environment variables (central audit)
//!
//! | Variable | Purpose |
//! |----------|---------|
//! | `NESTGATE_BIND_ADDRESS`, `NESTGATE_API_PORT`, `NESTGATE_METRICS_PORT`, `NESTGATE_HEALTH_PORT` | Core listen ports |
//! | `NESTGATE_ORCHESTRATOR_URL` | Full orchestrator base URL when not using discovery (see `RuntimeDefaults::orchestrator_url`) |
//! | `NESTGATE_ORCHESTRATOR_ADDR` | Orchestrator peer when discovery is empty (see [`crate::constants::hardcoding::get_orchestrator_fallback_addr`]) |
//! | `NESTGATE_WEBSOCKET_PORT`, `NESTGATE_RPC_PORT`, `NESTGATE_MQ_PORT`, `NESTGATE_ORCHESTRATION_PORT` | Service ports (see getters below) |
//! | `NESTGATE_DISCOVERY_TIMEOUT_MS` | Discovery timeout ([`crate::constants::hardcoding::discovery::get_timeout_ms`]) |
//!
//! **Deprecated [`ports`] constants → env overrides (document every fallback; wire via config/discovery in production):**
//!
//! | Constant | `NESTGATE_*` override |
//! |----------|------------------------|
//! | `HTTP_DEFAULT` | `NESTGATE_HTTP_PORT` (also influences orchestrator fallback with `NESTGATE_ORCHESTRATOR_ADDR` / `NESTGATE_ORCHESTRATOR_URL`) |
//! | `HTTPS_DEFAULT` | `NESTGATE_HTTPS_PORT` |
//! | `API_DEFAULT`, `API_ALT` | `NESTGATE_API_PORT` (see `RuntimeDefaults::api_port`); alt: `NESTGATE_API_ALT_PORT` |
//! | `METRICS_DEFAULT`, `PROMETHEUS`, `METRICS_ALT`, `METRICS_PROMETHEUS` | `NESTGATE_METRICS_PORT` |
//! | `HEALTH_CHECK`, `HEALTH_DEFAULT`, `SECURITY_SERVICE_DEFAULT` | `NESTGATE_HEALTH_PORT` |
//! | `GRPC_DEFAULT` | `NESTGATE_RPC_PORT` (see `RuntimeDefaults::grpc_port`) |
//! | `WEBSOCKET_DEFAULT`, `NETWORKING_SERVICE_DEFAULT` | `NESTGATE_WEBSOCKET_PORT` |
//! | `ADMIN_DEFAULT` | `NESTGATE_ADMIN_PORT` |
//! | `STORAGE_DEFAULT` | `NESTGATE_STORAGE_PORT` |
//! | `ORCHESTRATION_DEFAULT` | `NESTGATE_ORCHESTRATION_PORT` |
//! | `STORAGE_DISCOVERY_DEFAULT` | `NESTGATE_STORAGE_DISCOVERY_PORT` |
//! | `COMPUTE_DEFAULT` | `NESTGATE_COMPUTE_PORT` |
//! | `EXTENDED_SERVICES` | `NESTGATE_EXTENDED_SERVICES_PORT` |
//! | `DISCOVERY_SERVICE` | `NESTGATE_DISCOVERY_SERVICE_PORT` |
//! | `ORCHESTRATOR_DEFAULT` | `NESTGATE_ORCHESTRATOR_PORT` |
//! | `STREAMING_RPC_DEFAULT` | `NESTGATE_STREAMING_RPC_PORT` |
//! | `POSTGRES_DEFAULT` | `NESTGATE_POSTGRES_PORT` |
//! | `REDIS_DEFAULT` | `NESTGATE_REDIS_PORT` |
//! | `MONGODB_DEFAULT` | `NESTGATE_MONGODB_PORT` |
//! | `MYSQL_DEFAULT` | `NESTGATE_MYSQL_PORT` |
//! | `discovery::SCAN_PORT_START` / `SCAN_PORT_END` | `NESTGATE_DISCOVERY_SCAN_PORT_START` / `NESTGATE_DISCOVERY_SCAN_PORT_END` |
//! | `timeouts::*`, `limits::*` | `NESTGATE_CONNECT_TIMEOUT_MS`, `NESTGATE_REQUEST_TIMEOUT_MS`, `NESTGATE_LONG_OPERATION_TIMEOUT_MS`, `NESTGATE_BUFFER_SIZE`, `NESTGATE_MAX_CONNECTIONS` (when wired) |
//!
//! Timeouts and limits in [`crate::constants::hardcoding::timeouts`] and [`crate::constants::hardcoding::limits`] remain compile-time defaults; override via
//! capability config or future env wiring where those domains expose runtime tuning.

pub mod addresses;
pub mod discovery;
pub mod limits;
pub mod ports;
pub mod runtime_defaults;
pub mod runtime_fallback_ports;
pub mod timeouts;

pub use runtime_defaults::{
    RuntimeDefaults, get_api_port, get_bind_address, get_grpc_port, get_health_port,
    get_message_queue_port, get_metrics_port, get_orchestration_service_port,
    get_orchestrator_fallback_addr, get_orchestrator_url, get_websocket_port, get_zfs_bind_port,
};

#[cfg(test)]
#[allow(deprecated)]
mod tests;
