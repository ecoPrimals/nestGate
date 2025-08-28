/// 
/// This module provides a unified interface to all handler configurations
/// by re-exporting types from the split modules and providing convenience functions.
///
/// **ARCHITECTURE**:
/// - `handler_common` - Common configuration types shared by all handlers
/// - `handler_types` - Handler-specific configuration structures
/// - This module - Integration and convenience functions

// Re-export all types from split modules
pub use super::handler_common::*;
pub use super::handler_types::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ==================== SECTION ====================

/// Configuration builder for creating handler configurations with defaults
pub struct HandlerConfigBuilder<T> {
    common: CommonHandlerConfig,
    specific: Option<T>,
}

impl<T> HandlerConfigBuilder<T> {
    /// Create a new builder with default common configuration
    pub fn new() -> Self {
        Self {
            common: CommonHandlerConfig::default(),
            specific: None,
        }
    }

    /// Set the handler name
    pub fn with_name(mut self, name: String) -> Self {
        self.common.handler_name = name;
        self
    }

    /// Enable or disable the handler
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.common.enabled = enabled;
        self
    }

    /// Set the request timeout
    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.common.timeout = timeout;
        self
    }

    /// Set maximum concurrent requests
    pub fn with_max_concurrent(mut self, max: usize) -> Self {
        self.common.max_concurrent_requests = max;
        self
    }

    /// Set handler-specific configuration
    pub fn with_specific(mut self, specific: T) -> Self {
        self.specific = Some(specific);
        self
    }

    /// Build the final configuration
    pub fn build(self) -> Result<HandlerConfig<T>, ConfigError>
    where
        T: Default,
    {
        let specific = self.specific.unwrap_or_default();
        let extensions = HandlerExtensions {
            common: self.common,
            specific,
        };
        
        // Use the StandardDomainConfig pattern
        Ok(HandlerConfig::new(extensions))
    }
}

impl<T> Default for HandlerConfigBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== SECTION ====================

/// Configuration validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigError {
    pub field: String,
    pub message: String,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Configuration error in {}: {}", self.field, self.message)
    }
}

impl std::error::Error for ConfigError {}

/// Trait for validating handler configurations
pub trait ConfigValidator {
    /// Validate the configuration and return any errors
    fn validate(&self) -> Vec<ConfigError>;

    /// Check if the configuration is valid
    fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }
}

// Implement validation for common handler config
impl ConfigValidator for CommonHandlerConfig {
    fn validate(&self) -> Vec<ConfigError> {
        let mut errors = Vec::new();

        if self.handler_name.is_empty() {
            errors.push(ConfigError {
                field: Some("handler_name".to_string()),
                message: "Handler name cannot be empty".to_string(),
            });
        }

        if self.max_concurrent_requests == 0 {
            errors.push(ConfigError {
                field: Some("max_concurrent_requests".to_string()),
                message: "Max concurrent requests must be greater than 0".to_string(),
            });
        }

        if self.timeout.as_secs() == 0 {
            errors.push(ConfigError {
                field: Some("timeout".to_string()),
                message: "Timeout must be greater than 0".to_string(),
            });
        }

        errors
    }
}

// ==================== SECTION ====================

/// Factory functions for creating pre-configured handler configs
pub struct HandlerConfigFactory;

impl HandlerConfigFactory {
    /// Create a ZFS handler configuration with sensible defaults
    pub fn zfs_handler(name: String) -> HandlerConfigBuilder<ZfsHandlerSpecificConfig> {
        HandlerConfigBuilder::new()
            .with_name(name)
            .with_timeout(std::time::Duration::from_secs(60)) // ZFS operations can be slow
            .with_max_concurrent(50) // Conservative for ZFS operations
    }

    /// Create a performance handler configuration
    pub fn performance_handler(name: String) -> HandlerConfigBuilder<PerformanceHandlerSpecificConfig> {
        HandlerConfigBuilder::new()
            .with_name(name)
            .with_timeout(std::time::Duration::from_secs(30))
            .with_max_concurrent(100)
    }

    /// Create a dashboard handler configuration
    pub fn dashboard_handler(name: String) -> HandlerConfigBuilder<DashboardHandlerSpecificConfig> {
        HandlerConfigBuilder::new()
            .with_name(name)
            .with_timeout(std::time::Duration::from_secs(10)) // Fast UI responses
            .with_max_concurrent(200) // Many concurrent dashboard requests
    }

    /// Create a load testing handler configuration
    pub fn load_testing_handler(name: String) -> HandlerConfigBuilder<LoadTestingHandlerSpecificConfig> {
        HandlerConfigBuilder::new()
            .with_name(name)
            .with_timeout(std::time::Duration::from_secs(300)) // Long-running tests
            .with_max_concurrent(10) // Limit concurrent load tests
    }

    /// Create a workspace handler configuration
    pub fn workspace_handler(name: String) -> HandlerConfigBuilder<WorkspaceHandlerSpecificConfig> {
        HandlerConfigBuilder::new()
            .with_name(name)
            .with_timeout(std::time::Duration::from_secs(45))
            .with_max_concurrent(75)
    }

    /// Create an auth handler configuration
    pub fn auth_handler(name: String) -> HandlerConfigBuilder<AuthHandlerSpecificConfig> {
        HandlerConfigBuilder::new()
            .with_name(name)
            .with_timeout(std::time::Duration::from_secs(15)) // Fast auth responses
            .with_max_concurrent(500) // High auth throughput
    }
}

// ==================== SECTION ====================

/// Trait for loading configuration from environment variables
pub trait FromEnvironment {
    /// Load configuration from environment variables with a prefix
    fn from_env_with_prefix(prefix: &str) -> Result<Self, ConfigError>
    where
        Self: Sized;

    /// Load configuration from environment variables with default prefix
    fn from_env() -> Result<Self, ConfigError>
    where
        Self: Sized,
    {
        Self::from_env_with_prefix("NESTGATE")
    }
}

// ==================== SECTION ====================

/// Registry for managing multiple handler configurations
pub struct HandlerConfigRegistry {
    configs: HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
}

impl HandlerConfigRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    /// Register a handler configuration
    pub fn register<T>(&mut self, name: String, config: HandlerConfig<T>)
    where
        T: 'static + Send + Sync,
    {
        self.configs.insert(name, Box::new(config));
    }

    /// Get a handler configuration by name
    pub fn get<T>(&self, name: &str) -> Option<&HandlerConfig<T>>
    where
        T: 'static + Send + Sync,
    {
        self.configs
            .get(name)
            .and_then(|config| config.downcast_ref())
    }

    /// List all registered handler names
    pub fn list_handlers(&self) -> Vec<String> {
        self.configs.keys().cloned().collect()
    }

    /// Validate all registered configurations
    pub fn validate_all(&self) -> HashMap<String, Vec<ConfigError>> {
        // This would require implementing validation for all config types
        // For now, return empty map
        HashMap::new()
    }
}

impl Default for HandlerConfigRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== SECTION ====================

/// Create a default configuration for a specific handler type
pub fn default_config_for_handler(handler_type: &str) -> Result<Box<dyn std::any::Any>, ConfigError> {
    match handler_type {
        "zfs" => Ok(Box::new(
            HandlerConfigFactory::zfs_handler("default_zfs".to_string())
                .build()
                .unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
}),
        )),
        "performance" => Ok(Box::new(
            HandlerConfigFactory::performance_handler("default_performance".to_string())
                .build()
                .unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
}),
        )),
        "dashboard" => Ok(Box::new(
            HandlerConfigFactory::dashboard_handler("default_dashboard".to_string())
                .build()
                .unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
}),
        )),
        "load_testing" => Ok(Box::new(
            HandlerConfigFactory::load_testing_handler("default_load_testing".to_string())
                .build()
                .unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
}),
        )),
        "workspace" => Ok(Box::new(
            HandlerConfigFactory::workspace_handler("default_workspace".to_string())
                .build()
                .unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
}),
        )),
        "auth" => Ok(Box::new(
            HandlerConfigFactory::auth_handler("default_auth".to_string())
                .build()
                .unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
}),
        )),
        _ => Err(ConfigError {
            field: Some("handler_type".to_string()),
            message: format!("Unknown handler type: {}", handler_type),
        }),
    }
}

/// Get the list of supported handler types
pub fn supported_handler_types() -> Vec<&'static str> {
    vec![
        "zfs",
        "performance", 
        "dashboard",
        "load_testing",
        "workspace",
        "auth",
    ]
}

// ==================== SECTION ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_config_builder() {
        let config = HandlerConfigFactory::zfs_handler("test_zfs".to_string())
            .enabled(true)
            .with_max_concurrent(25)
            .build()
            .unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
});

        assert_eq!(config.extensions.common.handler_name, "test_zfs");
        assert_eq!(config.extensions.common.max_concurrent_requests, 25);
        assert!(config.extensions.common.enabled);
    }

    #[test]
    fn test_config_validation() {
        let mut config = CommonHandlerConfig::default();
        config.handler_name = String::new(); // Invalid empty name
        config.max_concurrent_requests = 0; // Invalid zero value

        let errors = config.validate();
        assert_eq!(errors.len(), 2);
        assert!(!config.is_valid());
    }

    #[test]
    fn test_config_registry() {
        let mut registry = HandlerConfigRegistry::new();
        let config = HandlerConfigFactory::zfs_handler("test".to_string())
            .build()
            .unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
});

        registry.register("test_zfs".to_string(), config);
        assert_eq!(registry.list_handlers().len(), 1);
        
        let retrieved: Option<&HandlerConfig<ZfsHandlerSpecificConfig>> = registry.get("test_zfs");
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_supported_handler_types() {
        let types = supported_handler_types();
        assert!(types.contains(&"zfs"));
        assert!(types.contains(&"performance"));
        assert!(types.contains(&"dashboard"));
        assert!(types.len() >= 6);
    }
} 