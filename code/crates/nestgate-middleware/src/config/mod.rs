// Simplified, unified middleware configuration using canonical patterns

// Removed unused import - using canonical EnvironmentSettings instead
use nestgate_core::config::canonical_unified::NestGateCanonicalUnifiedConfig as NestGateFinalConfig;

/// Canonical middleware configuration
/// CANONICAL MODERNIZATION: Direct type alias to unified config
pub type MiddlewareConfig = NestGateFinalConfig;

/// Canonical unified middleware configuration
/// CANONICAL MODERNIZATION: Simplified type alias without type parameters
pub type UnifiedMiddlewareConfig = NestGateFinalConfig;

// ==================== MIDDLEWARE-SPECIFIC EXTENSIONS ====================

/// Middleware configuration factory using canonical patterns
pub struct MiddlewareConfigFactory;

impl MiddlewareConfigFactory {
    /// Create development configuration using canonical modernized patterns
    pub fn development() -> MiddlewareConfig {
        
        // Note: Using canonical config defaults
        // Legacy fields removed in favor of canonical patterns
        MiddlewareConfig::default()
    }

    /// Create production configuration using canonical modernized patterns
    pub fn production() -> MiddlewareConfig {
        
        // Note: Using canonical config defaults
        // Legacy fields removed in favor of canonical patterns
        MiddlewareConfig::default()
    }
}
