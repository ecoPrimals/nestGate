// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Individual cache entry with metadata.

use super::tier::StorageTier;
use chrono;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Cache entry with data and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cacheentry
pub struct CacheEntry {
    /// Entry key
    pub key: String,
    /// Actual cached data
    pub data: Vec<u8>,
    /// Entry size in bytes
    pub size: u64,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last access timestamp
    pub accessed_at: chrono::DateTime<chrono::Utc>,
    /// Access count
    pub access_count: u64,
    /// Current storage tier
    pub tier: StorageTier,
    /// Time to live
    pub ttl: Option<Duration>,
}

impl CacheEntry {
    /// Create a new cache entry
    #[must_use]
    pub fn new(key: String, data: Vec<u8>, tier: StorageTier) -> Self {
        let now = chrono::Utc::now();
        let size = data.len() as u64;
        Self {
            key,
            data,
            size,
            created_at: now,
            accessed_at: now,
            access_count: 0,
            tier,
            ttl: None,
        }
    }

    /// Check if entry has expired
    #[must_use]
    pub fn is_expired(&self) -> bool {
        self.ttl.is_some_and(|ttl| {
            let expiry_time = self.created_at + chrono::Duration::from_std(ttl).unwrap_or_default();
            chrono::Utc::now() > expiry_time
        })
    }

    /// Update access timestamp and count
    pub fn touch(&mut self) {
        self.accessed_at = chrono::Utc::now();
        self.access_count += 1;
    }

    /// Get age of entry
    #[must_use]
    pub fn age(&self) -> chrono::Duration {
        chrono::Utc::now() - self.created_at
    }
}
