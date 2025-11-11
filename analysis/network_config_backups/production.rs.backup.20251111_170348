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
