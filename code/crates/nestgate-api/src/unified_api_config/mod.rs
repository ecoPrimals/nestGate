/// **UNIFIED API CONFIGURATION MODULE**
/// Consolidates all fragmented API configuration structs into the StandardDomainConfig pattern.
/// This eliminates configuration fragmentation across the API crate.

// Core unified API configuration
pub mod api_core;
pub mod api_settings;

// New consolidated primal configuration
pub mod primal_extensions;

// New unified handler configurations
pub mod handlers;

// Migration utilities have been removed - no longer needed
// All configurations have been successfully migrated to the unified system

// Re-export all public types for backward compatibility
pub use api_core::{UnifiedApiConfig, UnifiedApiExtensions};
pub use primal_extensions::{UnifiedPrimalConfig, UnifiedPrimalExtensions};
pub use handlers::{UnifiedApiHandlerConfig, ApiHandlerExtensions};

// Re-export settings types
pub use api_settings::*;

/// **DEPRECATED MODULES** - Use UnifiedApiConfig instead
/// The following modules are deprecated and will be removed in a future version:
/// - universal_primal_config.rs -> Use UnifiedPrimalConfig
/// - config/primal.rs -> Use UnifiedPrimalConfig
/// - config/storage.rs -> Use UnifiedApiConfig.extensions.storage
/// - config/network.rs -> Use UnifiedApiConfig.network
/// 
/// **MIGRATION GUIDE**:
/// ```rust
/// // OLD: Fragmented configs
/// let primal_config = UniversalNestGateConfig { ... };
/// let storage_config = StorageConfig { ... };
/// let network_config = ServerConfig { ... };
/// 
/// // NEW: Unified config
/// let config = UnifiedApiConfig {
///     // Base unified configs (network, security, monitoring, storage, memory)
///     network: UnifiedNetworkConfig::default(),
///     security: UnifiedSecurityConfig::default(),
///     // ... other base configs
///     
///     // API-specific extensions
///     extensions: UnifiedApiExtensions {
///         http_server: ApiHttpServerSettings::default(),
///         streaming: ApiStreamingSettings::default(),
///         // ... other API settings
///     },
///     // ... other fields
/// };
/// 
/// // For primal-specific configuration
/// let primal_config = UnifiedPrimalConfig {
///     // Base unified configs
///     network: UnifiedNetworkConfig::default(),
///     // ... other base configs
///     
///     // Primal-specific extensions
///     extensions: UnifiedPrimalExtensions {
///         ecosystem: PrimalEcosystemSettings::default(),
///         discovery: PrimalDiscoverySettings::default(),
///         // ... other primal settings
///     },
///     // ... other fields
/// };
/// ```
