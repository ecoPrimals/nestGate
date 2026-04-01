// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! [`ZfsHandlerImpl`] marker type used by [`crate::handlers::ZfsHandler`].

/// ZFS handler implementation for the API
#[derive(Debug, Clone)]
/// Zfshandlerimpl
pub struct ZfsHandlerImpl;

impl Default for ZfsHandlerImpl {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl ZfsHandlerImpl {
    /// Create a new ZFS handler instance
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
