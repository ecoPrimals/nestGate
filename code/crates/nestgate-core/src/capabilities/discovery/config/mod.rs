//
// This module provides a modularized implementation of unified dynamic discovery configuration,
// split from the original 909-line monolithic `unified_dynamic_config.rs` file.
//
// **MODULAR ORGANIZATION**:
// - `timeout.rs` - Timeout discovery settings and logic
// - `network.rs` - Network discovery configuration
// - `security.rs` - Security discovery settings
// - `environment.rs` - Environment discovery configuration
// - `storage.rs` - Storage discovery settings
// - `cache.rs` - Cache discovery configuration
// - `core.rs` - Core unified structures and coordination
//
// **ELIMINATES**: 909-line monolithic configuration file
// **PROVIDES**: Focused, maintainable discovery configuration modules

// ==================== MODULE DECLARATIONS ====================

/// Core unified structures and coordination
pub mod core;

/// Timeout discovery settings and logic
pub mod timeout;

/// Network discovery configuration
pub mod network;

/// Security discovery settings
pub mod security;

/// Environment discovery configuration
pub mod environment;

/// Storage discovery settings
pub mod storage;

/// Cache discovery configuration
pub mod cache;

// ==================== RE-EXPORTS ====================

// Re-export all functionality for backward compatibility
pub use core::*;
pub use timeout::*;
pub use network::*;
pub use security::*;
pub use environment::*;
pub use storage::*;
pub use cache::*;

// ==================== COMMON IMPORTS ====================

// Removed unused imports - using available types from canonical modernization
// Removed unused imports - using available types from canonical modernization
// Removed unused imports - using available types from canonical modernization

// **MODULARIZATION COMPLETE**
//
// The unified dynamic discovery configuration has been successfully split from a 909-line
// monolithic implementation into focused, maintainable modules following the proven patterns
// established in the canonical modernization process. 