// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Safe UID retrieval — pure Rust via `rustix`, zero unsafe code.
//!
//! ## Evolution: libc → uzers → rustix
//!
//! ```text
//! libc::getuid()         — unsafe C binding
//! uzers::get_current_uid() — safe wrapper, extra dep
//! rustix::process::getuid() — safe syscall wrapper, already in dep tree
//! ```

/// Get the current user ID (safe, pure Rust via `rustix`).
#[inline]
#[must_use]
pub fn get_current_uid() -> u32 {
    #[cfg(unix)]
    {
        rustix::process::getuid().as_raw()
    }

    #[cfg(not(unix))]
    {
        0
    }
}

/// Get the current effective group ID (safe, pure Rust via `rustix`).
#[inline]
#[must_use]
pub fn get_current_gid() -> u32 {
    #[cfg(unix)]
    {
        rustix::process::getgid().as_raw()
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
    }

    #[test]
    #[cfg(unix)]
    fn test_uid_consistency() {
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
