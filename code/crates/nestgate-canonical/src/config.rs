//! Canonical Configuration System for `NestGate`,
//!
//! Unified configuration that replaces all fragmented config structs
//! across the ecosystem with a single, canonical configuration hierarchy.

use std::path::Path;

use crate::error::{NestGateError, Result};
use crate::types::CanonicalConfig;

/// Canonical `NestGate` Configuration,
///
/// This replaces all fragmented configuration structs with a unified system
pub type NestGateConfig = CanonicalConfig;
impl NestGateConfig {
    /// Load configuration from environment variables
    /// Creates a canonical configuration from environment variables.
    ///
    /// Currently returns the default configuration but will be extended
    /// to read from environment variables in future versions.
    #[must_use]
    pub fn from_env() -> Self {
        Self::default()
    }

    /// Load configuration from file
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            NestGateError::validation(
                format!("Failed to read config file: {}", e)
            )
        })?;

        let config: Self = toml::from_str(&content).map_err(|e| {
            NestGateError::validation(
                format!("Failed to parse config: {}", e)
            )
        })?;

        config.validate()?;
        Ok(config)
    }

    /// Validate the configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn validate(&self) -> Result<()> {
        if self.service_name.is_empty() {
            return Err(NestGateError::validation(
                "Service name cannot be empty"
            ));
        }

        if self.network.port == 0 {
            return Err(NestGateError::validation("Port cannot be zero"));
        }

        if self.storage.data_directory.is_empty() {
            return Err(NestGateError::validation(
                "Data directory cannot be empty"
            ));
        }

        Ok(())
    }
}
