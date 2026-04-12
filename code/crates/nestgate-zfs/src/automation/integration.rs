// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS automation integration shims.
//!
//! The `nestgate-automation` crate was deprecated in v4.7.0 — automation /
//! orchestration concerns are delegated to biomeOS.  These thin types remain
//! for API compatibility until callers migrate to [`DatasetAutomation`](super::engine::DatasetAutomation).

/// Check if ecosystem services are available for ZFS automation.
///
/// Returns `false` — ecosystem availability is determined at runtime via
/// capability IPC, not compile-time feature flags.
#[must_use]
pub const fn check_zfs_ecosystem_availability() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_ecosystem_availability() {
        let available = check_zfs_ecosystem_availability();
        assert!(!available);
    }
}
