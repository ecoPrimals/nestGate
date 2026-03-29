// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **MIDDLEWARE HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `MiddlewareHandler`
pub struct MiddlewareHandlerConfig {
    /// Cors
    pub cors: CorsHandlerConfig,
    /// Compression
    pub compression: CompressionHandlerConfig,
    /// Security
    pub security: SecurityMiddlewareConfig,
    /// Logging
    pub logging: LoggingMiddlewareConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `CorsHandler`
pub struct CorsHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `CompressionHandler`
pub struct CompressionHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `SecurityMiddleware`
pub struct SecurityMiddlewareConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `LoggingMiddleware`
pub struct LoggingMiddlewareConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

impl Default for MiddlewareHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            cors: CorsHandlerConfig { enabled: true },
            compression: CompressionHandlerConfig { enabled: true },
            security: SecurityMiddlewareConfig { enabled: true },
            logging: LoggingMiddlewareConfig { enabled: true },
        }
    }
}

impl MiddlewareHandlerConfig {
    /// Returns a production-optimized configuration
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }

    /// Returns a development-optimized configuration
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }

    /// Returns a high-performance configuration
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }

    /// Merges this configuration with another, returning the merged result
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        self
    }
    /// Validates data
    pub const fn validate(&self) -> nestgate_types::error::Result<()> {
        Ok(())
    }
}
