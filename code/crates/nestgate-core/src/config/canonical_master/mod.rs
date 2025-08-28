/// **CANONICAL MASTER CONFIGURATION SYSTEM**
///
/// This is THE single source of truth for ALL NestGate configuration,
/// replacing and consolidating 200+ scattered configuration structures
/// across all 11 crates.
///
/// **MODULAR ARCHITECTURE**:
/// - `system_config`: System-level configuration
/// - `network_config`: Network and connectivity configuration  
/// - `storage_config`: Storage and ZFS configuration
/// - `security_config`: Security and authentication configuration
/// - `api_config`: API and handler configuration
/// - `performance_config`: Performance and optimization configuration
/// - `supporting_types`: Common types and enums
/// - `builders`: Configuration builders and factories

use serde::{Deserialize, Serialize};

// ==================== SECTION ====================

/// System-level configuration types
pub mod system_config;

/// Network and connectivity configuration types  
pub mod network_config;

/// Storage and ZFS configuration types
pub mod storage_config;

/// Security and authentication configuration types
pub mod security_config;

/// API and handler configuration types
pub mod api_config;

/// Performance and optimization configuration types
pub mod performance_config;

/// Supporting types and enums
pub mod supporting_types;

/// Configuration builders and factories
pub mod builders;

// ==================== SECTION ====================

pub use system_config::*;
pub use network_config::*;
pub use storage_config::*;
pub use security_config::*;
pub use api_config::*;
pub use performance_config::*;
pub use supporting_types::*;
pub use builders::*;

// ==================== SECTION ====================

/// **THE** canonical configuration for the entire NestGate ecosystem
/// This replaces ALL other configuration structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestGateCanonicalConfig<
    const MAX_CONNECTIONS: usize = 1000,
    const BUFFER_SIZE: usize = 65536,
    const TIMEOUT_MS: u64 = 30000,
    const API_PORT: u16 = 8080,
> {
    /// System-level configuration
    pub system: SystemConfig<MAX_CONNECTIONS, BUFFER_SIZE>,
    
    /// Network and connectivity configuration
    pub network: NetworkConfig<API_PORT, TIMEOUT_MS>,
    
    /// Storage and ZFS configuration
    pub storage: StorageConfig,
    
    /// Security and authentication configuration
    pub security: SecurityConfig,
    
    /// API and handler configuration
    pub api: ApiConfig,
    
    /// Monitoring and observability configuration
    pub monitoring: MonitoringConfig,
    
    /// Performance and optimization configuration
    pub performance: PerformanceConfig<MAX_CONNECTIONS, BUFFER_SIZE>,
    
    /// MCP (Model Context Protocol) configuration
    pub mcp: McpConfig,
    
    /// Automation configuration
    pub automation: AutomationConfig,
    
    /// File system monitor configuration
    pub fsmonitor: FsMonitorConfig,
    
    /// NAS configuration
    pub nas: NasConfig,
    
    /// Middleware configuration
    pub middleware: MiddlewareConfig,
    
    /// Environment-specific settings
    pub environment: EnvironmentConfig,
    
    /// Feature flags
    pub features: FeatureFlags,
    
    /// Configuration metadata
    pub metadata: ConfigMetadata,
}

// ==================== SECTION ====================

impl<
    const MAX_CONNECTIONS: usize,
    const BUFFER_SIZE: usize,
    const TIMEOUT_MS: u64,
    const API_PORT: u16,
> Default for NestGateCanonicalConfig<MAX_CONNECTIONS, BUFFER_SIZE, TIMEOUT_MS, API_PORT> {
    fn default() -> Self {
        Self {
            system: SystemConfig::default(),
            network: NetworkConfig::default(),
            storage: StorageConfig::default(),
            security: SecurityConfig::default(),
            api: ApiConfig::default(),
            monitoring: MonitoringConfig::default(),
            performance: PerformanceConfig::default(),
            mcp: McpConfig::default(),
            automation: AutomationConfig::default(),
            fsmonitor: FsMonitorConfig::default(),
            nas: NasConfig::default(),
            middleware: MiddlewareConfig::default(),
            environment: EnvironmentConfig::default(),
            features: FeatureFlags::default(),
            metadata: ConfigMetadata::default(),
        }
    }
}

// ==================== SECTION ====================

/// Standard configuration with default const generics
pub type StandardConfig = NestGateCanonicalConfig;

/// High-performance configuration with optimized const generics
pub type HighPerformanceConfig = NestGateCanonicalConfig<2000, 131072, 15000, 8080>;

/// Development configuration with relaxed limits
pub type DevelopmentConfig = NestGateCanonicalConfig<100, 8192, 60000, 3000>;

/// Production configuration with production-optimized settings
pub type ProductionConfig = NestGateCanonicalConfig<5000, 262144, 10000, 443>; 