// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Default network ports (deprecated compile-time fallbacks).
#![deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]

/// Default HTTP port
///
/// Prefer capability discovery; this is a compile-time fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const HTTP_DEFAULT: u16 = 8080;

/// Default HTTPS port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const HTTPS_DEFAULT: u16 = 8443;

/// Default API server port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const API_DEFAULT: u16 = 3000;

/// Alternative API port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const API_ALT: u16 = 3001;

/// Default metrics/monitoring port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const METRICS_DEFAULT: u16 = 9090;

/// Prometheus metrics port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const PROMETHEUS: u16 = 9090;

/// Default health check port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const HEALTH_CHECK: u16 = 8081;

/// Default gRPC port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const GRPC_DEFAULT: u16 = 50051;

/// Default WebSocket port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const WEBSOCKET_DEFAULT: u16 = 8082;

/// Default admin interface port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const ADMIN_DEFAULT: u16 = 9000;

/// Default storage service port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const STORAGE_DEFAULT: u16 = 5000;

/// Default orchestration service port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const ORCHESTRATION_DEFAULT: u16 = 8083;

/// Default storage discovery port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const STORAGE_DISCOVERY_DEFAULT: u16 = 8084;

/// Default compute service port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const COMPUTE_DEFAULT: u16 = 8085;

/// Extended services port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const EXTENDED_SERVICES: u16 = 3002;

/// Discovery service port
///
/// Prefer capability discovery—avoid fixed discovery ports in production.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const DISCOVERY_SERVICE: u16 = 3010;

/// Alternative metrics port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const METRICS_ALT: u16 = 9001;

/// Prometheus metrics port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const METRICS_PROMETHEUS: u16 = 9090;

/// Default health check port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const HEALTH_DEFAULT: u16 = 8081;

/// Orchestrator port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const ORCHESTRATOR_DEFAULT: u16 = 8090;

/// Generic security service default port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const SECURITY_SERVICE_DEFAULT: u16 = 8081;

/// Generic networking service default port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const NETWORKING_SERVICE_DEFAULT: u16 = 8082;

/// `PostgreSQL` database default port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const POSTGRES_DEFAULT: u16 = 5432;

/// Redis cache default port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const REDIS_DEFAULT: u16 = 6379;

/// `MongoDB` database default port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const MONGODB_DEFAULT: u16 = 27017;

/// `MySQL` database default port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const MYSQL_DEFAULT: u16 = 3306;

/// Streaming RPC default port
///
/// Prefer capability discovery at runtime; fallback only.
#[deprecated(
    since = "0.2.0",
    note = "Use capability-based discovery. Ports are resolved at runtime via primal discovery."
)]
pub const STREAMING_RPC_DEFAULT: u16 = 8001;
