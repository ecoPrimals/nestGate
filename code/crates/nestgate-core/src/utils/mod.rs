//! # Utilities Module
//! Module definitions and exports.
// **100% SAFE RUST** - System utilities using only safe code
//! Module definitions and exports.
// This module provides system operations, file handling, networking utilities,
//! and string processing without any unsafe code blocks.

pub mod fs;
pub mod memory_optimization;
pub mod network;
pub mod string;
pub mod system;

// **100% SAFE SYSTEM OPERATIONS** - Zero unsafe code
pub mod completely_safe_system;

// Re-export safe system operations for easy access
pub use completely_safe_system::{PrivilegeInfo, SafePrivilegeChecker, SafeSystemOps};

// Re-export key functions for convenience
pub use fs::*;
pub use network::*;
pub use string::*;
pub use system::*;
