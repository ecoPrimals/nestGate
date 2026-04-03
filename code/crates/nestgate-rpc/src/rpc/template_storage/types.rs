// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Graph template and metadata types for collaborative template storage.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Graph template with full metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphTemplate {
    /// Unique template identifier
    pub id: String,

    /// Human-readable template name
    pub name: String,

    /// Template description
    pub description: String,

    /// Full graph structure (JSON)
    pub graph_data: serde_json::Value,

    /// Owner user ID
    pub user_id: String,

    /// Family/app identifier for isolation
    pub family_id: String,

    /// Current version number
    pub version: u32,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,

    /// Template metadata
    pub metadata: TemplateMetadata,
}

/// Template metadata for search and ranking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    /// Search tags
    #[serde(default)]
    pub tags: Vec<String>,

    /// Niche type (e.g., "`web_service`", "`ml_pipeline`")
    #[serde(default)]
    pub niche_type: String,

    /// Number of times used
    #[serde(default)]
    pub usage_count: u64,

    /// Success rate (0.0 - 1.0)
    #[serde(default)]
    pub success_rate: f64,

    /// Available to community
    #[serde(default)]
    pub is_community: bool,

    /// Community rating (0.0 - 5.0)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub community_rating: Option<f64>,

    /// Number of ratings
    #[serde(default)]
    pub rating_count: u64,
}

impl Default for TemplateMetadata {
    fn default() -> Self {
        Self {
            tags: Vec::new(),
            niche_type: String::new(),
            usage_count: 0,
            success_rate: 0.0,
            is_community: false,
            community_rating: None,
            rating_count: 0,
        }
    }
}
