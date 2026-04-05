// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **CANONICAL API TYPES**
//!
//! Re-exports of canonical API configuration types for easy access within nestgate-api.
//! Provides backward-compatible type aliases for smooth migration.

// ==================== CANONICAL API CONFIGURATION RE-EXPORTS ====================

/// Re-export canonical API configuration types from nestgate-core
pub use nestgate_core::config::canonical_primary::domains::network::{
    ApiAlertConfig, ApiConfig as CanonicalApiConfig, ApiMonitoringConfig, ApiPerformanceConfig,
    ApiSecurityConfig, RateLimitingConfig, TlsConfig,
};

// ==================== BACKWARD COMPATIBILITY ALIASES ====================

/// Backward compatibility alias for `UnifiedApiConfig`
///
/// **Migration Path**: Use `CanonicalApiConfig` or the specific sub-configs instead.
pub use CanonicalApiConfig as UnifiedApiConfig;

/// Backward compatibility alias for `ApiConfig`
pub use CanonicalApiConfig as ApiConfig;

/// Backward compatibility alias for `NetworkApiConfig`
pub use CanonicalApiConfig as NetworkApiConfig;
