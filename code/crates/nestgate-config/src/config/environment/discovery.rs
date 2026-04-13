// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Discovery Configuration
//!
//! Service discovery environment configuration extracted for logical cohesion.
//!
//! **Phase 3: Smart Refactoring** - Extracted from monolithic `environment.rs` (Jan 30, 2026)

use nestgate_types::{EnvSource, ProcessEnv};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::ConfigError;

/// Service discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Enable service discovery (default: true)
    pub enabled: bool,

    /// Discovery interval in seconds (default: 30)
    pub interval_secs: u64,

    /// Discovery timeout in seconds (default: 5)
    pub timeout_secs: u64,

    /// Retry attempts (default: 3)
    pub retry_attempts: u32,

    /// Cache discovered services (default: true)
    pub cache_enabled: bool,

    /// Cache TTL in seconds (default: 300)
    pub cache_ttl_secs: u64,
}

impl DiscoveryConfig {
    /// Load from environment with NESTGATE_ prefix
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_env_with_prefix("NESTGATE")
    }

    /// Load from an injectable environment source (e.g. [`nestgate_types::MapEnv`] in tests).
    pub fn from_env_source(env: &(impl EnvSource + ?Sized)) -> Result<Self, ConfigError> {
        Self::from_env_with_prefix_source("NESTGATE", env)
    }

    /// Load from environment with custom prefix
    pub fn from_env_with_prefix(prefix: &str) -> Result<Self, ConfigError> {
        Self::from_env_with_prefix_source(prefix, &ProcessEnv)
    }

    /// Load with custom prefix from an injectable [`EnvSource`].
    pub fn from_env_with_prefix_source(
        prefix: &str,
        env: &(impl EnvSource + ?Sized),
    ) -> Result<Self, ConfigError> {
        Ok(Self {
            enabled: Self::env_var_or(prefix, "DISCOVERY_ENABLED", true, env)?,
            interval_secs: Self::env_var_or(prefix, "DISCOVERY_INTERVAL", 30, env)?,
            timeout_secs: Self::env_var_or(prefix, "DISCOVERY_TIMEOUT", 5, env)?,
            retry_attempts: Self::env_var_or(prefix, "DISCOVERY_RETRIES", 3, env)?,
            cache_enabled: Self::env_var_or(prefix, "DISCOVERY_CACHE", true, env)?,
            cache_ttl_secs: Self::env_var_or(prefix, "DISCOVERY_CACHE_TTL", 300, env)?,
        })
    }

    /// Helper to get environment variable or use default
    fn env_var_or<T: FromStr>(
        prefix: &str,
        key: &str,
        default: T,
        env: &(impl EnvSource + ?Sized),
    ) -> Result<T, ConfigError>
    where
        T::Err: std::error::Error + Send + Sync + 'static,
    {
        let var_name = format!("{prefix}_{key}");
        match env.get(&var_name) {
            Some(val) => val.parse::<T>().map_err(|e| ConfigError::ParseError {
                key: var_name,
                detail: e.to_string(),
            }),
            None => Ok(default),
        }
    }
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_secs: 30,
            timeout_secs: 5,
            retry_attempts: 3,
            cache_enabled: true,
            cache_ttl_secs: 300,
        }
    }
}
