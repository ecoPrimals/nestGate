// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # Security Configuration
//!
//! Security environment configuration extracted for logical cohesion.
//!
//! **Phase 3: Smart Refactoring** - Extracted from monolithic `environment.rs` (Jan 30, 2026)

use serde::{Deserialize, Serialize};
use std::env;
use std::str::FromStr;

use super::ConfigError;

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable TLS (default: false)
    pub tls_enabled: bool,

    /// TLS certificate path
    pub tls_cert_path: Option<String>,

    /// TLS private key path
    pub tls_key_path: Option<String>,

    /// API key for authentication
    pub api_key: Option<String>,

    /// Rate limiting enabled (default: true)
    pub rate_limit_enabled: bool,

    /// Rate limit: requests per minute (default: 100)
    pub rate_limit_rpm: u32,
}

impl SecurityConfig {
    /// Load from environment
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_env_with_prefix("NESTGATE")
    }

    /// Load from environment with custom prefix
    pub fn from_env_with_prefix(prefix: &str) -> Result<Self, ConfigError> {
        Ok(Self {
            tls_enabled: Self::env_var_or(prefix, "TLS_ENABLED", false)?,
            tls_cert_path: env::var(format!("{prefix}_TLS_CERT")).ok(),
            tls_key_path: env::var(format!("{prefix}_TLS_KEY")).ok(),
            api_key: env::var(format!("{prefix}_API_KEY")).ok(),
            rate_limit_enabled: Self::env_var_or(prefix, "RATE_LIMIT_ENABLED", true)?,
            rate_limit_rpm: Self::env_var_or(prefix, "RATE_LIMIT_RPM", 100)?,
        })
    }

    /// Helper to get environment variable or use default
    fn env_var_or<T: FromStr>(prefix: &str, key: &str, default: T) -> Result<T, ConfigError>
    where
        T::Err: std::error::Error + Send + Sync + 'static,
    {
        let var_name = format!("{prefix}_{key}");
        match env::var(&var_name) {
            Ok(val) => val.parse().map_err(|e| ConfigError::ParseError {
                key: var_name,
                source: Box::new(e),
            }),
            Err(_) => Ok(default),
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            tls_enabled: false,
            tls_cert_path: None,
            tls_key_path: None,
            api_key: None,
            rate_limit_enabled: true,
            rate_limit_rpm: 100,
        }
    }
}
