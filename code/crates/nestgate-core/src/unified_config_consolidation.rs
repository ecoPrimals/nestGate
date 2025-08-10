/// Unified Configuration Consolidation Module
/// This module provides standardized patterns and utilities for consolidating
/// the 50+ fragmented Config structs across the codebase into unified patterns.
/// **PROBLEM SOLVED**: Eliminates configuration fragmentation and provides
/// consistent patterns for all domain-specific configurations.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::unified_types::{
    UnifiedMemoryConfig, UnifiedMonitoringConfig, UnifiedNetworkConfig, UnifiedSecurityConfig,
    UnifiedServiceConfig, UnifiedStorageConfig,
};

// ==================== STANDARDIZED CONFIG PATTERNS ====================

/// **THE** standardized config pattern for all domain-specific configurations
/// This provides a consistent structure: base unified configs + domain extensions
#[derive(Debug, Clone, Serialize)]
pub struct StandardDomainConfig<T>
where
    T: Clone + Serialize + serde::de::DeserializeOwned,
{
    /// Base service configuration (standardized across all services)
    pub service: UnifiedServiceConfig,
    /// Network configuration (standardized across all services)
    pub network: UnifiedNetworkConfig,
    /// Security configuration (standardized across all services)
    pub security: UnifiedSecurityConfig,
    /// Monitoring configuration (standardized across all services)
    pub monitoring: UnifiedMonitoringConfig,
    /// Storage configuration (standardized across all services)
    pub storage: UnifiedStorageConfig,
    /// Memory configuration (standardized across all services)
    pub memory: UnifiedMemoryConfig,
    /// Domain-specific configuration extensions
    pub extensions: T,
    /// Service endpoints for capability-based discovery
    pub service_endpoints: HashMap<String, String>,
    /// Feature flags specific to this domain
    pub feature_flags: HashMap<String, bool>,
}

impl<T> StandardDomainConfig<T>
where
    T: Clone + Serialize + serde::de::DeserializeOwned,
{
    /// Create a new StandardDomainConfig with the provided extensions
    pub fn new(extensions: T) -> Self {
        Self {
            service: UnifiedServiceConfig::default(),
            network: UnifiedNetworkConfig::default(),
            security: UnifiedSecurityConfig::default(),
            monitoring: UnifiedMonitoringConfig::default(),
            storage: UnifiedStorageConfig::default(),
            memory: UnifiedMemoryConfig::default(),
            extensions,
            service_endpoints: HashMap::new(),
            feature_flags: HashMap::new(),
        }
    }

    /// Get a reference to the extensions
    pub fn extensions(&self) -> &T {
        &self.extensions
    }

    /// Get a mutable reference to the extensions
    pub fn extensions_mut(&mut self) -> &mut T {
        &mut self.extensions
    }
}

impl<T> StandardDomainConfig<T>
where
    T: Clone + Serialize + serde::de::DeserializeOwned + Default,
{
    /// Create a new domain config with default extensions
    pub fn with_defaults() -> Self {
        Self::new(T::default())
    }

    /// Create a config with specified service information
    pub fn with_service(extensions: T, service_name: &str, version: &str) -> Self {
        let mut config = Self::new(extensions);
        config.service.name = service_name.to_string();
        config.service.version = version.to_string();
        config
    }

    /// Add a service endpoint
    pub fn add_endpoint(&mut self, name: &str, url: &str) {
        self.service_endpoints
            .insert(name.to_string(), url.to_string());
    }

    /// Set a feature flag
    pub fn set_feature(&mut self, feature: &str, enabled: bool) {
        self.feature_flags.insert(feature.to_string(), enabled);
    }

    /// Check if a feature is enabled
    pub fn is_feature_enabled(&self, feature: &str) -> bool {
        self.feature_flags.get(feature).copied().unwrap_or(false)
    }

    /// Create a development configuration with debug settings
    pub fn development() -> Self {
        let mut config = Self::new(T::default());
        config.service.environment = "development".to_string();
        config.set_feature("debug_mode", true);
        config.set_feature("verbose_logging", true);
        config
    }

    /// Create a production configuration with optimized settings
    pub fn production() -> Self {
        let mut config = Self::new(T::default());
        config.service.environment = "production".to_string();
        config.set_feature("debug_mode", false);
        config.set_feature("verbose_logging", false);
        config
    }
}

impl<T> Default for StandardDomainConfig<T>
where
    T: Clone + Serialize + serde::de::DeserializeOwned + Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<'de, T> serde::Deserialize<'de> for StandardDomainConfig<T>
where
    T: Clone + Serialize + serde::de::DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;
        use std::marker::PhantomData;

        struct StandardDomainConfigVisitor<T> {
            marker: PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for StandardDomainConfigVisitor<T>
        where
            T: Clone + Serialize + serde::de::DeserializeOwned,
        {
            type Value = StandardDomainConfig<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct StandardDomainConfig")
            }

            fn visit_map<V>(self, mut map: V) -> Result<StandardDomainConfig<T>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut service = None;
                let mut network = None;
                let mut security = None;
                let mut monitoring = None;
                let mut storage = None;
                let mut memory = None;
                let mut extensions = None;
                let mut service_endpoints = None;
                let mut feature_flags = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "service" => {
                            service = Some(map.next_value()?);
                        }
                        "network" => {
                            network = Some(map.next_value()?);
                        }
                        "security" => {
                            security = Some(map.next_value()?);
                        }
                        "monitoring" => {
                            monitoring = Some(map.next_value()?);
                        }
                        "storage" => {
                            storage = Some(map.next_value()?);
                        }
                        "memory" => {
                            memory = Some(map.next_value()?);
                        }
                        "extensions" => {
                            extensions = Some(map.next_value()?);
                        }
                        "service_endpoints" => {
                            service_endpoints = Some(map.next_value()?);
                        }
                        "feature_flags" => {
                            feature_flags = Some(map.next_value()?);
                        }
                        _ => {
                            let _: serde_json::Value = map.next_value()?;
                        }
                    }
                }

                Ok(StandardDomainConfig {
                    service: service.unwrap_or_default(),
                    network: network.unwrap_or_default(),
                    security: security.unwrap_or_default(),
                    monitoring: monitoring.unwrap_or_default(),
                    storage: storage.unwrap_or_default(),
                    memory: memory.unwrap_or_default(),
                    extensions: extensions.ok_or_else(|| de::Error::missing_field("extensions"))?,
                    service_endpoints: service_endpoints.unwrap_or_default(),
                    feature_flags: feature_flags.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "StandardDomainConfig",
            &[
                "service",
                "network",
                "security",
                "monitoring",
                "storage",
                "memory",
                "extensions",
                "service_endpoints",
                "feature_flags",
            ],
            StandardDomainConfigVisitor {
                marker: PhantomData,
            },
        )
    }
}

// ==================== DOMAIN-SPECIFIC EXTENSIONS ====================

/// ZFS service extensions for StandardDomainConfig<T>
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsExtensions {
    /// Pool management settings
    pub pool_settings: ZfsPoolSettings,
    /// Performance optimization settings
    pub performance: ZfsPerformanceSettings,
    /// Security and encryption settings
    pub security: ZfsSecuritySettings,
    /// Health monitoring settings
    pub health: ZfsHealthSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPoolSettings {
    /// Default pool name
    pub default_pool_name: String,
    /// Enable compression
    pub enable_compression: bool,
    /// Enable deduplication
    pub enable_deduplication: bool,
    /// Enable encryption
    pub enable_encryption: bool,
    /// Auto-create pools if missing
    pub auto_pool_creation: bool,
}

impl Default for ZfsPoolSettings {
    fn default() -> Self {
        Self {
            default_pool_name: "nestgate-pool".to_string(),
            enable_compression: true,
            enable_deduplication: false,
            enable_encryption: true,
            auto_pool_creation: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsPerformanceSettings {
    /// Enable performance optimization
    pub enable_optimization: bool,
    /// Cache size in MB
    pub cache_size_mb: Option<u64>,
    /// I/O queue depth
    pub io_queue_depth: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsSecuritySettings {
    /// Enable access control
    pub enable_access_control: bool,
    /// Encryption algorithm
    pub encryption_algorithm: Option<String>,
    /// Key management settings
    pub key_management: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsHealthSettings {
    /// Health check interval in seconds
    pub check_interval_seconds: u64,
    /// Enable auto-repair
    pub enable_auto_repair: bool,
    /// Alert thresholds
    pub alert_thresholds: HashMap<String, f64>,
}

/// NAS service extensions for StandardDomainConfig<T>
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NasExtensions {
    /// Protocol settings
    pub protocols: NasProtocolSettings,
    /// Share management
    pub shares: NasShareSettings,
    /// Access control
    pub access_control: NasAccessSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasProtocolSettings {
    /// Enable SMB protocol
    pub smb_enabled: bool,
    /// Enable NFS protocol
    pub nfs_enabled: bool,
    /// Enable FTP protocol
    pub ftp_enabled: bool,
    /// Enable HTTP/WebDAV
    pub webdav_enabled: bool,
}

impl Default for NasProtocolSettings {
    fn default() -> Self {
        Self {
            smb_enabled: true,
            nfs_enabled: true,
            ftp_enabled: false,
            webdav_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NasShareSettings {
    /// Default share path
    pub default_share_path: String,
    /// Auto-create shares
    pub auto_create_shares: bool,
    /// Share permissions
    pub default_permissions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NasAccessSettings {
    /// Enable user authentication
    pub enable_user_auth: bool,
    /// Default access level
    pub default_access_level: String,
    /// Access control rules
    pub access_rules: Vec<String>,
}

/// MCP service extensions for StandardDomainConfig<T>
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpExtensions {
    /// Provider settings
    pub provider: McpProviderSettings,
    /// Session management
    pub session: McpSessionSettings,
    /// Quality of service
    pub qos: McpQosSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpProviderSettings {
    /// Provider name
    pub provider_name: String,
    /// Provider type
    pub provider_type: String,
    /// Provider capabilities
    pub capabilities: Vec<String>,
}

impl Default for McpProviderSettings {
    fn default() -> Self {
        Self {
            provider_name: "nestgate-mcp".to_string(),
            provider_type: "universal".to_string(),
            capabilities: vec!["storage".to_string(), "compute".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpSessionSettings {
    /// Session timeout in seconds
    pub timeout_seconds: u64,
    /// Max concurrent sessions
    pub max_concurrent_sessions: u32,
    /// Enable session persistence
    pub enable_persistence: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpQosSettings {
    /// Enable quality of service
    pub enable_qos: bool,
    /// Priority levels
    pub priority_levels: Vec<String>,
    /// Rate limiting settings
    pub rate_limits: HashMap<String, u64>,
}

// ==================== TYPE ALIASES FOR STANDARDIZED CONFIGS ====================

/// Standardized ZFS configuration
pub type UnifiedZfsConfig = StandardDomainConfig<ZfsExtensions>;

/// Standardized NAS configuration  
pub type UnifiedNasConfig = StandardDomainConfig<NasExtensions>;

/// Standardized MCP configuration
pub type UnifiedMcpConfig = StandardDomainConfig<McpExtensions>;

// ==================== MIGRATION UTILITIES ====================

/// Migration utilities for converting legacy configs to unified patterns
pub mod migration {
    use super::*;

    /// Convert legacy ZFS configs to unified pattern
    pub fn migrate_zfs_config(
        legacy_fields: HashMap<String, serde_json::Value>,
    ) -> UnifiedZfsConfig {
        let extensions = ZfsExtensions {
            pool_settings: ZfsPoolSettings {
                default_pool_name: legacy_fields
                    .get("pool_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("nestgate-pool")
                    .to_string(),
                enable_compression: legacy_fields
                    .get("compression")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true),
                enable_deduplication: legacy_fields
                    .get("deduplication")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
                enable_encryption: legacy_fields
                    .get("encryption")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true),
                auto_pool_creation: legacy_fields
                    .get("auto_create")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
            },
            ..Default::default()
        };

        StandardDomainConfig::with_service(extensions, "nestgate-zfs", env!("CARGO_PKG_VERSION"))
    }

    /// Convert legacy NAS configs to unified pattern
    pub fn migrate_nas_config(
        legacy_fields: HashMap<String, serde_json::Value>,
    ) -> UnifiedNasConfig {
        let extensions = NasExtensions {
            protocols: NasProtocolSettings {
                smb_enabled: legacy_fields
                    .get("smb_enabled")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true),
                nfs_enabled: legacy_fields
                    .get("nfs_enabled")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true),
                ftp_enabled: legacy_fields
                    .get("ftp_enabled")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
                webdav_enabled: legacy_fields
                    .get("webdav_enabled")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true),
            },
            ..Default::default()
        };

        StandardDomainConfig::with_service(extensions, "nestgate-nas", env!("CARGO_PKG_VERSION"))
    }

    /// Convert legacy MCP configs to unified pattern
    pub fn migrate_mcp_config(
        legacy_fields: HashMap<String, serde_json::Value>,
    ) -> UnifiedMcpConfig {
        let extensions = McpExtensions {
            provider: McpProviderSettings {
                provider_name: legacy_fields
                    .get("provider_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("nestgate-mcp")
                    .to_string(),
                provider_type: legacy_fields
                    .get("provider_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("universal")
                    .to_string(),
                capabilities: legacy_fields
                    .get("capabilities")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_else(|| vec!["storage".to_string(), "compute".to_string()]),
            },
            ..Default::default()
        };

        StandardDomainConfig::with_service(extensions, "nestgate-mcp", env!("CARGO_PKG_VERSION"))
    }
}

// ==================== CONFIGURATION VALIDATION ====================

/// Validation utilities for unified configurations
pub mod validation {
    use super::*;

    /// Validate a StandardDomainConfig
    pub fn validate_config<T>(config: &StandardDomainConfig<T>) -> Result<(), Vec<String>>
    where
        T: Clone + Serialize + serde::de::DeserializeOwned,
    {
        let mut errors = Vec::new();

        // Validate service configuration
        if config.service.name.is_empty() {
            errors.push("Service name cannot be empty".to_string());
        }

        if config.service.version.is_empty() {
            errors.push("Service version cannot be empty".to_string());
        }

        // Validate network configuration
        if config.network.bind_address.to_string().is_empty() {
            errors.push("Network host cannot be empty".to_string());
        }

        if config.network.port == 0 {
            errors.push("Network port must be greater than 0".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
