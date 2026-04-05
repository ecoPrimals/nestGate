// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Compile-time port fallbacks when environment variables are unset.
//!
//! These mirror the numeric defaults used by `RuntimeDefaults` and the deprecated `ports`
//! module. **Prefer** `RuntimeDefaults`, `get_api_port`, `get_metrics_port`, or capability
//! discovery at runtime instead of importing this module for new code.

/// Default HTTP service port fallback
pub const HTTP: u16 = 8080;
/// Default HTTPS service port fallback
pub const HTTPS: u16 = 8443;
/// Default API port fallback
pub const API: u16 = 3000;
/// Alternate API port fallback
pub const API_ALT: u16 = 3001;
/// Metrics / observability port fallback
pub const METRICS: u16 = 9090;
/// Prometheus scrape port fallback
pub const PROMETHEUS: u16 = 9090;
/// Health check endpoint port fallback
pub const HEALTH: u16 = 8081;
/// gRPC service port fallback
pub const GRPC: u16 = 50051;
/// WebSocket service port fallback
pub const WEBSOCKET: u16 = 8082;
/// Admin UI or control plane port fallback
pub const ADMIN: u16 = 9000;
/// Storage service port fallback
pub const STORAGE: u16 = 5000;
/// Orchestration service port fallback
pub const ORCHESTRATION: u16 = 8083;
/// Storage discovery service port fallback
pub const STORAGE_DISCOVERY: u16 = 8084;
/// Compute service port fallback
pub const COMPUTE: u16 = 8085;
/// Extended services port fallback
pub const EXTENDED_SERVICES: u16 = 3002;
/// Ecosystem orchestration service port fallback (dev bootstrap; prefer discovery)
pub const ECOSYSTEM: u16 = 6000;
/// Service discovery registry port fallback
pub const DISCOVERY_SERVICE: u16 = 3010;
/// Alternate metrics port fallback
pub const METRICS_ALT: u16 = 9001;
/// Metrics Prometheus alias port fallback
pub const METRICS_PROMETHEUS: u16 = 9090;
/// Default health-related port fallback
pub const HEALTH_DEFAULT: u16 = 8081;
/// Orchestrator peer port fallback
pub const ORCHESTRATOR: u16 = 8090;
/// Security service port fallback
pub const SECURITY_SERVICE: u16 = 8081;
/// Networking service port fallback
pub const NETWORKING_SERVICE: u16 = 8082;
/// `PostgreSQL` port fallback
pub const POSTGRES: u16 = 5432;
/// Redis port fallback
pub const REDIS: u16 = 6379;
/// `MongoDB` port fallback
pub const MONGODB: u16 = 27017;
/// `MySQL` port fallback
pub const MYSQL: u16 = 3306;
/// Streaming RPC port fallback
pub const STREAMING_RPC: u16 = 8001;
