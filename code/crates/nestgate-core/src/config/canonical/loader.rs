//! Configuration Loading
//!
//! This module handles file I/O and parsing operations for configuration.
//! Single responsibility: Load configuration from various sources.

use super::types::CanonicalConfig;
use crate::{NestGateError, Result};
use std::path::PathBuf;

/// Configuration loader with support for multiple sources
pub struct ConfigLoader;

impl ConfigLoader {
    /// Load configuration from a TOML file
    pub fn load_from_file(path: &PathBuf) -> Result<CanonicalConfig> {
        let content = std::fs::read_to_string(path).map_err(|e| NestGateError::Io {
            message: format!("Failed to read config file: {e}"),
            operation: "load_config".to_string(),
            path: Some(path.to_string_lossy().to_string()),
            retryable: false,
            context: None,
        })?;

        Self::parse_toml(&content)
    }

    /// Load configuration from a TOML string
    pub fn load_from_string(content: &str) -> Result<CanonicalConfig> {
        Self::parse_toml(content)
    }

    /// Load configuration from environment variables
    pub fn load_from_env() -> Result<CanonicalConfig> {
        // This would implement environment variable loading
        // For now, return a default config
        super::defaults::CanonicalConfigBuilder::new().build()
    }

    /// Parse TOML content into configuration
    fn parse_toml(content: &str) -> Result<CanonicalConfig> {
        let config: CanonicalConfig =
            toml::from_str(content).map_err(|e| NestGateError::Validation {
                field: Some("config_file".to_string()),
                message: format!("Invalid configuration format: {e}"),
                current_value: None,
                expected: Some("valid TOML".to_string()),
                context: None,
            })?;

        // Validate the loaded configuration
        super::validation::ConfigValidator::validate(&config)?;
        Ok(config)
    }

    /// Save configuration to a TOML file
    pub fn save_to_file(config: &CanonicalConfig, path: &PathBuf) -> Result<()> {
        // Validate before saving
        super::validation::ConfigValidator::validate(config)?;

        let content = toml::to_string_pretty(config).map_err(|e| NestGateError::Internal {
            message: format!("Failed to serialize configuration: {e}"),
            location: Some(format!("ConfigLoader::save_to_file - Path: {}", path.display())),
            is_bug: false,
            context: None,
        })?;

        std::fs::write(path, content).map_err(|e| NestGateError::Io {
            message: format!("Failed to write config file: {e}"),
            operation: "save_config".to_string(),
            path: Some(path.to_string_lossy().to_string()),
            retryable: true,
            context: None,
        })?;

        Ok(())
    }

    /// Load configuration with fallback sources
    pub fn load_with_fallback(
        primary_path: &PathBuf,
        fallback_paths: &[PathBuf],
    ) -> Result<CanonicalConfig> {
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
