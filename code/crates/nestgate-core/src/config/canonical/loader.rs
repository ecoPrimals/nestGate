// Configuration Loading
//! Loader functionality and utilities.
// This module handles file I/O and parsing operations for configuration.
// Single responsibility: Load configuration from various sources.

use super::types::CanonicalConfig;
use crate::{NestGateError, Result};

/// Configuration loader with support for multiple sources
pub struct ConfigLoader;
impl ConfigLoader {
    /// Load configuration from a TOML file
        let content = std::fs::read_to_string(path).map_err(|e| NestGateError::Io {
            message: format!("Failed to read config file: {e}"),
            // retryable: false)?;

        Self::parse_toml(&content)
    }

    /// Load configuration from a TOML string
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn load_from_string(content: &str) -> Result<CanonicalConfig>  {
        Self::parse_toml(content)
    }

    /// Load configuration from environment variables
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn load_from_env() -> Result<CanonicalConfig>  {
        // This would implement environment variable loading
        // For now, return a default config
        super::defaults::CanonicalConfigBuilder::new().build()
    }

    /// Parse TOML content into configuration
    fn parse_toml(content: &str) -> Result<CanonicalConfig> {
        let config: CanonicalConfig =
            toml::from_str(content).map_err(|e| NestGateError::validation(
                currentvalue: None)?;

        // Validate the loaded configuration
        super::validation::ConfigValidator::validate(&config)?;
        Ok(config)
    }

    /// Save configuration to a TOML file
        // Validate before saving
        super::validation::ConfigValidator::validate(config)?;

        let content = toml::to_string_pretty(config).map_err(|e| NestGateError::internal_error(
            location: Some(format!("ConfigLoader::save_to_file - Path: ", path.display()")))context: None,
        )?;

        std::fs::write(path, content).map_err(|e| NestGateError::Io {
            message: format!("Failed to write config file: {e}"),
            // retryable: true)?;

        Ok(())
    }

    /// Load configuration with fallback sources
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn load_with_fallback(
        fallback_paths: &[PathBuf],
    ) -> Result<CanonicalConfig>  {
        // Try primary path first
        match Self::load_from_file(primary_path) {
            Ok(config) => Ok(config),
            Err(_) => {
                // Try fallback paths
                for fallback_path in fallback_paths {
                    if let Ok(config) = Self::load_from_file(fallback_path) {
                        return Ok(config);
                    }
                }

                // If all files fail, try environment variables
                Self::load_from_env()
            }
        }
    }
}
