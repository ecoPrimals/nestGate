// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Compile-time port fallbacks for bootstrap, tests, and legacy paths.
//!
//! Prefer [`crate::constants::capability_port_discovery::RuntimePortResolver`] or the
//! `discover_*_port` helpers for production; these values are last-resort defaults when env and
//! discovery are unset.

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
/// Fallback port for `Prometheus` scrape endpoint (alias of METRICS).
pub const PROMETHEUS: u16 = 9090;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_declared_ports_are_nonzero() {
        let ports = [
            HTTP,
            HTTPS,
            API,
            API_ALT,
            METRICS,
            PROMETHEUS,
            HEALTH,
            TARPC,
            JSONRPC,
            GRPC,
            WEBSOCKET,
            ADMIN,
            STORAGE,
            ORCHESTRATION,
            COMPUTE,
            EXTENDED_SERVICES,
            ECOSYSTEM,
            DISCOVERY_SERVICE,
            ORCHESTRATOR_DEFAULT,
            METRICS_ALT,
            POSTGRES,
            REDIS,
            MONGODB,
            MYSQL,
        ];
        for p in ports {
            assert!(p > 0, "port must be non-zero, got {p}");
        }
    }

    #[test]
    fn metrics_and_prometheus_are_intentional_aliases() {
        assert_eq!(METRICS, PROMETHEUS);
        assert_eq!(METRICS, 9090);
    }

    #[test]
    fn all_primary_service_ports_are_unique_except_known_aliases() {
        use std::collections::HashMap;

        let pairs: [(&str, u16); 24] = [
            ("HTTP", HTTP),
            ("HTTPS", HTTPS),
            ("API", API),
            ("API_ALT", API_ALT),
            ("METRICS", METRICS),
            ("PROMETHEUS", PROMETHEUS),
            ("HEALTH", HEALTH),
            ("TARPC", TARPC),
            ("JSONRPC", JSONRPC),
            ("GRPC", GRPC),
            ("WEBSOCKET", WEBSOCKET),
            ("ADMIN", ADMIN),
            ("STORAGE", STORAGE),
            ("ORCHESTRATION", ORCHESTRATION),
            ("COMPUTE", COMPUTE),
            ("EXTENDED_SERVICES", EXTENDED_SERVICES),
            ("ECOSYSTEM", ECOSYSTEM),
            ("DISCOVERY_SERVICE", DISCOVERY_SERVICE),
            ("ORCHESTRATOR_DEFAULT", ORCHESTRATOR_DEFAULT),
            ("METRICS_ALT", METRICS_ALT),
            ("POSTGRES", POSTGRES),
            ("REDIS", REDIS),
            ("MONGODB", MONGODB),
            ("MYSQL", MYSQL),
        ];

        let mut by_value: HashMap<u16, Vec<&str>> = HashMap::new();
        for (name, port) in pairs {
            by_value.entry(port).or_default().push(name);
        }

        for (port, names) in &by_value {
            if names.len() > 1 {
                assert!(
                    *port == METRICS && names.len() == 2,
                    "unexpected duplicate port {port}: {names:?}"
                );
                assert!(names.contains(&"METRICS") && names.contains(&"PROMETHEUS"));
            }
        }
    }
}
