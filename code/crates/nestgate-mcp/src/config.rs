//! **MIGRATED MCP CONFIGURATION MODULE**
//!
//! This module now uses the canonical configuration system instead of
//! scattered MCP-specific configuration structures. All MCP configurations
//! are now part of the unified canonical configuration.

// Re-export from canonical configuration system
pub use nestgate_core::config::canonical_master::{McpConfig, NestGateCanonicalConfig};

// Removed unused imports for pedantic perfection

// ==================== MIGRATION COMPLETE ====================
//
// All deprecated MCP configuration structures have been removed.
// Use the canonical configuration system instead:
//
// ```rust
// use nestgate_core::config::canonical_master::{NestGateCanonicalConfig, McpConfig};
//
// let config = NestGateCanonicalConfig::default();
// let mcp_config = config.services.mcp;
// ```

// ==================== CONVENIENCE FUNCTIONS ====================

/// Create a new canonical MCP configuration
pub const fn new_mcp_config() -> McpConfig {
    McpConfig::default()
}
/// Create a development-optimized MCP configuration
pub const fn dev_mcp_config() -> McpConfig {
    // Development-specific optimizations would go here
    McpConfig::default()
}
/// Create a production-optimized MCP configuration
pub const fn prod_mcp_config() -> McpConfig {
    // Production-specific optimizations would go here
    McpConfig::default()
}
