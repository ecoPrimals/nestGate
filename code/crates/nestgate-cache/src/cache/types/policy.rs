// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Cache write/read policy.

use serde::{Deserialize, Serialize};

/// Cache policy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
/// Cachepolicy
pub enum CachePolicy {
    /// No caching
    None,
    /// Read-only caching
    ReadOnly,
    /// Write-through caching (writes go to both cache and backing store)
    #[default]
    /// Writethrough
    WriteThrough,
    /// Write-back caching (writes go to cache, then are flushed to backing store)
    WriteBack,
}

impl std::fmt::Display for CachePolicy {
    /// Fmt
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::ReadOnly => write!(f, "read-only"),
            Self::WriteThrough => write!(f, "write-through"),
            Self::WriteBack => write!(f, "write-back"),
        }
    }
}
