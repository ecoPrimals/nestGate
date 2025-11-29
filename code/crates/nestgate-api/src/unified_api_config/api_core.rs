/// **UNIFIED API CONFIGURATION - CORE MODULE**
/// Contains the main UnifiedApiConfig struct and core configuration logic.
/// This eliminates fragmented API config structs across multiple modules.
// CANONICAL MODERNIZATION: Migrated from deprecated unified_final_config
use nestgate_core::canonical_modernization::CanonicalModernizedConfig;
use serde::{Deserialize, Serialize};
/// **UNIFIED API EXTENSIONS**
/// Consolidates all API-specific configuration patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Unifiedapiextensions
pub struct UnifiedApiExtensions {
    /// HTTP server and REST API settings
    pub http_server: ApiHttpServerSettings,
    /// Streaming and real-time settings
    pub streaming: ApiStreamingSettings,
    /// Service mesh and RPC settings
    pub service_mesh: ApiServiceMeshSettings,
    /// Server-Sent Events settings
    pub sse: ApiSseSettings,
    /// Primal ecosystem integration
    pub primal: ApiPrimalSettings,
    /// Authentication and authorization
    pub auth: ApiAuthSettings,
    /// Performance and optimization
    pub performance: ApiPerformanceSettings,
    /// Health checks and monitoring
    pub health: ApiHealthSettings,
    /// Storage and persistence
    pub storage: ApiStorageSettings,
}
impl Default for UnifiedApiExtensions {
    /// Returns the default instance
    fn default() -> Self { Self {
            http_server: ApiHttpServerSettings::default(),
            streaming: ApiStreamingSettings::default(),
            service_mesh: ApiServiceMeshSettings::default(),
            sse: ApiSseSettings::default(),
            primal: ApiPrimalSettings::default(),
            auth: ApiAuthSettings::default(),
            performance: ApiPerformanceSettings::default(),
            health: ApiHealthSettings::default(),
            storage: ApiStorageSettings::default(),
         }
}

/// **UNIFIED API CONFIGURATION - CANONICAL MODERNIZATION**
/// The single source of truth for all API configuration across the system
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::UnifiedApiConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::UnifiedApiConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for UnifiedApi
pub struct UnifiedApiConfig {
    /// Base canonical configuration
    pub base: CanonicalModernizedConfig,
    /// API-specific extensions
    pub api_extensions: UnifiedApiExtensions,
}
impl Default for UnifiedApiConfig {
    /// Returns the default instance
    fn default() -> Self { Self {
            base: CanonicalModernizedConfig::default(),
            api_extensions: UnifiedApiExtensions::default(),
         }
}

impl UnifiedApiConfig {
    /// Create development configuration optimized for local development
    #[must_use]
    pub fn development() -> Self { let mut config = Self::default();
        config.base.runtime.deployment_environment = 
            nestgate_core::canonical_modernization::CanonicalEnvironment::Development;
        config
    , /// Create production configuration optimized for high-load production
    #[must_use]
    pub fn production() -> Self {
        Self::create_for_environment("production")
     }

    /// Create high-performance configuration for maximum throughput
    pub fn high_performance() -> Self { Self::create_for_workload("high-performance")
    , /// Create testing configuration optimized for integration tests
    #[must_use]
    pub fn testing() -> Self {
        Self::create_for_environment("testing")
     }

    /// Create staging configuration for pre-production testing
    pub fn staging() -> Self { Self::create_for_environment("staging")
     }

impl Default for UnifiedApiExtensions {
    /// Returns the default instance
    fn default() -> Self { Self {
            http_server: ApiHttpServerSettings::default(),
            streaming: ApiStreamingSettings::default(),
            service_mesh: ApiServiceMeshSettings::default(),
            sse: ApiSseSettings::default(),
            primal: ApiPrimalSettings::default(),
            auth: ApiAuthSettings::default(),
            performance: ApiPerformanceSettings::default(),
            health: ApiHealthSettings::default(),
            storage: ApiStorageSettings::default(),
     }
    }

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Unifiedapiconfigcanonical
pub type UnifiedApiConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using UnifiedApiConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

