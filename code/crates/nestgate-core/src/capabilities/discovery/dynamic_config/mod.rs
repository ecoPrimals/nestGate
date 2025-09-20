//
// This module provides modularized dynamic discovery configuration,
// split from the original 909-line monolithic file for better maintainability.
//
// **MODULAR ORGANIZATION**:
// - `settings.rs` - Configuration settings and structures
// - `manager.rs` - Main discovery manager implementation
// - `timeout.rs` - Timeout configuration and management
// - `network.rs` - Network discovery configuration
// - `security.rs` - Security discovery configuration
// - `environment.rs` - Environment discovery configuration
// - `storage.rs` - Storage discovery configuration
// - `cache.rs` - Cache discovery configuration

// ==================== SECTION ====================

// Configuration settings and structures
pub mod settings;
// Main discovery manager implementation
pub mod manager;
// Timeout configuration and management
pub mod timeout;
// Network discovery configuration
pub mod network;
// Security discovery configuration
pub mod security;
// Environment discovery configuration
pub mod environment;
// Storage discovery configuration
pub mod storage;
// Cache discovery configuration
pub mod cache;
// ==================== SECTION ====================

pub use settings::*;
pub use manager::*;
pub use timeout::*;
pub use network::*;
pub use security::*;
pub use storage::*;
pub use cache::*; 