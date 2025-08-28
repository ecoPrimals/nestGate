//
// This module provides the core unified structure that coordinates all discovery
// configuration types, extracted from the monolithic unified_dynamic_config.rs file.
//
// **PROVIDES**:
// - Main UnifiedDynamicDiscoveryExtensions structure
// - Coordination between different discovery types
// - Default implementations and builders
// - Integration with universal adapter patterns
//
// **EXTRACTED FROM**: unified_dynamic_config.rs lines 26-42 (core structure)

use super::{
    timeout::TimeoutDiscoverySettings,
    network::NetworkDiscoverySettings,
    security::SecurityDiscoverySettings,
    environment::EnvironmentDiscoverySettings,
    storage::StorageDiscoverySettings,
    cache::CacheDiscoverySettings,
};
use serde::{Deserialize, Serialize};

// ==================== SECTION ====================

/// **UNIFIED DYNAMIC DISCOVERY EXTENSIONS**
/// 
/// Consolidates all dynamic discovery configuration patterns into a single,
/// comprehensive structure that coordinates discovery across all domains.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UnifiedDynamicDiscoveryExtensions {
    /// Timeout discovery settings
    pub timeout: TimeoutDiscoverySettings,
    /// Network discovery settings  
    pub network: NetworkDiscoverySettings,
    /// Security discovery settings
    pub security: SecurityDiscoverySettings,
    /// Environment discovery settings
    pub environment: EnvironmentDiscoverySettings,
    /// Storage discovery settings
    pub storage: StorageDiscoverySettings,
    /// Cache discovery settings
    pub cache: CacheDiscoverySettings,
}

// ==================== SECTION ====================


impl UnifiedDynamicDiscoveryExtensions {
    /// Create a new unified discovery configuration with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new unified discovery configuration with custom settings
    pub fn with_custom_settings() -> UnifiedDynamicDiscoveryExtensionsBuilder {
        UnifiedDynamicDiscoveryExtensionsBuilder::new()
    }

    /// Validate all discovery settings
    pub fn validate(&self) -> crate::Result<()> {
        // Validate each discovery type
        self.timeout.validate()?;
        self.network.validate()?;
        self.security.validate()?;
        self.environment.validate()?;
        self.storage.validate()?;
        self.cache.validate()?;

        Ok(())
    }
}

// ==================== SECTION ====================

/// Builder for creating UnifiedDynamicDiscoveryExtensions with custom settings
pub struct UnifiedDynamicDiscoveryExtensionsBuilder {
    timeout: Option<TimeoutDiscoverySettings>,
    network: Option<NetworkDiscoverySettings>,
    security: Option<SecurityDiscoverySettings>,
    environment: Option<EnvironmentDiscoverySettings>,
    storage: Option<StorageDiscoverySettings>,
    cache: Option<CacheDiscoverySettings>,
}

impl UnifiedDynamicDiscoveryExtensionsBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            timeout: None,
            network: None,
            security: None,
            environment: None,
            storage: None,
            cache: None,
        }
    }

    /// Set timeout discovery settings
    pub fn with_timeout(mut self, timeout: TimeoutDiscoverySettings) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set network discovery settings
    pub fn with_network(mut self, network: NetworkDiscoverySettings) -> Self {
        self.network = Some(network);
        self
    }

    /// Set security discovery settings
    pub fn with_security(mut self, security: SecurityDiscoverySettings) -> Self {
        self.security = Some(security);
        self
    }

    /// Set environment discovery settings
    pub fn with_environment(mut self, environment: EnvironmentDiscoverySettings) -> Self {
        self.environment = Some(environment);
        self
    }

    /// Set storage discovery settings
    pub fn with_storage(mut self, storage: StorageDiscoverySettings) -> Self {
        self.storage = Some(storage);
        self
    }

    /// Set cache discovery settings
    pub fn with_cache(mut self, cache: CacheDiscoverySettings) -> Self {
        self.cache = Some(cache);
        self
    }

    /// Build the unified discovery configuration
    pub fn build(self) -> UnifiedDynamicDiscoveryExtensions {
        UnifiedDynamicDiscoveryExtensions {
            timeout: self.timeout.unwrap_or_default(),
            network: self.network.unwrap_or_default(),
            security: self.security.unwrap_or_default(),
            environment: self.environment.unwrap_or_default(),
            storage: self.storage.unwrap_or_default(),
            cache: self.cache.unwrap_or_default(),
        }
    }
}

impl Default for UnifiedDynamicDiscoveryExtensionsBuilder {
    fn default() -> Self {
        Self::new()
    }
} 