// **PHASE 2C SUPPORTING TYPES**
//! Supporting types and enums for the Phase 2C configuration unification enhancements.
//! Phase2c Types functionality and utilities.
//! This module contains the core type definitions used throughout the Phase 2C
//! configuration system, including environment types, tier definitions, and
//! operational enumerations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ==================== ENVIRONMENT TYPES ====================

/// **ENVIRONMENT ENUMERATION**
///
/// Defines the deployment environment for configuration validation and behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    /// Development environment - relaxed validation and debugging features
    Development,

    /// Staging environment - production-like but with additional testing features
    Staging,

    /// Production environment - strict validation and optimized for performance
    Production,

    /// Testing environment - specialized for automated testing
    Testing,
}
impl Default for Environment {
    fn default() -> Self {
        Self::Development
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Development => write!(f, "development"),
            Environment::Staging => write!(f, "staging"),
            Environment::Production => write!(f, "production"),
            Environment::Testing => write!(f, "testing"),
        }
    }
}

impl std::str::FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Ok(Environment::Development),
            "staging" | "stage" => Ok(Environment::Staging),
            "production" | "prod" => Ok(Environment::Production),
            "testing" | "test" => Ok(Environment::Testing),
            _ => Err(format!("Unknown environment: {s}")),
        }
    }
}

// ==================== CONFIGURATION OVERRIDES ====================

/// **CONFIGURATION OVERRIDES**
///
/// Allows runtime overriding of configuration values
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigOverrides {
    /// Override the environment
    pub environment: Option<Environment>,

    /// Domain-specific configuration overrides
    pub domain_overrides: Option<HashMap<String, serde_json::Value>>,

    /// Network configuration overrides
    pub network_overrides: Option<NetworkOverrides>,

    /// Security configuration overrides
    pub security_overrides: Option<SecurityOverrides>,

    /// Performance configuration overrides
    pub performance_overrides: Option<PerformanceOverrides>,
}

/// **NETWORK CONFIGURATION OVERRIDES**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkOverrides {
    /// Override API port
    pub api_port: Option<u16>,

    /// Override bind address
    pub bind_address: Option<String>,

    /// Override timeout settings
    pub timeout_ms: Option<u64>,

    /// Override worker count
    pub workers: Option<usize>,
}
/// **SECURITY CONFIGURATION OVERRIDES**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityOverrides {
    /// Override TLS enabled state
    pub tls_enabled: Option<bool>,

    /// Override authentication requirement
    pub require_auth: Option<bool>,

    /// Override development mode bypass
    pub dev_mode_bypass: Option<bool>,

    /// Override certificate path
    pub cert_path: Option<String>,
}
/// **PERFORMANCE CONFIGURATION OVERRIDES**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceOverrides {
    /// Override maximum connections
    pub max_connections: Option<usize>,

    /// Override buffer size
    pub buffer_size: Option<usize>,

    /// Override cache size
    pub cache_size: Option<usize>,

    /// Override optimization level
    pub optimization_level: Option<OptimizationLevel>,
}
/// **OPTIMIZATION LEVEL**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptimizationLevel {
    /// Minimal optimization - fastest compilation, slower runtime
    Debug,

    /// Balanced optimization - good compromise
    Balanced,

    /// Maximum optimization - slower compilation, fastest runtime
    Performance,

    /// Size optimization - optimized for binary size
    Size,
}
impl Default for OptimizationLevel {
    fn default() -> Self {
        Self::Balanced
    }
}

// ==================== CONFIGURATION METADATA ====================

/// **CONFIGURATION METADATA**
///
/// Metadata about the configuration instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMetadata {
    /// Configuration version
    pub version: String,

    /// Configuration creation timestamp
    pub created_at: std::time::SystemTime,

    /// Last modification timestamp
    pub modified_at: std::time::SystemTime,

    /// Configuration source (file, environment, default, etc.)
    pub source: ConfigSource,

    /// Additional metadata
    pub additional_metadata: HashMap<String, String>,
}
impl Default for ConfigMetadata {
    fn default() -> Self {
        let now = std::time::SystemTime::now();
        Self {
            version: "2.0.0".to_string(),
            created_at: now,
            modified_at: now,
            source: ConfigSource::Default,
            additional_metadata: HashMap::new(),
        }
    }
}

/// **CONFIGURATION SOURCE**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfigSource {
    /// Default configuration
    Default,

    /// Loaded from configuration file
    File(String),

    /// Loaded from environment variables
    Environment,

    /// Loaded from database
    Database,

    /// Loaded from remote service
    Remote(String),

    /// Merged from multiple sources
    Merged(Vec<ConfigSource>),
}
// ==================== FEATURE FLAGS ====================

/// **FEATURE FLAGS**
///
/// Runtime feature toggles for the configuration system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    /// Enable experimental features
    pub experimental_features: bool,

    /// Enable debug logging
    pub debug_logging: bool,

    /// Enable performance monitoring
    pub performance_monitoring: bool,

    /// Enable configuration hot-reload
    pub hot_reload: bool,

    /// Enable configuration validation
    pub strict_validation: bool,

    /// Custom feature flags
    pub custom_flags: HashMap<String, bool>,
}
impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            experimental_features: false,
            debug_logging: cfg!(debug_assertions),
            performance_monitoring: true,
            hot_reload: cfg!(debug_assertions),
            strict_validation: !cfg!(debug_assertions),
            custom_flags: HashMap::new(),
        }
    }
}

impl FeatureFlags {
    /// Check if a feature is enabled
    #[must_use]
    pub fn is_enabled(&self, feature: &str) -> bool {
        match feature {
            "experimental_features" => self.experimental_features,
            "debug_logging" => self.debug_logging,
            "performance_monitoring" => self.performance_monitoring,
            "hot_reload" => self.hot_reload,
            "strict_validation" => self.strict_validation,
            _ => self.custom_flags.get(feature).copied().unwrap_or(false),
        }
    }

    /// Enable a feature
    pub fn enable(&mut self, feature: &str) {
        match feature {
            "experimental_features" => self.experimental_features = true,
            "debug_logging" => self.debug_logging = true,
            "performance_monitoring" => self.performance_monitoring = true,
            "hot_reload" => self.hot_reload = true,
            "strict_validation" => self.strict_validation = true,
            _ => {
                self.custom_flags.insert(feature.to_string(), true);
            }
        }
    }

    /// Disable a feature
    pub fn disable(&mut self, feature: &str) {
        match feature {
            "experimental_features" => self.experimental_features = false,
            "debug_logging" => self.debug_logging = false,
            "performance_monitoring" => self.performance_monitoring = false,
            "hot_reload" => self.hot_reload = false,
            "strict_validation" => self.strict_validation = false,
            _ => {
                self.custom_flags.insert(feature.to_string(), false);
            }
        }
    }
}

// ==================== VALIDATION TYPES ====================

/// **VALIDATION WARNING**
///
/// Warning message from configuration validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    /// Warning message
    pub message: String,

    /// Configuration field that generated the warning
    pub field: Option<String>,

    /// Warning severity
    pub severity: WarningSeverity,

    /// Suggested resolution
    pub suggested_resolution: Option<String>,
}
/// **WARNING SEVERITY**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarningSeverity {
    /// Low severity - informational
    Info,

    /// Medium severity - should be addressed
    Warning,

    /// High severity - should be addressed urgently
    High,

    /// Critical - may cause issues
    Critical,
}
// ==================== UTILITY IMPLEMENTATIONS ====================

impl Environment {
    /// Check if this is a production-like environment
    #[must_use]
    pub fn is_production_like(&self) -> bool {
        matches!(self, Self::Production | Self::Staging)
    }

    /// Check if this is a development-like environment
    #[must_use]
    pub fn is_development_like(&self) -> bool {
        matches!(self, Environment::Development | Environment::Testing)
    }

    /// Get default port for this environment
    #[must_use]
    pub fn default_port(&self) -> u16 {
        match self {
            Environment::Production => 443,
            Environment::Staging => 8443,
            Environment::Development => 8080,
            Environment::Testing => 3000,
        }
    }

    /// Get default optimization level for this environment
    #[must_use]
    pub fn default_optimization(&self) -> OptimizationLevel {
        match self {
            Environment::Production => OptimizationLevel::Performance,
            Environment::Staging => OptimizationLevel::Balanced,
            Environment::Development => OptimizationLevel::Debug,
            Environment::Testing => OptimizationLevel::Debug,
        }
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_from_str() {
        assert_eq!(
            "development".parse::<Environment>().unwrap(),
            Environment::Development
        );
        assert_eq!(
            "production".parse::<Environment>().unwrap(),
            Environment::Production
        );
        assert_eq!(
            "staging".parse::<Environment>().unwrap(),
            Environment::Staging
        );
        assert_eq!(
            "testing".parse::<Environment>().unwrap(),
            Environment::Testing
        );
    }

    #[test]
    fn test_environment_default_port() {
        assert_eq!(Environment::Production.default_port(), 443);
        assert_eq!(Environment::Development.default_port(), 8080);
        assert_eq!(Environment::Staging.default_port(), 8443);
        assert_eq!(Environment::Testing.default_port(), 3000);
    }

    #[test]
    fn test_feature_flags() {
        let mut flags = FeatureFlags::default();
        assert!(!flags.is_enabled("experimental_features"));

        flags.enable("experimental_features");
        assert!(flags.is_enabled("experimental_features"));

        flags.disable("experimental_features");
        assert!(!flags.is_enabled("experimental_features"));
    }

    #[test]
    fn test_config_metadata_default() {
        let metadata = ConfigMetadata::default();
        assert_eq!(metadata.version, "2.0.0");
        assert_eq!(metadata.source, ConfigSource::Default);
    }
}
