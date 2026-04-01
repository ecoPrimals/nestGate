// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Remote ZFS Service Tests
//!
//! Comprehensive test suite for the remote ZFS service implementation,
//! covering connection management, error handling, and service discovery.
//!
//! ## Test Coverage Status
//!
//! **Current Coverage**: Basic service creation and configuration
//!
//! **Pending Test Expansion** (tracked in coverage sprint):
//! - Advanced connection management (circuit breaker, retry, pooling)
//! - Error recovery scenarios
//! - Concurrent connection handling
//! - Full service lifecycle integration
//!
//! ## Environment-Driven Configuration
//!
//! Tests use environment variables for configuration:
//! - `NESTGATE_HOSTNAME`: Service hostname (default: from network constants)
//! - `NESTGATE_API_PORT`: Service port (default: from port constants)
//!
//! ## Note on Ignored Tests
//!
//! Several tests are marked `#[ignore]` as they require implementation of:
//! - Connection pool management types
//! - Circuit breaker patterns
//! - Advanced health checking
//!
//! These will be implemented during the connection infrastructure expansion phase.

mod basic_tests;
mod config_tests;
mod connection_error_tests;
mod connection_stats_tests;
mod service_tests;
mod validation_tests;

// Re-export for convenience
