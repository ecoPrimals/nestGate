//! Network Filesystem Module
//!
//! **UNIVERSAL ARCHITECTURE** - Phase 2 Task 3 (Jan 31, 2026)
//!
//! Provides network filesystem support with universal mount detection.

mod backend;
pub mod mount_detection;

pub use backend::*;
pub use mount_detection::{UniversalMountDetector, DiscoveredMount};
