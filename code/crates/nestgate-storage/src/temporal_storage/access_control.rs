// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **Access Control & Security**
//!
//! Domain: Access control, authentication, and rate limiting
//!
//! This module handles:
//! - Access requirements for data sources
//! - Authentication methods (API keys, OAuth, certificates, etc.)
//! - Rate limiting configurations
//! - Geographic and legal restrictions

use serde::{Deserialize, Serialize};

/// Access requirements for data sources
///
/// Defines the security and access control requirements
/// needed to connect to and use a data source.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessRequirements {
    /// Authentication method required (if any)
    pub authentication: Option<AuthenticationMethod>,
    /// Rate limiting configuration (if any)
    pub rate_limits: Option<RateLimits>,
    /// Geographic restrictions (list of restricted regions)
    pub geographic_restrictions: Vec<String>,
    /// Legal requirements or compliance notes
    pub legal_requirements: Vec<String>,
}

/// Authentication methods for data source access
///
/// Comprehensive authentication method support for all data sources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    /// API key authentication with key value
    ApiKey(String),
    /// OAuth 2.0 authentication with token
    OAuth {
        /// OAuth access token
        token: String,
        /// Token expiration time (Unix timestamp)
        expires_at: Option<u64>,
    },
    /// Username and password authentication
    BasicAuth {
        /// Username
        username: String,
        /// Password (should be encrypted/hashed in production)
        password: String,
    },
    /// Certificate-based authentication
    Certificate {
        /// Path to certificate file
        cert_path: String,
        /// Path to private key file
        key_path: String,
    },
    /// JWT token authentication
    Jwt {
        /// JWT token string
        token: String,
        /// Token expiration time (Unix timestamp)
        expires_at: Option<u64>,
    },
    /// No authentication required (public access)
    None,
    /// Custom authentication method with name and parameters
    Custom {
        /// Authentication method name
        method: String,
        /// Authentication parameters
        params: std::collections::HashMap<String, String>,
    },
}

/// Rate limiting configuration
///
/// Defines rate limits for API or data access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimits {
    /// Maximum requests per minute
    pub requests_per_minute: u32,
    /// Maximum requests per hour
    pub requests_per_hour: u32,
    /// Maximum requests per day
    pub requests_per_day: u32,
}

impl RateLimits {
    /// Create rate limits with per-minute limit
    ///
    /// # Arguments
    ///
    /// * `per_minute` - Requests allowed per minute
    ///
    /// # Returns
    ///
    /// Rate limits with reasonable hour/day limits derived from per-minute
    #[must_use]
    pub const fn per_minute(per_minute: u32) -> Self {
        Self {
            requests_per_minute: per_minute,
            requests_per_hour: per_minute * 60,
            requests_per_day: per_minute * 60 * 24,
        }
    }

    /// Create rate limits with per-hour limit
    ///
    /// # Arguments
    ///
    /// * `per_hour` - Requests allowed per hour
    ///
    /// # Returns
    ///
    /// Rate limits with reasonable minute/day limits derived from per-hour
    #[must_use]
    pub const fn per_hour(per_hour: u32) -> Self {
        Self {
            requests_per_minute: per_hour / 60,
            requests_per_hour: per_hour,
            requests_per_day: per_hour * 24,
        }
    }

    /// Create rate limits with per-day limit
    ///
    /// # Arguments
    ///
    /// * `per_day` - Requests allowed per day
    ///
    /// # Returns
    ///
    /// Rate limits with reasonable minute/hour limits derived from per-day
    #[must_use]
    pub const fn per_day(per_day: u32) -> Self {
        Self {
            requests_per_minute: per_day / (60 * 24),
            requests_per_hour: per_day / 24,
            requests_per_day: per_day,
        }
    }

    /// Check if a request count exceeds any limit
    ///
    /// # Arguments
    ///
    /// * `requests_minute` - Requests in current minute
    /// * `requests_hour` - Requests in current hour
    /// * `requests_day` - Requests in current day
    ///
    /// # Returns
    ///
    /// `true` if any limit is exceeded
    #[must_use]
    pub const fn is_exceeded(
        &self,
        requests_minute: u32,
        requests_hour: u32,
        requests_day: u32,
    ) -> bool {
        requests_minute >= self.requests_per_minute
            || requests_hour >= self.requests_per_hour
            || requests_day >= self.requests_per_day
    }
}

impl AccessRequirements {
    /// Create public access requirements (no auth, no limits)
    ///
    /// # Returns
    ///
    /// Access requirements for public data
    #[must_use]
    pub const fn public() -> Self {
        Self {
            authentication: Some(AuthenticationMethod::None),
            rate_limits: None,
            geographic_restrictions: Vec::new(),
            legal_requirements: Vec::new(),
        }
    }

    /// Create access requirements with API key
    ///
    /// # Arguments
    ///
    /// * `api_key` - API key for authentication
    ///
    /// # Returns
    ///
    /// Access requirements with API key authentication
    #[must_use]
    pub const fn with_api_key(api_key: String) -> Self {
        Self {
            authentication: Some(AuthenticationMethod::ApiKey(api_key)),
            rate_limits: None,
            geographic_restrictions: Vec::new(),
            legal_requirements: Vec::new(),
        }
    }

    /// Add geographic restriction
    ///
    /// # Arguments
    ///
    /// * `region` - Region code to restrict (e.g., "US", "EU", "CN")
    pub fn add_geographic_restriction(&mut self, region: String) {
        self.geographic_restrictions.push(region);
    }

    /// Add legal requirement
    ///
    /// # Arguments
    ///
    /// * `requirement` - Legal requirement description
    pub fn add_legal_requirement(&mut self, requirement: String) {
        self.legal_requirements.push(requirement);
    }

    /// Check if authentication is required
    ///
    /// # Returns
    ///
    /// `true` if authentication is required
    #[must_use]
    pub const fn requires_authentication(&self) -> bool {
        !matches!(self.authentication, None | Some(AuthenticationMethod::None))
    }
}
