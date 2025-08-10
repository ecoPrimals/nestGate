use super::api_settings::*;
/// **UNIFIED API CONFIGURATION - CORE MODULE**
/// Contains the main UnifiedApiConfig struct and core configuration logic.
/// This eliminates fragmented API config structs across multiple modules.
use nestgate_core::unified_config_consolidation::StandardDomainConfig;
use serde::{Deserialize, Serialize};

/// **UNIFIED API EXTENSIONS**
/// Consolidates all API-specific configuration patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// **UNIFIED API CONFIGURATION**
/// The single source of truth for all API configuration across the system
pub type UnifiedApiConfig = StandardDomainConfig<UnifiedApiExtensions>;

impl UnifiedApiConfig {
    /// Create development configuration optimized for local development
    pub fn development() -> Self {
        Self::create_for_environment("development")
    }

    /// Create production configuration optimized for high-load production
    pub fn production() -> Self {
        Self::create_for_environment("production")
    }

    /// Create high-performance configuration for maximum throughput
    pub fn high_performance() -> Self {
        Self::create_for_workload("high-performance")
    }

    /// Create testing configuration optimized for integration tests
    pub fn testing() -> Self {
        Self::create_for_environment("testing")
    }

    /// Create staging configuration for pre-production testing
    pub fn staging() -> Self {
        Self::create_for_environment("staging")
    }
    }

impl Default for UnifiedApiExtensions {
    fn default() -> Self {
        Self {
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
    }
