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
/// Configuration for UniversalAdapterProduction
pub struct UniversalAdapterProductionConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Timeout
    pub timeout: Duration,
    /// Max Connections
    pub max_connections: usize,
    /// Size of buffer
    pub buffer_size: usize,
}

impl Default for UniversalAdapterProductionConfig {
    /// Returns the default instance
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
/// Status values for Health
pub enum HealthStatus {
    /// Healthy
    Healthy,
    /// Degraded
    Degraded,
    /// Unhealthy
    Unhealthy,
}

/// Performance metrics for monitoring
pub struct Metrics {
    /// Requests Processed
    pub requests_processed: u64,
    /// Errors Encountered
    pub errors_encountered: u64,
    /// Average Response Time
    pub average_response_time: Duration,
    /// Memory Usage Bytes
    pub memory_usage_bytes: u64,
}

impl Default for Metrics {
    /// Returns the default instance
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
/// Type alias for Universaladapterproductionconfigcanonical
pub type UniversalAdapterProductionConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using UniversalAdapterProductionConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

