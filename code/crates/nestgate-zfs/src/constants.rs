//
// This module provides ZFS-specific constants that are not yet integrated
// into the canonical constants system.

/// ZFS compression setting for disabled compression
pub const COMPRESSION_OFF: &str = "off";
/// ZFS record size constants
pub const RECORDSIZE_128K: &str = "128K";
pub const RECORDSIZE_1M: &str = "1M";
pub const RECORDSIZE_64K: &str = "64K";
// Re-export commonly used constants
pub use nestgate_core::canonical_modernization::canonical_constants::storage::*;
