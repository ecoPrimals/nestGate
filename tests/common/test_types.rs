// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Common test types and utilities
//!
//! This module provides type aliases for test configurations with const generics.

use nestgate_core::config::canonical_primary::NestGateCanonicalConfig;

// ==================== TYPE ALIASES FOR CONST GENERICS ====================

/// Default test configuration with standard parameters
/// - MAX_CONNECTIONS: 1000
/// - BUFFER_SIZE: 4096
/// - TIMEOUT_MS: 30000
/// - API_PORT: 8080
pub type DefaultTestConfig = NestGateCanonicalConfig<1000, 4096, 30000, 8080>;

/// High-performance test configuration
/// - MAX_CONNECTIONS: 5000
/// - BUFFER_SIZE: 8192
/// - TIMEOUT_MS: 60000
/// - API_PORT: 8080
pub type HighPerfTestConfig = NestGateCanonicalConfig<5000, 8192, 60000, 8080>;

/// Lightweight test configuration for quick tests
/// - MAX_CONNECTIONS: 100
/// - BUFFER_SIZE: 1024
/// - TIMEOUT_MS: 5000
/// - API_PORT: 8080
pub type LightweightTestConfig = NestGateCanonicalConfig<100, 1024, 5000, 8080>;
