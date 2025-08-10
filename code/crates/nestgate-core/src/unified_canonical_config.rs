/// **UNIFIED CANONICAL CONFIGURATION - ROOT SYSTEM**
/// This is the ultimate configuration unification that brings together all domain-specific
/// unified configurations under a single canonical root system following the proven patterns.

use nestgate_core::unified_config_consolidation::StandardDomainConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Import all unified domain configurations
// Note: UnifiedApiConfig moved to avoid circular dependency
// use nestgate_api::unified_api_config::{UnifiedApiConfig, UnifiedPrimalConfig};
use nestgate_automation::unified_automation_config::UnifiedAutomationConfig;
use nestgate_fsmonitor::unified_fsmonitor_config::UnifiedFsMonitorConfig;
use nestgate_mcp::unified_mcp_config::UnifiedMcpConfig;
use nestgate_middleware::unified_middleware_config::UnifiedMiddlewareConfig;
use nestgate_nas::unified_nas_config::UnifiedNasConfig;
use nestgate_network::unified_network_extensions::UnifiedNetworkConfig;
use nestgate_zfs::unified_zfs_config::UnifiedZfsConfig;

// Re-export the existing canonical types for compatibility
pub use crate::config::canonical::{
    CanonicalConfig as LegacyCanonicalConfig, CanonicalConfigBuilder as LegacyCanonicalConfigBuilder,
    Environment, EnvironmentConfig, IntegrationsConfig, MonitoringConfig, NetworkConfig,
    PerformanceConfig, SecurityConfig, StorageConfig, SystemConfig,
};

/// **UNIFIED CANONICAL EXTENSIONS**
/// Consolidates all unified domain configurations into a single root system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCanonicalExtensions {
    /// API and HTTP server configuration
    pub api: UnifiedApiConfig,
    /// Primal ecosystem integration configuration
    pub primal: UnifiedPrimalConfig,
    /// Network and orchestration configuration
    pub network: UnifiedNetworkConfig,
    /// Storage and ZFS configuration
    pub zfs: UnifiedZfsConfig,
    /// NAS and file sharing configuration
    pub nas: UnifiedNasConfig,
    /// MCP protocol configuration
    pub mcp: UnifiedMcpConfig,
    /// Middleware and request processing configuration
    pub middleware: UnifiedMiddlewareConfig,
    /// Automation and lifecycle configuration
    pub automation: UnifiedAutomationConfig,
    /// File system monitoring configuration
    pub fsmonitor: UnifiedFsMonitorConfig,
    /// Custom domain configurations
    pub custom_domains: HashMap<String, serde_json::Value>,
    /// Feature flags for domain enablement
    pub domain_features: DomainFeatureFlags,
    /// Cross-domain integration settings
    pub integrations: CrossDomainIntegrations,
}

/// **UNIFIED CANONICAL CONFIGURATION**
/// The ultimate single source of truth for ALL NestGate configuration across ALL domains
pub type UnifiedCanonicalConfig = StandardDomainConfig<UnifiedCanonicalExtensions>;

/// Domain feature flags for selective enablement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainFeatureFlags {
    /// Enable API server functionality
    pub api_enabled: bool,
    /// Enable primal ecosystem integration
    pub primal_enabled: bool,
    /// Enable advanced networking features
    pub network_enabled: bool,
    /// Enable ZFS storage management
    pub zfs_enabled: bool,
    /// Enable NAS file sharing
    pub nas_enabled: bool,
    /// Enable MCP protocol support
    pub mcp_enabled: bool,
    /// Enable middleware processing
    pub middleware_enabled: bool,
    /// Enable automation features
    pub automation_enabled: bool,
    /// Enable file system monitoring
    pub fsmonitor_enabled: bool,
    /// Enable experimental features
    pub experimental_enabled: bool,
}

/// Cross-domain integration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossDomainIntegrations {
    /// API to storage integration
    pub api_storage_bridge: ApiStorageBridge,
    /// Automation to monitoring integration
    pub automation_monitoring_bridge: AutomationMonitoringBridge,
    /// Network to security integration
    pub network_security_bridge: NetworkSecurityBridge,
    /// Middleware to authentication integration
    pub middleware_auth_bridge: MiddlewareAuthBridge,
    /// Custom integration configurations
    pub custom_bridges: HashMap<String, serde_json::Value>,
}

/// API to storage integration bridge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiStorageBridge {
    /// Enable direct API to ZFS integration
    pub direct_zfs_access: bool,
    /// Enable API to NAS integration
    pub nas_api_exposure: bool,
    /// Storage operation timeout
    pub operation_timeout: std::time::Duration,
    /// Maximum concurrent storage operations
    pub max_concurrent_ops: u32,
}

/// Automation to monitoring integration bridge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationMonitoringBridge {
    /// Enable automation event monitoring
    pub event_monitoring: bool,
    /// Enable performance metrics collection
    pub performance_metrics: bool,
    /// Metrics collection interval
    pub metrics_interval: std::time::Duration,
    /// Alert thresholds
    pub alert_thresholds: HashMap<String, f64>,
}

/// Network to security integration bridge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecurityBridge {
    /// Enable network-level security enforcement
    pub network_security: bool,
    /// Enable TLS termination
    pub tls_termination: bool,
    /// Enable rate limiting
    pub rate_limiting: bool,
    /// Security policy enforcement
    pub policy_enforcement: SecurityPolicyEnforcement,
}

/// Security policy enforcement configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicyEnforcement {
    /// Enable IP-based access control
    pub ip_access_control: bool,
    /// Enable certificate-based authentication
    pub certificate_auth: bool,
    /// Enable API key validation
    pub api_key_validation: bool,
    /// Allowed IP ranges
    pub allowed_ip_ranges: Vec<String>,
    /// Blocked IP ranges
    pub blocked_ip_ranges: Vec<String>,
}

/// Middleware to authentication integration bridge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareAuthBridge {
    /// Enable authentication middleware
    pub auth_middleware: bool,
    /// Enable authorization middleware
    pub authz_middleware: bool,
    /// Authentication providers
    pub auth_providers: Vec<AuthProvider>,
    /// Session management
    pub session_management: SessionManagement,
}

/// Authentication provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProvider {
    /// Provider name
    pub name: String,
    /// Provider type
    pub provider_type: AuthProviderType,
    /// Provider configuration
    pub config: HashMap<String, serde_json::Value>,
    /// Provider priority
    pub priority: u32,
}

/// Authentication provider types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthProviderType {
    /// JWT token provider
    Jwt,
    /// OAuth2 provider
    OAuth2,
    /// LDAP provider
    Ldap,
    /// Database provider
    Database,
    /// Custom provider
    Custom(String),
}

/// Session management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionManagement {
    /// Enable session management
    pub enabled: bool,
    /// Session timeout
    pub timeout: std::time::Duration,
    /// Session storage backend
    pub storage: SessionStorageBackend,
    /// Enable session encryption
    pub encryption: bool,
}

/// Session storage backends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStorageBackend {
    /// In-memory storage
    Memory,
    /// Redis storage
    Redis { url: String },
    /// Database storage
    Database { connection_string: String },
    /// Custom storage
    Custom(String),
}

impl UnifiedCanonicalConfig {
    /// Create development configuration with all domains enabled
    pub fn development() -> Self {
        let mut config = Self::create_for_environment("development");
        config.extensions.domain_features = DomainFeatureFlags::development();
        config
    }

    /// Create production configuration with selective domain enablement
    pub fn production() -> Self {
        let mut config = Self::create_for_environment("production");
        config.extensions.domain_features = DomainFeatureFlags::production();
        config
    }

    /// Create testing configuration optimized for integration tests
    pub fn testing() -> Self {
        let mut config = Self::create_for_environment("testing");
        config.extensions.domain_features = DomainFeatureFlags::testing();
        config
    }

    /// Create staging configuration for pre-production testing
    pub fn staging() -> Self {
        let mut config = Self::create_for_environment("staging");
        config.extensions.domain_features = DomainFeatureFlags::staging();
        config
    }

    /// Create minimal configuration with only essential domains
    pub fn minimal() -> Self {
        let mut config = Self::create_for_environment("minimal");
        config.extensions.domain_features = DomainFeatureFlags::minimal();
        config
    }

    /// Create custom configuration for specific use cases
    pub fn custom(domains: &[&str]) -> Self {
        let mut config = Self::create_for_environment("custom");
        config.extensions.domain_features = DomainFeatureFlags::custom(domains);
        config
    }

    /// Enable specific domain
    pub fn enable_domain(&mut self, domain: &str) {
        match domain {
            "api" => self.extensions.domain_features.api_enabled = true,
            "primal" => self.extensions.domain_features.primal_enabled = true,
            "network" => self.extensions.domain_features.network_enabled = true,
            "zfs" => self.extensions.domain_features.zfs_enabled = true,
            "nas" => self.extensions.domain_features.nas_enabled = true,
            "mcp" => self.extensions.domain_features.mcp_enabled = true,
            "middleware" => self.extensions.domain_features.middleware_enabled = true,
            "automation" => self.extensions.domain_features.automation_enabled = true,
            "fsmonitor" => self.extensions.domain_features.fsmonitor_enabled = true,
            "experimental" => self.extensions.domain_features.experimental_enabled = true,
            _ => {
                // Custom domain - add to custom_domains
                self.extensions.custom_domains.insert(
                    domain.to_string(),
                    serde_json::json!({"enabled": true}),
                );
            }
        }
    }

    /// Disable specific domain
    pub fn disable_domain(&mut self, domain: &str) {
        match domain {
            "api" => self.extensions.domain_features.api_enabled = false,
            "primal" => self.extensions.domain_features.primal_enabled = false,
            "network" => self.extensions.domain_features.network_enabled = false,
            "zfs" => self.extensions.domain_features.zfs_enabled = false,
            "nas" => self.extensions.domain_features.nas_enabled = false,
            "mcp" => self.extensions.domain_features.mcp_enabled = false,
            "middleware" => self.extensions.domain_features.middleware_enabled = false,
            "automation" => self.extensions.domain_features.automation_enabled = false,
            "fsmonitor" => self.extensions.domain_features.fsmonitor_enabled = false,
            "experimental" => self.extensions.domain_features.experimental_enabled = false,
            _ => {
                // Custom domain - remove or disable
                self.extensions.custom_domains.remove(domain);
            }
        }
    }

    /// Get enabled domains
    pub fn get_enabled_domains(&self) -> Vec<String> {
        let mut domains = Vec::new();
        
        if self.extensions.domain_features.api_enabled {
            domains.push("api".to_string());
        }
        if self.extensions.domain_features.primal_enabled {
            domains.push("primal".to_string());
        }
        if self.extensions.domain_features.network_enabled {
            domains.push("network".to_string());
        }
        if self.extensions.domain_features.zfs_enabled {
            domains.push("zfs".to_string());
        }
        if self.extensions.domain_features.nas_enabled {
            domains.push("nas".to_string());
        }
        if self.extensions.domain_features.mcp_enabled {
            domains.push("mcp".to_string());
        }
        if self.extensions.domain_features.middleware_enabled {
            domains.push("middleware".to_string());
        }
        if self.extensions.domain_features.automation_enabled {
            domains.push("automation".to_string());
        }
        if self.extensions.domain_features.fsmonitor_enabled {
            domains.push("fsmonitor".to_string());
        }
        if self.extensions.domain_features.experimental_enabled {
            domains.push("experimental".to_string());
        }

        // Add custom domains
        for (domain, config) in &self.extensions.custom_domains {
            if let Some(enabled) = config.get("enabled") {
                if enabled.as_bool().unwrap_or(false) {
                    domains.push(domain.clone());
                }
            }
        }

        domains
    }

    /// Validate configuration consistency across domains
    pub fn validate_cross_domain_consistency(&self) -> Result<(), String> {
        // API domain requires network domain
        if self.extensions.domain_features.api_enabled && !self.extensions.domain_features.network_enabled {
            return Err("API domain requires network domain to be enabled".to_string());
        }

        // NAS domain requires ZFS domain
        if self.extensions.domain_features.nas_enabled && !self.extensions.domain_features.zfs_enabled {
            return Err("NAS domain requires ZFS domain to be enabled".to_string());
        }

        // Automation domain benefits from monitoring
        if self.extensions.domain_features.automation_enabled && !self.extensions.integrations.automation_monitoring_bridge.event_monitoring {
            // This is a warning, not an error
            eprintln!("Warning: Automation domain is enabled but monitoring integration is disabled");
        }

        // Middleware domain requires API domain for HTTP processing
        if self.extensions.domain_features.middleware_enabled && !self.extensions.domain_features.api_enabled {
            return Err("Middleware domain requires API domain to be enabled".to_string());
        }

        Ok(())
    }
}

impl Default for UnifiedCanonicalExtensions {
    fn default() -> Self {
        Self {
            api: UnifiedApiConfig::default(),
            primal: UnifiedPrimalConfig::default(),
            network: UnifiedNetworkConfig::default(),
            zfs: UnifiedZfsConfig::default(),
            nas: UnifiedNasConfig::default(),
            mcp: UnifiedMcpConfig::default(),
            middleware: UnifiedMiddlewareConfig::default(),
            automation: UnifiedAutomationConfig::default(),
            fsmonitor: UnifiedFsMonitorConfig::default(),
            custom_domains: HashMap::new(),
            domain_features: DomainFeatureFlags::default(),
            integrations: CrossDomainIntegrations::default(),
        }
    }
}

impl DomainFeatureFlags {
    /// Development environment - all domains enabled
    pub fn development() -> Self {
        Self {
            api_enabled: true,
            primal_enabled: true,
            network_enabled: true,
            zfs_enabled: true,
            nas_enabled: true,
            mcp_enabled: true,
            middleware_enabled: true,
            automation_enabled: true,
            fsmonitor_enabled: true,
            experimental_enabled: true,
        }
    }

    /// Production environment - stable domains only
    pub fn production() -> Self {
        Self {
            api_enabled: true,
            primal_enabled: true,
            network_enabled: true,
            zfs_enabled: true,
            nas_enabled: true,
            mcp_enabled: true,
            middleware_enabled: true,
            automation_enabled: true,
            fsmonitor_enabled: false, // Disabled in production for performance
            experimental_enabled: false,
        }
    }

    /// Testing environment - core domains for testing
    pub fn testing() -> Self {
        Self {
            api_enabled: true,
            primal_enabled: false, // Disabled for isolated testing
            network_enabled: true,
            zfs_enabled: true,
            nas_enabled: false,
            mcp_enabled: false,
            middleware_enabled: true,
            automation_enabled: false,
            fsmonitor_enabled: false,
            experimental_enabled: false,
        }
    }

    /// Staging environment - production-like with monitoring
    pub fn staging() -> Self {
        Self {
            api_enabled: true,
            primal_enabled: true,
            network_enabled: true,
            zfs_enabled: true,
            nas_enabled: true,
            mcp_enabled: true,
            middleware_enabled: true,
            automation_enabled: true,
            fsmonitor_enabled: true, // Enabled for testing
            experimental_enabled: false,
        }
    }

    /// Minimal environment - essential domains only
    pub fn minimal() -> Self {
        Self {
            api_enabled: true,
            primal_enabled: false,
            network_enabled: true,
            zfs_enabled: true,
            nas_enabled: false,
            mcp_enabled: false,
            middleware_enabled: false,
            automation_enabled: false,
            fsmonitor_enabled: false,
            experimental_enabled: false,
        }
    }

    /// Custom environment - enable specific domains
    pub fn custom(domains: &[&str]) -> Self {
        let mut flags = Self::minimal(); // Start with minimal
        
        for domain in domains {
            match *domain {
                "api" => flags.api_enabled = true,
                "primal" => flags.primal_enabled = true,
                "network" => flags.network_enabled = true,
                "zfs" => flags.zfs_enabled = true,
                "nas" => flags.nas_enabled = true,
                "mcp" => flags.mcp_enabled = true,
                "middleware" => flags.middleware_enabled = true,
                "automation" => flags.automation_enabled = true,
                "fsmonitor" => flags.fsmonitor_enabled = true,
                "experimental" => flags.experimental_enabled = true,
                _ => {} // Ignore unknown domains
            }
        }
        
        flags
    }
}

impl Default for DomainFeatureFlags {
    fn default() -> Self {
        Self::development() // Default to development for safety
    }
}

impl Default for CrossDomainIntegrations {
    fn default() -> Self {
        Self {
            api_storage_bridge: ApiStorageBridge::default(),
            automation_monitoring_bridge: AutomationMonitoringBridge::default(),
            network_security_bridge: NetworkSecurityBridge::default(),
            middleware_auth_bridge: MiddlewareAuthBridge::default(),
            custom_bridges: HashMap::new(),
        }
    }
}

impl Default for ApiStorageBridge {
    fn default() -> Self {
        Self {
            direct_zfs_access: true,
            nas_api_exposure: true,
            operation_timeout: std::time::Duration::from_secs(30),
            max_concurrent_ops: 10,
        }
    }
}

impl Default for AutomationMonitoringBridge {
    fn default() -> Self {
        Self {
            event_monitoring: true,
            performance_metrics: true,
            metrics_interval: std::time::Duration::from_secs(60),
            alert_thresholds: HashMap::new(),
        }
    }
}

impl Default for NetworkSecurityBridge {
    fn default() -> Self {
        Self {
            network_security: true,
            tls_termination: true,
            rate_limiting: true,
            policy_enforcement: SecurityPolicyEnforcement::default(),
        }
    }
}

impl Default for SecurityPolicyEnforcement {
    fn default() -> Self {
        Self {
            ip_access_control: false,
            certificate_auth: false,
            api_key_validation: true,
            allowed_ip_ranges: vec!["127.0.0.1/8".to_string(), "10.0.0.0/8".to_string()],
            blocked_ip_ranges: Vec::new(),
        }
    }
}

impl Default for MiddlewareAuthBridge {
    fn default() -> Self {
        Self {
            auth_middleware: true,
            authz_middleware: true,
            auth_providers: Vec::new(),
            session_management: SessionManagement::default(),
        }
    }
}

impl Default for SessionManagement {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: std::time::Duration::from_secs(3600), // 1 hour
            storage: SessionStorageBackend::Memory,
            encryption: true,
        }
    }
}

/// **CONVENIENCE BUILDER FUNCTIONS**
impl UnifiedCanonicalConfig {
    /// Create a configuration for API-only deployment
    pub fn api_only() -> Self {
        Self::custom(&["api", "network", "middleware"])
    }

    /// Create a configuration for storage-only deployment
    pub fn storage_only() -> Self {
        Self::custom(&["zfs", "nas", "network"])
    }

    /// Create a configuration for monitoring deployment
    pub fn monitoring_only() -> Self {
        Self::custom(&["api", "network", "middleware", "fsmonitor"])
    }

    /// Create a configuration for automation deployment
    pub fn automation_only() -> Self {
        Self::custom(&["automation", "network", "zfs"])
    }

    /// Create a configuration for primal integration deployment
    pub fn primal_integration() -> Self {
        Self::custom(&["primal", "api", "network", "middleware", "mcp"])
    }
} 