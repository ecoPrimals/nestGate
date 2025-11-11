//! Modern production Module
//! 
//! This module provides core functionality using modern Rust patterns
//! and zero-cost abstractions.

use std::time::Duration;
use crate::error::NestGateUnifiedError;
use std::sync::Arc;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::error::{NestGateError, Result};
// ==================== MODULE CONSTANTS ====================
/// Module version for compatibility tracking
pub use crate::constants::shared::MODULE_VERSION;
/// Default configuration values
/// Default configuration values from canonical constants
pub use crate::constants::network::{
    DEFAULT_TIMEOUT_MS, DEFAULT_BUFFER_SIZE, DEFAULT_MAX_CONNECTIONS
};
// ==================== CORE TYPES ====================
/// Configuration for this module
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::config::UniversalAdapterProductionConfig;
/// 
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::UniversalAdapterProductionConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct UniversalAdapterProductionConfig {
    pub enabled: bool,
    pub timeout: Duration,
    pub max_connections: usize,
    pub buffer_size: usize,
}

impl Default for UniversalAdapterProductionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_millis(DEFAULT_TIMEOUT_MS),
            max_connections: DEFAULT_MAX_CONNECTIONS,
            buffer_size: DEFAULT_BUFFER_SIZE,
        }
    }
}

/// Service interface for this module
pub trait Service: Send + Sync {
    /// Initialize the service
    fn initialize(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    /// Check service health
    fn health_check(&self) -> impl std::future::Future<Output = Result<HealthStatus>> + Send;
    /// Shutdown the service gracefully
    fn shutdown(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}

/// Health status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Performance metrics for monitoring
pub struct Metrics {
    pub requests_processed: u64,
    pub errors_encountered: u64,
    pub average_response_time: Duration,
    pub memory_usage_bytes: u64,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            requests_processed: 0,
            errors_encountered: 0,
            average_response_time: Duration::from_millis(0),
            memory_usage_bytes: 0,
        }
    }
}

// ==================== ERROR TYPES ====================
// Note: Deprecated UniversalAdapterError removed (November 10, 2025)
// Use NestGateUnifiedError or NestGateError instead

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type UniversalAdapterProductionConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using UniversalAdapterProductionConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

