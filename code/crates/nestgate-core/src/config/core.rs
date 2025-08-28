//! **CORE CONFIGURATION TYPES**
//!
//! This module provides the core configuration structures for NestGate,
//! including the main NestGateCanonicalConfig struct and system-level configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
// Duration import removed - unused

use crate::{NestGateError, Result};
use super::domains::{
    ApiConfig, ServerConfig, StorageConfig, NetworkConfig, SecurityConfig,
    MonitoringConfig, EnvironmentConfig, McpConfig
};

// ==================== SECTION ====================

/// **ENHANCED UNIFIED CONFIGURATION WITH CONST GENERICS**
/// 
/// This provides compile-time configuration optimization while maintaining
/// runtime flexibility for production deployments.
/// 
/// **PERFORMANCE**: Zero-cost configuration access through const generics
/// **FLEXIBILITY**: Runtime configuration override capability maintained
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestGateCanonicalConfig<
    const MAX_CONNECTIONS: usize = 1000,
    const BUFFER_SIZE: usize = 65536,
    const TIMEOUT_MS: u64 = 30000,
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
    pub environment: EnvironmentConfig,
    /// MCP (Model Context Protocol) configuration
    pub mcp: McpConfig,
    /// Feature flags
    pub features: HashMap<String, bool>,
    /// Environment-specific overrides
    pub environment_overrides: HashMap<String, String>,
}

/// **CONST GENERIC SYSTEM CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig<
    const MAX_CONNECTIONS: usize = 1000,
    const BUFFER_SIZE: usize = 65536,
> {
    pub service_name: String,
    pub version: String,
    pub environment: String,
    pub log_level: LogLevel,
    pub debug_mode: bool,
    pub metrics_enabled: bool,
    pub tracing_enabled: bool,
    pub max_concurrent_requests: Option<usize>, // Runtime override for MAX_CONNECTIONS
    pub buffer_size_override: Option<usize>,    // Runtime override for BUFFER_SIZE
    pub data_dir: PathBuf,
    pub config_dir: PathBuf,
}

impl<const MAX_CONNECTIONS: usize, const BUFFER_SIZE: usize> SystemConfig<MAX_CONNECTIONS, BUFFER_SIZE> {
    /// Get max connections - compile-time optimized
    pub const fn max_connections() -> usize {
        MAX_CONNECTIONS
    }

    /// Get buffer size - compile-time optimized  
    pub const fn buffer_size() -> usize {
        BUFFER_SIZE
    }

    /// Get effective max connections (runtime override or compile-time)
    pub fn effective_max_connections(&self) -> usize {
        self.max_concurrent_requests.unwrap_or(MAX_CONNECTIONS)
    }

    /// Get effective buffer size (runtime override or compile-time)
    pub fn effective_buffer_size(&self) -> usize {
        self.buffer_size_override.unwrap_or(BUFFER_SIZE)
    }
}

impl<const MAX_CONNECTIONS: usize, const BUFFER_SIZE: usize> Default for SystemConfig<MAX_CONNECTIONS, BUFFER_SIZE> {
    fn default() -> Self {
        Self {
            service_name: "nestgate".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: "development".to_string(),
            log_level: LogLevel::Info,
            debug_mode: false,
            metrics_enabled: true,
            tracing_enabled: true,
            max_concurrent_requests: None,
            buffer_size_override: None,
            data_dir: PathBuf::from("./data"),
            config_dir: PathBuf::from("./config"),
        }
    }
}

/// Log level enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

// ==================== SECTION ====================

impl NestGateCanonicalConfig {
    /// Create production configuration
    pub fn production() -> Self {
        let mut config = Self::default();
        config.system.environment = "production".to_string();
        config.system.log_level = LogLevel::Warn;
        config.system.debug_mode = false;
        config
    }

    /// Create development configuration
    pub fn development() -> Self {
        let mut config = Self::default();
        config.system.environment = "development".to_string();
        config.system.log_level = LogLevel::Debug;
        config.system.debug_mode = true;
        config
    }

    /// Validate configuration consistency
    pub fn validate(&self) -> Result<()> {
        // Validate system configuration
        if self.system.service_name.is_empty() {
            return Err(NestGateError::Configuration {
                field: "system.service_name".to_string(),
                message: "Service name cannot be empty".to_string(),
                current_value: Some(self.system.service_name.clone()),
                expected: Some("non-empty string".to_string()),
                user_error: true,
            });
        }

        // Validate data directory exists or can be created
        if !self.system.data_dir.exists() {
            std::fs::create_dir_all(&self.system.data_dir)
                .map_err(|e| NestGateError::Configuration {
                    field: "system.data_dir".to_string(),
                    message: format!("Cannot create data directory: {}", e),
                    current_value: Some(self.system.data_dir.display().to_string()),
                    expected: Some("writable directory path".to_string()),
                    user_error: true,
                })?;
        }

        // Additional validations can be added here
        Ok(())
    }

    /// Load configuration from environment variables
    pub fn from_environment() -> Result<Self> {
        // Implementation for environment variable loading
        // This would use the existing environment loading patterns
        Ok(Self::default())
    }

    /// Load configuration from TOML file
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| NestGateError::Configuration {
                field: "file_path".to_string(),
                message: format!("Cannot read config file: {}", e),
                current_value: Some(path.display().to_string()),
                expected: Some("readable file path".to_string()),
                user_error: true,
            })?;

        toml::from_str(&content)
            .map_err(|e| NestGateError::Configuration {
                field: "toml_content".to_string(),
                message: format!("Invalid TOML configuration: {}", e),
                current_value: Some(content.chars().take(100).collect::<String>() + "..."),
                expected: Some("valid TOML format".to_string()),
                user_error: true,
            })
    }
}

impl Default for NestGateCanonicalConfig {
    fn default() -> Self {
        Self {
            system: SystemConfig::default(),
            api: ApiConfig::default(),
            server: ServerConfig::default(),
            storage: StorageConfig::default(),
            network: NetworkConfig::default(),
            security: SecurityConfig::default(),
            monitoring: MonitoringConfig::default(),
            environment: EnvironmentConfig::default(),
            mcp: McpConfig::default(),
            features: HashMap::new(),
            environment_overrides: HashMap::new(),
        }
    }
} 