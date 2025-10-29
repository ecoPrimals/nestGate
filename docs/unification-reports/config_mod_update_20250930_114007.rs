//! Configuration system module
//! 
//! **PRIMARY SYSTEM**: Use `canonical_master::NestGateCanonicalConfig`
//! All other config systems are deprecated.

// ==================== THE CANONICAL CONFIG SYSTEM ====================

/// **THE** canonical configuration system - use this for all new code
pub mod canonical_master;

// Re-export the canonical config and domain configs
pub use canonical_master::{
    NestGateCanonicalConfig,
    ConsolidatedDomainConfigs,
    domains::*,
};

// ==================== DEPRECATED CONFIG SYSTEMS ====================

#[deprecated(note = "Use canonical_master instead")]
pub mod canonical;

#[deprecated(note = "Use canonical_master instead")]
pub mod canonical_config;

#[deprecated(note = "Use canonical_master instead")]
pub mod canonical_unified;

#[deprecated(note = "Use canonical_master instead")]
pub mod unified_types;

#[deprecated(note = "Use canonical_master instead")]
pub mod unified_config_consolidation;

// ==================== HELPER MODULES ====================

/// Domain-specific configuration types (to be consolidated)
pub mod domains;

/// Configuration validation utilities
pub mod validation;

/// Builder patterns for configuration
pub mod builders;

/// Configuration defaults
pub mod defaults;

/// Dynamic configuration updates
pub mod dynamic_config;

/// Migration helpers (temporary - remove after migration)
pub mod migration_helpers;

// ==================== CONVENIENCE RE-EXPORTS ====================

/// Type alias for the canonical config
pub type Config = NestGateCanonicalConfig;

/// Result type for configuration operations
pub type ConfigResult<T> = Result<T, crate::error::NestGateUnifiedError>;
