/// Unified Timeout Configuration Module
/// Consolidates all timeout patterns across the entire system
/// **PROBLEM SOLVED**: Eliminates 150+ duplicate timeout fields
use serde::{Deserialize, Serialize};
use std::time::Duration;
/// **THE** Unified Timeout Configuration - eliminates 150+ duplicate timeout fields
/// Consolidates all timeout patterns across the entire system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedTimeoutConfig {
    /// Default timeout for general operations
    pub default_timeout: Duration,
    /// Connection establishment timeout
    pub connection_timeout: Duration,
    /// Request/response timeout
    pub request_timeout: Duration,
    /// Health check timeout
    pub health_check_timeout: Duration,
    /// Database operation timeout
    pub database_timeout: Duration,
    /// Network operation timeout
    pub network_timeout: Duration,
    /// File I/O timeout
    pub file_timeout: Duration,
    /// Service discovery timeout
    pub discovery_timeout: Duration,
}
impl Default for UnifiedTimeoutConfig {
    fn default() -> Self {
        Self {
            default_timeout: Duration::from_secs(30),
            connection_timeout: Duration::from_secs(10),
            request_timeout: Duration::from_secs(60),
            health_check_timeout: Duration::from_secs(5),
            database_timeout: Duration::from_secs(30),
            network_timeout: Duration::from_secs(15),
            file_timeout: Duration::from_secs(30),
            discovery_timeout: Duration::from_secs(10),
        }
    }
}

impl UnifiedTimeoutConfig {
    /// Create a production-optimized timeout configuration
    #[must_use]
    pub const fn production() -> Self {
        Self {
            default_timeout: Duration::from_secs(60),
            connection_timeout: Duration::from_secs(15),
            request_timeout: Duration::from_secs(120),
            health_check_timeout: Duration::from_secs(10),
            database_timeout: Duration::from_secs(45),
            network_timeout: Duration::from_secs(30),
            file_timeout: Duration::from_secs(60),
            discovery_timeout: Duration::from_secs(20),
        }
    }

    /// Create a development-optimized timeout configuration
    #[must_use]
    pub const fn development() -> Self {
        Self {
            default_timeout: Duration::from_secs(10),
            connection_timeout: Duration::from_secs(5),
            request_timeout: Duration::from_secs(30),
            health_check_timeout: Duration::from_secs(3),
            database_timeout: Duration::from_secs(15),
            network_timeout: Duration::from_secs(10),
            file_timeout: Duration::from_secs(15),
            discovery_timeout: Duration::from_secs(5),
        }
    }
}
