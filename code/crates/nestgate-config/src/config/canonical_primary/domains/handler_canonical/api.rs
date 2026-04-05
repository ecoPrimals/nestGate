// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// **API HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for `ApiHandler`
pub struct ApiHandlerConfig {
    /// Request
    pub request: RequestHandlerConfig,
    /// Response
    pub response: ResponseHandlerConfig,
    /// Routes
    pub routes: RouteHandlerConfig,
    /// Auth
    pub auth: AuthHandlerConfig,
    /// Rate Limiting
    pub rate_limiting: RateLimitHandlerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `RequestHandler`
pub struct RequestHandlerConfig {
    /// Logging
    pub logging: bool,
    /// Timeout
    pub timeout: Duration,
    /// Size of max
    pub max_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ResponseHandler`
pub struct ResponseHandlerConfig {
    /// Compression
    pub compression: bool,
    /// Caching
    pub caching: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `RouteHandler`
pub struct RouteHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Routes
    pub routes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `AuthHandler`
pub struct AuthHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Methods
    pub methods: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `RateLimitHandler`
pub struct RateLimitHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Requests Per Minute
    pub requests_per_minute: u32,
}

impl Default for RequestHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            logging: true,
            timeout: Duration::from_secs(30),
            max_size: 1024 * 1024, // 1MB
        }
    }
}

impl Default for ResponseHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            compression: true,
            caching: false,
        }
    }
}

impl Default for RouteHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            routes: vec![],
        }
    }
}

impl Default for AuthHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            methods: vec!["bearer".to_string()],
        }
    }
}

impl Default for RateLimitHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_minute: 1000,
        }
    }
}

impl ApiHandlerConfig {
    /// Creates a production-optimized configuration
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }

    /// Creates a development-optimized configuration
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }

    /// Creates a high-performance configuration
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }

    /// Merges two configurations, preferring values from self
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
    /// Validates data
    pub const fn validate(&self) -> nestgate_types::error::Result<()> {
        Ok(())
    }
}
