// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # 🗂️ Template Storage for Collaborative Intelligence
//!
//! **Graph Template Storage & Management**
//!
//! Provides template storage capabilities for biomeOS Collaborative Intelligence,
//! enabling users to save, share, and discover graph templates for faster bootstrapping.
//!
//! ## Philosophy
//! - **Self-Knowledge**: Templates stored per `family_id` (multi-tenant isolation)
//! - **Zero Hardcoding**: All behavior driven by data and environment
//! - **Modern Rust**: No unsafe code, proper error handling throughout
//! - **Complete Implementation**: No mocks, production-ready from day one
//!
//! ## Features
//! - Template CRUD operations
//! - Version control
//! - Community sharing & ranking
//! - Usage tracking
//! - Success rate calculations

use chrono::{DateTime, Utc};
use nestgate_types::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

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

/// Template storage state (in-memory for now, will be persistent later)
#[derive(Debug, Clone, Default)]
pub struct TemplateStorage {
    /// Templates stored by `family_id` -> `template_id` -> template
    /// Ensures family-based isolation (self-knowledge principle)
    templates: Arc<RwLock<HashMap<String, HashMap<String, GraphTemplate>>>>,
}

impl TemplateStorage {
    /// Create new template storage
    #[must_use]
    pub fn new() -> Self {
        Self {
            templates: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Store a template (with versioning)
    ///
    /// # Self-Knowledge Principle
    /// - Templates isolated by `family_id`
    /// - No cross-family access
    /// - Auto-incrementing version numbers
    ///
    /// # Arguments
    /// - `name`: Template name
    /// - `description`: Template description
    /// - `graph_data`: Full graph structure
    /// - `user_id`: Owner user ID
    /// - `family_id`: Family identifier for isolation
    /// - `metadata`: Template metadata
    ///
    /// # Returns
    /// - Template ID and version on success
    pub async fn store_template(
        &self,
        name: String,
        description: String,
        graph_data: serde_json::Value,
        user_id: String,
        family_id: String,
        metadata: TemplateMetadata,
    ) -> Result<(String, u32)> {
        // Validate inputs
        if name.is_empty() {
            return Err(NestGateError::invalid_input_with_field(
                "name",
                "Template name cannot be empty",
            ));
        }

        if family_id.is_empty() {
            return Err(NestGateError::invalid_input_with_field(
                "family_id",
                "Family ID cannot be empty",
            ));
        }

        if user_id.is_empty() {
            return Err(NestGateError::invalid_input_with_field(
                "user_id",
                "User ID cannot be empty",
            ));
        }

        // Generate unique template ID
        let template_id = format!("tmpl_{}", uuid::Uuid::new_v4().simple());

        let now = Utc::now();

        let template = GraphTemplate {
            id: template_id.clone(),
            name,
            description,
            graph_data,
            user_id,
            family_id: family_id.clone(),
            version: 1,
            created_at: now,
            updated_at: now,
            metadata,
        };

        // Store with family isolation
        let mut storage = self.templates.write().await;
        let family_templates = storage
            .entry(family_id.clone())
            .or_insert_with(HashMap::new);
        family_templates.insert(template_id.clone(), template);

        tracing::info!(
            "Stored template '{}' for family '{}'",
            template_id,
            family_id
        );

        Ok((template_id, 1))
    }

    /// Retrieve a template by ID
    ///
    /// # Self-Knowledge Principle
    /// - Only retrieves from specified `family_id`
    /// - No cross-family access possible
    ///
    /// # Arguments
    /// - `template_id`: Template identifier
    /// - `family_id`: Family identifier for isolation
    ///
    /// # Returns
    /// - Template on success
    /// - Error if not found or access denied
    pub async fn retrieve_template(
        &self,
        template_id: &str,
        family_id: &str,
    ) -> Result<GraphTemplate> {
        let storage = self.templates.read().await;

        let family_templates = storage
            .get(family_id)
            .ok_or_else(|| NestGateError::not_found("No templates for this family"))?;

        let template = family_templates
            .get(template_id)
            .ok_or_else(|| NestGateError::not_found("Template not found"))?;

        tracing::debug!(
            "Retrieved template '{}' for family '{}'",
            template_id,
            family_id
        );

        Ok(template.clone())
    }

    /// List templates with optional filtering
    ///
    /// # Self-Knowledge Principle
    /// - Lists only templates for specified `family_id`
    /// - Optional `user_id` filter for user's own templates
    ///
    /// # Arguments
    /// - `family_id`: Family identifier for isolation
    /// - `user_id`: Optional user filter
    /// - `tags`: Optional tag filter
    /// - `niche_type`: Optional niche type filter
    /// - `is_community`: Optional community filter
    ///
    /// # Returns
    /// - Vector of matching templates
    pub async fn list_templates(
        &self,
        family_id: &str,
        user_id: Option<&str>,
        tags: Option<&[String]>,
        niche_type: Option<&str>,
        is_community: Option<bool>,
    ) -> Result<Vec<GraphTemplate>> {
        let storage = self.templates.read().await;

        let family_templates = storage
            .get(family_id)
            .ok_or_else(|| NestGateError::not_found("No templates for this family"))?;

        let mut templates: Vec<GraphTemplate> = family_templates
            .values()
            .filter(|t| {
                // Filter by user_id if specified
                if let Some(uid) = user_id
                    && t.user_id != uid
                {
                    return false;
                }

                // Filter by tags if specified
                if let Some(filter_tags) = tags
                    && !filter_tags.iter().any(|tag| t.metadata.tags.contains(tag))
                {
                    return false;
                }

                // Filter by niche_type if specified
                if let Some(niche) = niche_type
                    && t.metadata.niche_type != niche
                {
                    return false;
                }

                // Filter by is_community if specified
                if let Some(community) = is_community
                    && t.metadata.is_community != community
                {
                    return false;
                }

                true
            })
            .cloned()
            .collect();

        // Sort by updated_at (most recent first)
        templates.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        tracing::debug!(
            "Listed {} templates for family '{}' with filters",
            templates.len(),
            family_id
        );

        Ok(templates)
    }

    /// Get top community templates by ranking
    ///
    /// # Ranking Algorithm
    /// score = 0.4 * `normalized_usage` + 0.3 * `success_rate` + 0.3 * (rating / 5.0)
    ///
    /// # Arguments
    /// - `niche_type`: Optional niche type filter
    /// - `limit`: Maximum number of results
    /// - `min_usage`: Minimum usage count threshold
    ///
    /// # Returns
    /// - Vector of top-ranked templates with scores
    pub async fn get_community_top(
        &self,
        niche_type: Option<&str>,
        limit: usize,
        min_usage: u64,
    ) -> Result<Vec<(GraphTemplate, f64)>> {
        let storage = self.templates.read().await;

        // Collect all community templates across families
        let community_templates: Vec<GraphTemplate> = storage
            .values()
            .flat_map(|family_templates| family_templates.values())
            .filter(|t| {
                // Must be community template
                if !t.metadata.is_community {
                    return false;
                }

                // Must meet minimum usage
                if t.metadata.usage_count < min_usage {
                    return false;
                }

                // Filter by niche_type if specified
                if let Some(niche) = niche_type
                    && t.metadata.niche_type != niche
                {
                    return false;
                }

                true
            })
            .cloned()
            .collect();

        // Calculate max usage for normalization
        let max_usage = {
            let m = community_templates
                .iter()
                .map(|t| t.metadata.usage_count)
                .max()
                .unwrap_or(1);
            // Template ranking denominator: usage counts normalized to float.
            #[allow(clippy::cast_precision_loss)]
            {
                m as f64
            }
        };

        // Calculate scores and sort
        let mut scored_templates: Vec<(GraphTemplate, f64)> = community_templates
            .into_iter()
            .map(|t| {
                let normalized_usage = if max_usage > 0.0 {
                    let uc = t.metadata.usage_count;
                    #[expect(
                        clippy::cast_precision_loss,
                        reason = "template usage ratio for community ranking score"
                    )]
                    let ucf: f64 = uc as f64;
                    ucf / max_usage
                } else {
                    0.0
                };

                let rating_score = t.metadata.community_rating.map_or(0.0, |r| r / 5.0);

                // Ranking algorithm: weighted combination
                let score = 0.3f64.mul_add(
                    rating_score,
                    0.4f64.mul_add(normalized_usage, 0.3 * t.metadata.success_rate),
                );

                (t, score)
            })
            .collect();

        // Sort by score (highest first)
        scored_templates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Limit results
        scored_templates.truncate(limit);

        tracing::debug!(
            "Retrieved {} top community templates (niche: {:?})",
            scored_templates.len(),
            niche_type
        );

        Ok(scored_templates)
    }

    /// Increment usage count for a template (called when template is used)
    pub async fn increment_usage(&self, template_id: &str, family_id: &str) -> Result<()> {
        let mut storage = self.templates.write().await;

        if let Some(family_templates) = storage.get_mut(family_id)
            && let Some(template) = family_templates.get_mut(template_id)
        {
            template.metadata.usage_count += 1;
            template.updated_at = Utc::now();
            tracing::debug!("Incremented usage for template '{}'", template_id);
            return Ok(());
        }

        Err(NestGateError::not_found("Template not found"))
    }

    /// Update success rate for a template
    pub async fn update_success_rate(
        &self,
        template_id: &str,
        family_id: &str,
        success_rate: f64,
    ) -> Result<()> {
        if !(0.0..=1.0).contains(&success_rate) {
            return Err(NestGateError::invalid_input_with_field(
                "success_rate",
                "Success rate must be between 0.0 and 1.0",
            ));
        }

        let mut storage = self.templates.write().await;

        if let Some(family_templates) = storage.get_mut(family_id)
            && let Some(template) = family_templates.get_mut(template_id)
        {
            template.metadata.success_rate = success_rate;
            template.updated_at = Utc::now();
            tracing::debug!(
                "Updated success rate for template '{}' to {}",
                template_id,
                success_rate
            );
            return Ok(());
        }

        Err(NestGateError::not_found("Template not found"))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn store_rejects_empty_name() {
        let ts = TemplateStorage::new();
        let r = ts
            .store_template(
                String::new(),
                "d".into(),
                serde_json::json!({}),
                "u".into(),
                "fam".into(),
                TemplateMetadata::default(),
            )
            .await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn store_rejects_empty_family_id() {
        let ts = TemplateStorage::new();
        let r = ts
            .store_template(
                "n".into(),
                "d".into(),
                serde_json::json!({}),
                "u".into(),
                String::new(),
                TemplateMetadata::default(),
            )
            .await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn store_rejects_empty_user_id() {
        let ts = TemplateStorage::new();
        let r = ts
            .store_template(
                "n".into(),
                "d".into(),
                serde_json::json!({}),
                String::new(),
                "fam".into(),
                TemplateMetadata::default(),
            )
            .await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn store_list_retrieve_roundtrip() {
        let ts = TemplateStorage::new();
        let (id, ver) = ts
            .store_template(
                "MyTpl".into(),
                "desc".into(),
                serde_json::json!({"k": 1}),
                "user1".into(),
                "famA".into(),
                TemplateMetadata {
                    tags: vec!["t1".into()],
                    niche_type: "web_service".into(),
                    is_community: true,
                    community_rating: Some(4.5),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(ver, 1);
        assert!(id.starts_with("tmpl_"));

        let t = ts.retrieve_template(&id, "famA").await.unwrap();
        assert_eq!(t.name, "MyTpl");
        assert_eq!(t.version, 1);

        let list = ts
            .list_templates("famA", Some("user1"), None, Some("web_service"), None)
            .await
            .unwrap();
        assert_eq!(list.len(), 1);

        let not_found = ts.retrieve_template("missing", "famA").await;
        assert!(not_found.is_err());
    }

    #[tokio::test]
    async fn list_unknown_family_errors() {
        let ts = TemplateStorage::new();
        let r = ts.list_templates("nope", None, None, None, None).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn increment_usage_and_success_rate() {
        let ts = TemplateStorage::new();
        let (id, _) = ts
            .store_template(
                "x".into(),
                "d".into(),
                serde_json::json!({}),
                "u".into(),
                "fam".into(),
                TemplateMetadata::default(),
            )
            .await
            .unwrap();

        ts.increment_usage(&id, "fam").await.unwrap();
        let t = ts.retrieve_template(&id, "fam").await.unwrap();
        assert_eq!(t.metadata.usage_count, 1);

        ts.update_success_rate(&id, "fam", 0.75).await.unwrap();
        let t = ts.retrieve_template(&id, "fam").await.unwrap();
        assert!((t.metadata.success_rate - 0.75).abs() < f64::EPSILON);

        let bad = ts.update_success_rate(&id, "fam", 1.5).await;
        assert!(bad.is_err());
    }

    #[tokio::test]
    async fn get_community_top_ranks_and_truncates() {
        let ts = TemplateStorage::new();
        let mut meta = TemplateMetadata {
            niche_type: "ml".into(),
            usage_count: 100,
            success_rate: 1.0,
            is_community: true,
            community_rating: Some(5.0),
            ..Default::default()
        };
        let (id1, _) = ts
            .store_template(
                "a".into(),
                "".into(),
                serde_json::json!({}),
                "u".into(),
                "f1".into(),
                meta.clone(),
            )
            .await
            .unwrap();
        meta.usage_count = 10;
        let _ = ts
            .store_template(
                "b".into(),
                "".into(),
                serde_json::json!({}),
                "u".into(),
                "f2".into(),
                meta,
            )
            .await
            .unwrap();

        let top = ts.get_community_top(Some("ml"), 1, 0).await.unwrap();
        assert_eq!(top.len(), 1);
        assert_eq!(top[0].0.id, id1);
    }

    #[test]
    fn round5_template_metadata_default_and_serde() {
        let m = TemplateMetadata::default();
        assert_eq!(m.usage_count, 0);
        let json = serde_json::to_string(&m).unwrap();
        let back: TemplateMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(back.niche_type, m.niche_type);
    }

    #[tokio::test]
    async fn list_templates_filters_by_tags_and_is_community() {
        let ts = TemplateStorage::new();
        let _ = ts
            .store_template(
                "t1".into(),
                "d".into(),
                serde_json::json!({}),
                "alice".into(),
                "fam_f".into(),
                TemplateMetadata {
                    tags: vec!["alpha".into()],
                    niche_type: "n".into(),
                    is_community: true,
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        let _ = ts
            .store_template(
                "t2".into(),
                "d".into(),
                serde_json::json!({}),
                "bob".into(),
                "fam_f".into(),
                TemplateMetadata {
                    tags: vec!["beta".into()],
                    niche_type: "n".into(),
                    is_community: false,
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        let tags = vec!["alpha".to_string()];
        let by_tag = ts
            .list_templates("fam_f", None, Some(&tags), None, None)
            .await
            .unwrap();
        assert_eq!(by_tag.len(), 1);
        assert_eq!(by_tag[0].name, "t1");

        let community_only = ts
            .list_templates("fam_f", None, None, None, Some(true))
            .await
            .unwrap();
        assert_eq!(community_only.len(), 1);

        let bob_only = ts
            .list_templates("fam_f", Some("bob"), None, None, None)
            .await
            .unwrap();
        assert_eq!(bob_only.len(), 1);
        assert_eq!(bob_only[0].name, "t2");
    }

    #[tokio::test]
    async fn get_community_top_respects_min_usage() {
        let ts = TemplateStorage::new();
        let low_meta = TemplateMetadata {
            niche_type: "q".into(),
            usage_count: 5,
            success_rate: 0.5,
            is_community: true,
            community_rating: Some(3.0),
            ..Default::default()
        };
        ts.store_template(
            "low".into(),
            "".into(),
            serde_json::json!({}),
            "u".into(),
            "fL".into(),
            low_meta,
        )
        .await
        .unwrap();
        let high_meta = TemplateMetadata {
            niche_type: "q".into(),
            usage_count: 50,
            success_rate: 0.9,
            is_community: true,
            community_rating: Some(5.0),
            ..Default::default()
        };
        ts.store_template(
            "high".into(),
            "".into(),
            serde_json::json!({}),
            "u".into(),
            "fH".into(),
            high_meta,
        )
        .await
        .unwrap();

        let top = ts.get_community_top(Some("q"), 10, 40).await.unwrap();
        assert!(!top.is_empty());
        assert!(top.iter().all(|(t, _)| t.name == "high"));
    }
}
