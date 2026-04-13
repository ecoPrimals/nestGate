// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **DEPRECATED**: Compile-time port fallbacks.
//!
//! Use [`crate::constants::capability_port_discovery::RuntimePortResolver`] or the
//! `discover_*_port` family instead. These constants remain only for backward compatibility
//! during the migration period.

/// Fallback port for plain HTTP API listeners.
#[deprecated(since = "0.4.0", note = "use RuntimePortResolver or discover_api_port")]
pub const HTTP: u16 = 8080;
/// Fallback port for TLS-terminated API listeners.
#[deprecated(since = "0.4.0", note = "use RuntimePortResolver")]
pub const HTTPS: u16 = 8443;
/// Fallback port for the primary REST/JSON-RPC API.
#[deprecated(since = "0.4.0", note = "use RuntimePortResolver or discover_api_port")]
pub const API: u16 = 3000;
/// Fallback alternate API port.
#[deprecated(since = "0.4.0", note = "use RuntimePortResolver")]
pub const API_ALT: u16 = 3001;
/// Fallback port for `Prometheus`-compatible metrics endpoint.
#[deprecated(
    since = "0.4.0",
    note = "use RuntimePortResolver or discover_metrics_port"
)]
pub const METRICS: u16 = 9090;
/// Fallback port for `Prometheus` scrape endpoint (alias of METRICS).
#[deprecated(
    since = "0.4.0",
    note = "use RuntimePortResolver or discover_metrics_port"
)]
pub const PROMETHEUS: u16 = 9090;
/// Fallback port for the health/liveness probe endpoint.
#[deprecated(
    since = "0.4.0",
    note = "use RuntimePortResolver or discover_health_port"
)]
pub const HEALTH: u16 = 8081;
/// Fallback port for gRPC listeners.
#[deprecated(since = "0.4.0", note = "use RuntimePortResolver")]
pub const GRPC: u16 = 50051;
/// Fallback port for `WebSocket` connections.
#[deprecated(since = "0.4.0", note = "use RuntimePortResolver")]
pub const WEBSOCKET: u16 = 8082;
/// Fallback port for the admin/management API.
#[deprecated(
    since = "0.4.0",
    note = "use RuntimePortResolver or discover_admin_port"
)]
pub const ADMIN: u16 = 9000;
/// Fallback port for the storage service endpoint.
#[deprecated(
    since = "0.4.0",
    note = "use RuntimePortResolver or discover_storage_port"
)]
pub const STORAGE: u16 = 5000;
/// Fallback port for the orchestration service.
#[deprecated(since = "0.4.0", note = "use RuntimePortResolver")]
pub const ORCHESTRATION: u16 = 8083;
/// Fallback port for compute service endpoints.
#[deprecated(since = "0.4.0", note = "use RuntimePortResolver")]
pub const COMPUTE: u16 = 8085;
/// Fallback port for extended/auxiliary services.
#[deprecated(since = "0.4.0", note = "use RuntimePortResolver")]
pub const EXTENDED_SERVICES: u16 = 3002;
/// Fallback port for the ecosystem coordination endpoint.
#[deprecated(since = "0.4.0", note = "use RuntimePortResolver")]
pub const ECOSYSTEM: u16 = 6000;
/// Fallback port for the discovery service.
#[deprecated(since = "0.4.0", note = "use RuntimePortResolver")]
pub const DISCOVERY_SERVICE: u16 = 3010;
/// Fallback alternate metrics port.
#[deprecated(
    since = "0.4.0",
    note = "use RuntimePortResolver or discover_metrics_port"
)]
pub const METRICS_ALT: u16 = 9001;
/// Fallback port for `PostgreSQL` (external service, env-resolved).
#[deprecated(
    since = "0.4.0",
    note = "use RuntimePortResolver::resolve_env_or_default"
)]
pub const POSTGRES: u16 = 5432;
/// Fallback port for `Redis` (external service, env-resolved).
#[deprecated(
    since = "0.4.0",
    note = "use RuntimePortResolver::resolve_env_or_default"
)]
pub const REDIS: u16 = 6379;
/// Fallback port for `MongoDB` (external service, env-resolved).
#[deprecated(
    since = "0.4.0",
    note = "use RuntimePortResolver::resolve_env_or_default"
)]
pub const MONGODB: u16 = 27017;
/// Fallback port for `MySQL` (external service, env-resolved).
#[deprecated(
    since = "0.4.0",
    note = "use RuntimePortResolver::resolve_env_or_default"
)]
pub const MYSQL: u16 = 3306;
