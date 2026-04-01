// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Safe UID retrieval - 100% Pure Rust, Zero unsafe code
//!
//! Provides safe abstractions for getting the current user ID across platforms.
//!
//! ## Evolution: Unsafe C → Safe Pure Rust
//! ```rust,ignore
//! // ❌ OLD (unsafe C via libc):
//! let uid = unsafe { libc::getuid() };
//!
//! // ✅ NEW (safe pure Rust via uzers):
//! let uid = nestgate_core::platform::get_current_uid();
//! ```
//!
//! ## Pure Rust Evolution
//! - **Before**: `libc::getuid()` (unsafe C binding)
//! - **After**: `uzers::get_current_uid()` (100% safe Rust)
//! - **Result**: Zero unsafe blocks, better cross-platform support

/// Get the current user ID (100% safe pure Rust, cross-platform)
///
/// # Platform Support
/// - **Unix/Linux**: Uses `uzers` crate (pure Rust)
/// - **macOS**: Uses `uzers` crate (pure Rust)
/// - **Windows**: Returns a placeholder (0) - SIDs out of scope
///
/// # Pure Rust Evolution
/// This function has been evolved from `unsafe { libc::getuid() }` to
/// pure Rust using the `uzers` crate. Zero unsafe code!
///
/// # Examples
/// ```
/// use nestgate_platform::get_current_uid;
///
/// let uid = get_current_uid();
/// println!("Current UID: {}", uid);
/// ```
#[inline]
#[must_use]
pub fn get_current_uid() -> u32 {
    #[cfg(unix)]
    {
        // ✅ PURE RUST! No unsafe code!
        // Uses uzers crate for safe UID retrieval
        uzers::get_current_uid()
    }

    #[cfg(not(unix))]
    {
        // Windows doesn't have UIDs in the same sense - return a placeholder
        // Real Windows support would use SIDs, but that's out of scope
        0
    }
}

/// Get the current effective group ID (100% safe pure Rust on Unix via `uzers`).
///
/// On non-Unix platforms, returns `0` as a placeholder (same rationale as [`get_current_uid`]).
#[inline]
#[must_use]
pub fn get_current_gid() -> u32 {
    #[cfg(unix)]
    {
        uzers::get_current_gid()
    }

    #[cfg(not(unix))]
    {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_uid() {
        let _uid = get_current_uid();
        // UID retrieval should not panic
        // On Unix: Returns actual UID (u32)
        // On Windows: Returns 0 (placeholder)
        // Success if no panic!
    }

    #[test]
    #[cfg(unix)]
    fn test_uid_consistency() {
        // UID should be consistent across calls
        let uid1 = get_current_uid();
        let uid2 = get_current_uid();
        assert_eq!(uid1, uid2);
    }

    #[test]
    #[cfg(unix)]
    fn test_gid_consistency() {
        let gid1 = get_current_gid();
        let gid2 = get_current_gid();
        assert_eq!(gid1, gid2);
    }
}
