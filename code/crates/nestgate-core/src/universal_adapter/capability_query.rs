// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! [`CapabilityQuery`] — filter structure for capability discovery via the universal adapter.

use serde::{Deserialize, Serialize};

/// Query for discovering capabilities through universal adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Capabilityquery
pub struct CapabilityQuery {
    /// Capability
    pub capability: String,
    /// Operation
    pub operation: Option<String>,
    /// Filters
    pub filters: Vec<String>,
}

impl CapabilityQuery {
    /// Creates a new capability query with the specified capability type
    #[must_use]
    pub fn new(capability_type: impl Into<String>) -> Self {
        Self {
            capability: capability_type.into(),
            operation: None,
            filters: Vec::new(),
        }
    }

    /// Create a search query for a specific capability
    pub fn search(capability_type: impl Into<String>) -> Self {
        Self::new(capability_type)
    }

    /// Adds an operation filter to the capability query
    #[must_use]
    pub fn with_operation(mut self, operation: impl Into<String>) -> Self {
        self.operation = Some(operation.into());
        self
    }

    /// Adds a custom filter to the query
    #[must_use]
    pub fn with_filter(mut self, filter: impl Into<String>) -> Self {
        self.filters.push(filter.into());
        self
    }
}
