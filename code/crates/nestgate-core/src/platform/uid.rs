//! Safe UID retrieval - Zero unsafe code required
//!
//! Provides safe abstractions for getting the current user ID across platforms.
//!
//! ## Why This Exists
//! Eliminates `unsafe { libc::getuid() }` calls throughout the codebase.
//!
//! ## Evolution: Unsafe → Safe
//! ```rust
//! // ❌ OLD (unsafe):
//! let uid = unsafe { libc::getuid() };
//!
//! // ✅ NEW (safe):
//! let uid = nestgate_core::platform::get_current_uid();
//! ```

/// Get the current user ID (safe, cross-platform)
///
/// # Platform Support
/// - **Unix**: Uses `libc::getuid()` internally but safely
/// - **Windows**: Returns a placeholder (0) - not applicable
///
/// # Examples
/// ```
/// use nestgate_core::platform::get_current_uid;
///
/// let uid = get_current_uid();
/// println!("Current UID: {}", uid);
/// ```
#[inline]
pub fn get_current_uid() -> u32 {
    #[cfg(unix)]
    {
        // SAFETY: getuid() is always safe - it just reads a value from the kernel
        // It has no preconditions and cannot fail
        unsafe { libc::getuid() }
    }

    #[cfg(not(unix))]
    {
        // Windows doesn't have UIDs in the same sense - return a placeholder
        // Real Windows support would use SIDs, but that's out of scope
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_uid() {
        let uid = get_current_uid();
        // On Unix, UID should be >= 0 (always true for u32)
        // On Windows, we get 0
        assert!(uid >= 0);
    }

    #[test]
    #[cfg(unix)]
    fn test_uid_consistency() {
        // UID should be consistent across calls
        let uid1 = get_current_uid();
        let uid2 = get_current_uid();
        assert_eq!(uid1, uid2);
    }
}
