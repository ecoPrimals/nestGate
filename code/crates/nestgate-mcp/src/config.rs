// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **MIGRATED MCP CONFIGURATION MODULE**
//!
//! This module now uses the canonical configuration system instead of
//! scattered MCP-specific configuration structures. All MCP configurations
//! are now part of the unified canonical configuration.

// Re-export from canonical configuration system
pub use nestgate_core::config::canonical_primary::{McpConfig, NestGateCanonicalConfig};

// Removed unused imports for pedantic perfection

// ==================== MIGRATION COMPLETE ====================
//
// All deprecated MCP configuration structures have been removed.
// Use the canonical configuration system instead:
//
// ```rust
// use nestgate_core::config::canonical_primary::{NestGateCanonicalConfig, McpConfig};
//
// let config = NestGateCanonicalConfig::default();
// let mcp_config = config.services.mcp;
// ```

// ==================== CONVENIENCE FUNCTIONS ====================

/// Create a new canonical MCP configuration
#[must_use]
pub fn new_mcp_config() -> McpConfig {
    McpConfig::default()
}
/// Create a development-optimized MCP configuration
#[must_use]
pub fn dev_mcp_config() -> McpConfig {
    // Development-specific optimizations would go here
    McpConfig::default()
}
/// Create a production-optimized MCP configuration
#[must_use]
pub fn prod_mcp_config() -> McpConfig {
    // Production-specific optimizations would go here
    McpConfig::default()
}
