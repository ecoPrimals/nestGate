// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Access-pattern history and tier hints from observed events.

use std::collections::HashMap;
use std::time::SystemTime;

use nestgate_core::unified_enums::StorageTier;

use crate::types::prediction::{AccessEvent, AccessType};

use super::types::storage_tier_from_extension;

type PatternHistory = tokio::sync::RwLock<HashMap<String, Vec<AccessEvent>>>;

/// Pattern analyzer for tracking access patterns
#[derive(Debug)]
pub struct PatternAnalyzer {
    pattern_history: PatternHistory,
}

impl PatternAnalyzer {
    #[must_use]
    pub fn new() -> Self {
        Self {
            pattern_history: tokio::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Record an access event
    pub async fn record_access(&self, file_path: &str, access_type: AccessType) {
        let access_event = AccessEvent {
            file_path: file_path.to_string(),
            access_type,
            timestamp: SystemTime::now(),
            size_bytes: 0,
        };

        let mut history = self.pattern_history.write().await;
        history
            .entry(file_path.to_string())
            .or_insert_with(Vec::new)
            .push(access_event);
    }

    /// Get access patterns for a file
    pub async fn get_patterns(&self, file_path: &str) -> Vec<AccessEvent> {
        let history = self.pattern_history.read().await;
        history.get(file_path).cloned().unwrap_or_default()
    }

    /// Analyze patterns to determine storage tier recommendation
    pub fn recommend_tier(&self, file_path: &str) -> StorageTier {
        storage_tier_from_extension(file_path)
    }
}

impl Default for PatternAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
