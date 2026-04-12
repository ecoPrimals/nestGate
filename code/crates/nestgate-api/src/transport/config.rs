// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **TRANSPORT CONFIGURATION**
//!
//! Environment-driven configuration for TRUE PRIMAL transport.

use nestgate_core::error::{NestGateError, Result};
use nestgate_types::{EnvSource, ProcessEnv};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// **TRANSPORT CONFIGURATION**
///
/// Configuration for Unix socket + JSON-RPC transport.
///
/// ## Environment Variables
///
/// - `NESTGATE_FAMILY_ID`: Family identifier (default: "default")
/// - `NESTGATE_SOCKET_PATH`: Unix socket path (default: `/tmp/nestgate-{family}.sock`)
/// - `NESTGATE_SECURITY_PROVIDER`: Security provider socket path (discovered at runtime)
/// - `NESTGATE_HTTP_PORT`: Optional HTTP fallback port (default: None)
///
/// ## Examples
///
/// ```rust,ignore
/// // From environment
/// let config = TransportConfig::from_env()?;
///
/// // With explicit values
/// let config = TransportConfig::new("nat0")
///     .with_socket_path("/tmp/nestgate-nat0.sock")
///     .with_security_provider("/run/user/1000/security-nat0.sock");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportConfig {
    /// Family ID for this primal instance
    pub family_id: String,

    /// Unix socket path for primary transport
    pub socket_path: PathBuf,

    /// Security provider socket path (capability-based, not primal-specific)
    pub security_provider: PathBuf,

    /// Optional HTTP fallback port
    pub http_port: Option<u16>,

    /// Enable verbose logging
    pub verbose: bool,
}

impl TransportConfig {
    /// Create configuration from environment variables
    ///
    /// # Errors
    ///
    /// Returns error if environment is misconfigured
    pub fn from_env() -> Result<Self> {
        Self::from_env_source(&ProcessEnv)
    }

    /// Like [`Self::from_env`], but reads transport variables from `env`.
    pub fn from_env_source(env: &(impl EnvSource + ?Sized)) -> Result<Self> {
        let family_id = env
            .get("NESTGATE_FAMILY_ID")
            .unwrap_or_else(|| "default".to_string());

        let socket_path = env
            .get("NESTGATE_SOCKET_PATH")
            .unwrap_or_else(|| format!("/tmp/nestgate-{family_id}.sock"));

        let security_slug = env
            .get("NESTGATE_SECURITY_SLUG")
            .unwrap_or_else(|| "security".to_string());
        let security_provider = env
            .get("NESTGATE_SECURITY_PROVIDER")
            .unwrap_or_else(|| format!("/tmp/{security_slug}-{family_id}-default.sock"));

        let http_port = env.get("NESTGATE_HTTP_PORT").and_then(|s| s.parse().ok());

        let verbose = env
            .get("NESTGATE_VERBOSE")
            .is_some_and(|v| v == "1" || v.to_lowercase() == "true");

        Ok(Self {
            family_id,
            socket_path: PathBuf::from(socket_path),
            security_provider: PathBuf::from(security_provider),
            http_port,
            verbose,
        })
    }

    /// Create new configuration with family ID
    #[must_use]
    pub fn new(family_id: impl Into<String>) -> Self {
        let family_id = family_id.into();
        let security_slug =
            std::env::var("NESTGATE_SECURITY_SLUG").unwrap_or_else(|_| "security".to_string());
        Self {
            socket_path: PathBuf::from(format!("/tmp/nestgate-{family_id}.sock")),
            security_provider: PathBuf::from(format!(
                "/tmp/{security_slug}-{family_id}-default.sock"
            )),
            family_id,
            http_port: None,
            verbose: false,
        }
    }

    /// Set Unix socket path
    #[must_use]
    pub fn with_socket_path(mut self, path: impl AsRef<Path>) -> Self {
        self.socket_path = path.as_ref().to_path_buf();
        self
    }

    /// Set security provider socket path
    #[must_use]
    pub fn with_security_provider(mut self, path: impl AsRef<Path>) -> Self {
        self.security_provider = path.as_ref().to_path_buf();
        self
    }

    /// Enable HTTP fallback on specified port
    #[must_use]
    pub const fn with_http_fallback(mut self, port: u16) -> Self {
        self.http_port = Some(port);
        self
    }

    /// Enable verbose logging
    #[must_use]
    pub const fn with_verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// Validate configuration
    ///
    /// # Errors
    ///
    /// Returns error if configuration is invalid
    pub fn validate(&self) -> Result<()> {
        // Validate socket path
        if self.socket_path.as_os_str().is_empty() {
            return Err(NestGateError::api_error("Socket path cannot be empty"));
        }

        // Validate security provider path
        if self.security_provider.as_os_str().is_empty() {
            return Err(NestGateError::api_error(
                "Security provider path cannot be empty",
            ));
        }

        // Validate HTTP port if specified
        if let Some(port) = self.http_port
            && port == 0
        {
            return Err(NestGateError::api_error("HTTP port cannot be 0"));
        }

        Ok(())
    }
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self::new("default")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = TransportConfig::new("nat0");
        assert_eq!(config.family_id, "nat0");
        assert_eq!(config.socket_path, PathBuf::from("/tmp/nestgate-nat0.sock"));
        assert!(config.http_port.is_none());
    }

    #[test]
    fn test_config_with_http_fallback() {
        let config = TransportConfig::new("nat0").with_http_fallback(8080);
        assert_eq!(config.http_port, Some(8080));
    }

    #[test]
    fn test_config_validation() {
        let config = TransportConfig::new("test");
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_from_env() {
        let orig = std::env::var("NESTGATE_FAMILY_ID").ok();
        nestgate_core::env_process::set_var("NESTGATE_FAMILY_ID", "test123");
        let config = TransportConfig::from_env().unwrap();
        match orig {
            Some(v) => nestgate_core::env_process::set_var("NESTGATE_FAMILY_ID", v),
            None => nestgate_core::env_process::remove_var("NESTGATE_FAMILY_ID"),
        }
        assert_eq!(config.family_id, "test123");
    }
}
