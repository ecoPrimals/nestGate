// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Runtime port fallbacks for bootstrap, tests, and legacy paths.
//!
//! Prefer [`crate::constants::capability_port_discovery::RuntimePortResolver`] or the
//! `discover_*_port` helpers for production; these values are last-resort defaults when env and
//! discovery are unset.
//!
//! Each port can be overridden via `NESTGATE_FALLBACK_PORT_<NAME>` environment variables.
//! Compile-time defaults live in [`defaults`]; accessor functions resolve env overrides once.

use std::sync::LazyLock;

/// Compile-time fallback port defaults (used when env vars are unset).
pub mod defaults {
    /// Fallback port for plain HTTP API listeners.
    pub const HTTP: u16 = 8080;
    /// Fallback port for TLS-terminated API listeners.
    pub const HTTPS: u16 = 8443;
    /// Fallback port for the primary REST/JSON-RPC API.
    pub const API: u16 = 3000;
    /// Fallback alternate API port.
    pub const API_ALT: u16 = 3001;
    /// Fallback port for `Prometheus`-compatible metrics endpoint.
    pub const METRICS: u16 = 9090;
    /// Fallback port for the health/liveness probe endpoint.
    pub const HEALTH: u16 = 8081;
    /// Fallback port for tarpc RPC listeners.
    pub const TARPC: u16 = 8091;
    /// Fallback port for the jsonrpsee JSON-RPC HTTP server.
    pub const JSONRPC: u16 = 8092;
    /// Fallback port for gRPC listeners.
    pub const GRPC: u16 = 50051;
    /// Fallback port for `WebSocket` connections.
    pub const WEBSOCKET: u16 = 8082;
    /// Fallback port for the admin/management API.
    pub const ADMIN: u16 = 9000;
    /// Fallback port for the storage service endpoint.
    pub const STORAGE: u16 = 5000;
    /// Fallback port for the orchestration service.
    pub const ORCHESTRATION: u16 = 8083;
    /// Fallback port for compute service endpoints.
    pub const COMPUTE: u16 = 8085;
    /// Fallback port for extended/auxiliary services.
    pub const EXTENDED_SERVICES: u16 = 3002;
    /// Fallback port for the ecosystem coordination endpoint.
    pub const ECOSYSTEM: u16 = 6000;
    /// Fallback port for the discovery service.
    pub const DISCOVERY_SERVICE: u16 = 3010;
    /// Standalone orchestrator coordination port (legacy bootstrap default).
    pub const ORCHESTRATOR_DEFAULT: u16 = 8090;
    /// Fallback alternate metrics port.
    pub const METRICS_ALT: u16 = 9001;
    /// Fallback port for `PostgreSQL` (external service, env-resolved).
    pub const POSTGRES: u16 = 5432;
    /// Fallback port for `Redis` (external service, env-resolved).
    pub const REDIS: u16 = 6379;
    /// Fallback port for `MongoDB` (external service, env-resolved).
    pub const MONGODB: u16 = 27017;
    /// Fallback port for `MySQL` (external service, env-resolved).
    pub const MYSQL: u16 = 3306;
}

/// Returns the value from env var `var` if set and parseable, else `default`.
fn env_or_default<T: std::str::FromStr>(var: &str, default: T) -> T {
    std::env::var(var)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

macro_rules! define_fallback_port {
    ($(#[doc = $doc:expr])* $fn_name:ident, $default:ident, $static_name:ident, $env:literal) => {
        static $static_name: LazyLock<u16> =
            LazyLock::new(|| env_or_default($env, defaults::$default));

        $(#[doc = $doc])*
        #[must_use]
        #[inline]
        pub fn $fn_name() -> u16 {
            *$static_name
        }
    };
}

define_fallback_port!(/// Plain HTTP (env: `NESTGATE_FALLBACK_PORT_HTTP`, default 8080).
    http, HTTP, HTTP_LAZY, "NESTGATE_FALLBACK_PORT_HTTP");
define_fallback_port!(/// TLS-terminated (env: `NESTGATE_FALLBACK_PORT_HTTPS`, default 8443).
    https, HTTPS, HTTPS_LAZY, "NESTGATE_FALLBACK_PORT_HTTPS");
define_fallback_port!(/// Primary API (env: `NESTGATE_FALLBACK_PORT_API`, default 3000).
    api, API, API_LAZY, "NESTGATE_FALLBACK_PORT_API");
define_fallback_port!(/// Alternate API (env: `NESTGATE_FALLBACK_PORT_API_ALT`, default 3001).
    api_alt, API_ALT, API_ALT_LAZY, "NESTGATE_FALLBACK_PORT_API_ALT");
define_fallback_port!(/// Prometheus metrics (env: `NESTGATE_FALLBACK_PORT_METRICS`, default 9090).
    metrics, METRICS, METRICS_LAZY, "NESTGATE_FALLBACK_PORT_METRICS");
define_fallback_port!(/// Health probe (env: `NESTGATE_FALLBACK_PORT_HEALTH`, default 8081).
    health, HEALTH, HEALTH_LAZY, "NESTGATE_FALLBACK_PORT_HEALTH");
define_fallback_port!(/// tarpc RPC (env: `NESTGATE_FALLBACK_PORT_TARPC`, default 8091).
    tarpc, TARPC, TARPC_LAZY, "NESTGATE_FALLBACK_PORT_TARPC");
define_fallback_port!(/// jsonrpsee JSON-RPC HTTP (env: `NESTGATE_FALLBACK_PORT_JSONRPC`, default 8092).
    jsonrpc, JSONRPC, JSONRPC_LAZY, "NESTGATE_FALLBACK_PORT_JSONRPC");
define_fallback_port!(/// gRPC (env: `NESTGATE_FALLBACK_PORT_GRPC`, default 50051).
    grpc, GRPC, GRPC_LAZY, "NESTGATE_FALLBACK_PORT_GRPC");
define_fallback_port!(/// `WebSocket` (env: `NESTGATE_FALLBACK_PORT_WEBSOCKET`, default 8082).
    websocket, WEBSOCKET, WEBSOCKET_LAZY, "NESTGATE_FALLBACK_PORT_WEBSOCKET");
define_fallback_port!(/// Admin API (env: `NESTGATE_FALLBACK_PORT_ADMIN`, default 9000).
    admin, ADMIN, ADMIN_LAZY, "NESTGATE_FALLBACK_PORT_ADMIN");
define_fallback_port!(/// Storage service (env: `NESTGATE_FALLBACK_PORT_STORAGE`, default 5000).
    storage, STORAGE, STORAGE_LAZY, "NESTGATE_FALLBACK_PORT_STORAGE");
define_fallback_port!(/// Orchestration (env: `NESTGATE_FALLBACK_PORT_ORCHESTRATION`, default 8083).
    orchestration, ORCHESTRATION, ORCHESTRATION_LAZY, "NESTGATE_FALLBACK_PORT_ORCHESTRATION");
define_fallback_port!(/// Compute service (env: `NESTGATE_FALLBACK_PORT_COMPUTE`, default 8085).
    compute, COMPUTE, COMPUTE_LAZY, "NESTGATE_FALLBACK_PORT_COMPUTE");
define_fallback_port!(/// Extended services (env: `NESTGATE_FALLBACK_PORT_EXTENDED_SERVICES`, default 3002).
    extended_services, EXTENDED_SERVICES, EXTENDED_SERVICES_LAZY, "NESTGATE_FALLBACK_PORT_EXTENDED_SERVICES");
define_fallback_port!(/// Ecosystem coordination (env: `NESTGATE_FALLBACK_PORT_ECOSYSTEM`, default 6000).
    ecosystem, ECOSYSTEM, ECOSYSTEM_LAZY, "NESTGATE_FALLBACK_PORT_ECOSYSTEM");
define_fallback_port!(/// Discovery service (env: `NESTGATE_FALLBACK_PORT_DISCOVERY_SERVICE`, default 3010).
    discovery_service, DISCOVERY_SERVICE, DISCOVERY_SERVICE_LAZY, "NESTGATE_FALLBACK_PORT_DISCOVERY_SERVICE");
define_fallback_port!(/// Orchestrator legacy default (env: `NESTGATE_FALLBACK_PORT_ORCHESTRATOR_DEFAULT`, default 8090).
    orchestrator_default, ORCHESTRATOR_DEFAULT, ORCHESTRATOR_DEFAULT_LAZY, "NESTGATE_FALLBACK_PORT_ORCHESTRATOR_DEFAULT");
define_fallback_port!(/// Alternate metrics (env: `NESTGATE_FALLBACK_PORT_METRICS_ALT`, default 9001).
    metrics_alt, METRICS_ALT, METRICS_ALT_LAZY, "NESTGATE_FALLBACK_PORT_METRICS_ALT");
define_fallback_port!(/// `PostgreSQL` (env: `NESTGATE_FALLBACK_PORT_POSTGRES`, default 5432).
    postgres, POSTGRES, POSTGRES_LAZY, "NESTGATE_FALLBACK_PORT_POSTGRES");
define_fallback_port!(/// Redis (env: `NESTGATE_FALLBACK_PORT_REDIS`, default 6379).
    redis, REDIS, REDIS_LAZY, "NESTGATE_FALLBACK_PORT_REDIS");
define_fallback_port!(/// `MongoDB` (env: `NESTGATE_FALLBACK_PORT_MONGODB`, default 27017).
    mongodb, MONGODB, MONGODB_LAZY, "NESTGATE_FALLBACK_PORT_MONGODB");
define_fallback_port!(/// `MySQL` (env: `NESTGATE_FALLBACK_PORT_MYSQL`, default 3306).
    mysql, MYSQL, MYSQL_LAZY, "NESTGATE_FALLBACK_PORT_MYSQL");

/// Fallback port for `Prometheus` scrape endpoint (alias of [`metrics`]).
#[must_use]
#[inline]
pub fn prometheus() -> u16 {
    metrics()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_declared_ports_are_nonzero() {
        let ports = [
            http(),
            https(),
            api(),
            api_alt(),
            metrics(),
            prometheus(),
            health(),
            tarpc(),
            jsonrpc(),
            grpc(),
            websocket(),
            admin(),
            storage(),
            orchestration(),
            compute(),
            extended_services(),
            ecosystem(),
            discovery_service(),
            orchestrator_default(),
            metrics_alt(),
            postgres(),
            redis(),
            mongodb(),
            mysql(),
        ];
        for p in ports {
            assert!(p > 0, "port must be non-zero, got {p}");
        }
    }

    #[test]
    fn metrics_and_prometheus_are_intentional_aliases() {
        assert_eq!(metrics(), prometheus());
        assert_eq!(metrics(), defaults::METRICS);
    }

    #[test]
    fn all_primary_service_ports_are_unique_except_known_aliases() {
        use std::collections::HashMap;

        let pairs: [(&str, u16); 24] = [
            ("HTTP", http()),
            ("HTTPS", https()),
            ("API", api()),
            ("API_ALT", api_alt()),
            ("METRICS", metrics()),
            ("PROMETHEUS", prometheus()),
            ("HEALTH", health()),
            ("TARPC", tarpc()),
            ("JSONRPC", jsonrpc()),
            ("GRPC", grpc()),
            ("WEBSOCKET", websocket()),
            ("ADMIN", admin()),
            ("STORAGE", storage()),
            ("ORCHESTRATION", orchestration()),
            ("COMPUTE", compute()),
            ("EXTENDED_SERVICES", extended_services()),
            ("ECOSYSTEM", ecosystem()),
            ("DISCOVERY_SERVICE", discovery_service()),
            ("ORCHESTRATOR_DEFAULT", orchestrator_default()),
            ("METRICS_ALT", metrics_alt()),
            ("POSTGRES", postgres()),
            ("REDIS", redis()),
            ("MONGODB", mongodb()),
            ("MYSQL", mysql()),
        ];

        let mut by_value: HashMap<u16, Vec<&str>> = HashMap::new();
        for (name, port) in pairs {
            by_value.entry(port).or_default().push(name);
        }

        for (port, names) in &by_value {
            if names.len() > 1 {
                assert!(
                    *port == metrics() && names.len() == 2,
                    "unexpected duplicate port {port}: {names:?}"
                );
                assert!(names.contains(&"METRICS") && names.contains(&"PROMETHEUS"));
            }
        }
    }
}
