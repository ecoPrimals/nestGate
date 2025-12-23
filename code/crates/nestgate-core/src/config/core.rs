// **CORE CONFIGURATION TYPES**
//! Configuration types and utilities.
// This module provides the core configuration structures for NestGate,
//! including the main NestGateCanonicalConfig struct and system-level configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Duration import removed - unused

use crate::{NestGateError, Result};
use super::domains::{
    ApiConfig, ServerConfig, StorageConfig, NetworkConfig, SecurityConfig,
    MonitoringConfig, EnvironmentConfig, McpConfig
};
use super::canonical_primary::SystemConfig;

// ==================== SECTION ====================

/// **ENHANCED UNIFIED CONFIGURATION WITH CONST GENERICS**
/// 
/// This provides compile-time configuration optimization while maintaining
/// runtime flexibility for production deployments.
/// 
/// **PERFORMANCE**: Zero-cost configuration access through const generics
/// **FLEXIBILITY**: Runtime configuration override capability maintained
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for NestGateCanonical
pub struct NestGateCanonicalConfig<
    const MAX_CONNECTIONS: usize = 1000,
    const BUFFER_SIZE: usize = 65536,
    const TIMEOUT_MS: u64 = 30000,
    /// Api Port
    const API_PORT: u16 = 8080,
> {
    /// System configuration
    pub system: SystemConfig<MAX_CONNECTIONS, BUFFER_SIZE>,
    /// API configuration
    pub api: ApiConfig<API_PORT, TIMEOUT_MS>,
    /// Server configuration
    pub server: ServerConfig<MAX_CONNECTIONS>,
    /// Storage configuration
    pub storage: StorageConfig,
    /// Network configuration
    pub network: NetworkConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// Monitoring configuration
    pub monitoring: MonitoringConfig,
    /// Environment configuration
    /// MCP (Model Context Protocol) configuration
    pub mcp: McpConfig,
    /// Feature flags
    pub features: HashMap<String, bool>,
    /// Environment-specific overrides
    pub environment_overrides: HashMap<String, String>,
}
// SystemConfig is now imported from canonical_primary

/// Log level enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Loglevel
pub enum LogLevel {
    /// Error
    Error,
    /// Warn
    Warn,
    /// Info
    Info,
    /// Debug
    Debug,
    /// Trace
    Trace,
}
impl Default for LogLevel {
    /// Returns the default instance
    fn default() -> Self {
        LogLevel::Info
    }
}

// ==================== SECTION ====================

impl NestGateCanonicalConfig {
    /// Create production configuration
    #[must_use]
    pub fn production() -> Self {
        let mut config = Self::default();
        config.system.environment = "production".to_string();
        config.system.log_level = LogLevel::Warn;
        config.system.debug_mode = false;
        config
    }

    /// Create development configuration
    #[must_use]
    pub fn development() -> Self {
        let mut config = Self::default();
        config.system.environment = "development".to_string();
        config.system.log_level = LogLevel::Debug;
        config.system.debug_mode = true;
        config
    }

    /// Validate configuration consistency
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn validate(&self) -> Result<()>  {
        // Validate system configuration
        if self.system.instance_name.is_empty() {
            return Err(NestGateError::configuration_error_detailed(Some("field".to_string()), "Service name cannot be empty".to_string(), Some(self.system.instance_name.clone()), Some("non-empty string".to_string()), true));
        }

        // Validate data directory exists or can be created
        if !self.system.data_dir.exists() {
            std::fs::create_dir_all(&self.system.data_dir)
                .map_err(|e| NestGateError::configuration_error(Some("field".to_string()), format!("Cannot create data directory: {}"))?;
        }

        // Additional validations can be added here
        Ok(())
    }

    /// Load configuration from environment variables
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn from_environment() -> Result<Self>  {
        // Implementation for environment variable loading
        // This would use the existing environment loading patterns
        Ok(Self::default())
    }

    /// Load configuration from TOML file
        let content = std::fs::read_to_string(path)
            .map_err(|e| NestGateError::configuration_error(Some("field".to_string()), format!("Cannot read config file: {}"))?;

        toml::from_str(&content)
            .map_err(|e| NestGateError::configuration_error(Some("field".to_string()), format!("Invalid TOML configuration: {}"))
    }
}

impl Default for NestGateCanonicalConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            system: SystemConfig::default(),
            api: ApiConfig::default(),
            server: ServerConfig::default(),
            storage: StorageConfig::default(),
            network: NetworkConfig::default(),
            security: SecurityConfig::default(),
            monitoring: MonitoringConfig::default(),
            mcp: McpConfig::default(),
            features: HashMap::new(),
            environment_overrides: HashMap::new(),
        }
    }
} 