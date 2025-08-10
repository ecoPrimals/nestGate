/// NestGate API Configuration System
/// **MODERNIZED**: Unified configuration system with standardized patterns.
/// Legacy individual config modules are deprecated in favor of unified approach.

// Re-export the unified configuration system
pub mod unified_api_config;
pub use unified_api_config::{UnifiedApiConfig, ApiExtensions};

// **DEPRECATED MODULES REMOVED**
// Use unified_api_config::UnifiedApiConfig for all API configurations.

// Modern unified configuration - THE way to configure API services
pub type Config = UnifiedApiConfig;

/// Create a default API configuration
pub fn default_config() -> UnifiedApiConfig {
    UnifiedApiConfig::development()
    }

/// Create a production API configuration  
pub fn production_config() -> UnifiedApiConfig {
    UnifiedApiConfig::production()
    }

// Migration utilities removed - all configurations now use UnifiedApiConfig directly
