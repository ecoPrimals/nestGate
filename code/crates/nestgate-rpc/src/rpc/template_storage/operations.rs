// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! CRUD and ranking operations on in-memory template storage.

use chrono::Utc;
use nestgate_types::error::{NestGateError, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::types::{GraphTemplate, TemplateMetadata};

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
    ///
    /// # Errors
    ///
    /// Returns [`NestGateError`](nestgate_types::error::NestGateError) with invalid input when
    /// `name`, `family_id`, or `user_id` is empty.
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
        self.templates
            .write()
            .await
            .entry(family_id.clone())
            .or_insert_with(HashMap::new)
            .insert(template_id.clone(), template);

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
    ///
    /// # Errors
    ///
    /// Returns [`NestGateError`](nestgate_types::error::NestGateError) as not found when there is
    /// no template bucket for `family_id`, or when `template_id` is not present under that family.
    pub async fn retrieve_template(
        &self,
        template_id: &str,
        family_id: &str,
    ) -> Result<GraphTemplate> {
        let template = self
            .templates
            .read()
            .await
            .get(family_id)
            .ok_or_else(|| NestGateError::not_found("No templates for this family"))?
            .get(template_id)
            .ok_or_else(|| NestGateError::not_found("Template not found"))?
            .clone();

        tracing::debug!(
            "Retrieved template '{}' for family '{}'",
            template_id,
            family_id
        );

        Ok(template)
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
    ///
    /// # Errors
    ///
    /// Returns [`NestGateError`](nestgate_types::error::NestGateError) as not found when
    /// `family_id` has no stored templates.
    pub async fn list_templates(
        &self,
        family_id: &str,
        user_id: Option<&str>,
        tags: Option<&[String]>,
        niche_type: Option<&str>,
        is_community: Option<bool>,
    ) -> Result<Vec<GraphTemplate>> {
        let mut templates: Vec<GraphTemplate> = self
            .templates
            .read()
            .await
            .get(family_id)
            .ok_or_else(|| NestGateError::not_found("No templates for this family"))?
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
    ///
    /// # Errors
    ///
    /// The current implementation always returns [`Ok`]; the [`Result`] is reserved for future
    /// validation or storage failures.
    pub async fn get_community_top(
        &self,
        niche_type: Option<&str>,
        limit: usize,
        min_usage: u64,
    ) -> Result<Vec<(GraphTemplate, f64)>> {
        // Collect all community templates across families
        let community_templates: Vec<GraphTemplate> = {
            let storage = self.templates.read().await;

            storage
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
                .collect()
        };

        // Calculate max usage for normalization
        let max_usage = {
            let m = community_templates
                .iter()
                .map(|t| t.metadata.usage_count)
                .max()
                .unwrap_or(1);
            // Template ranking denominator: usage counts normalized to float.
            #[expect(clippy::cast_precision_loss)]
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
    ///
    /// # Errors
    ///
    /// Returns [`NestGateError`](nestgate_types::error::NestGateError) as not found when
    /// `template_id` is missing under `family_id`.
    pub async fn increment_usage(&self, template_id: &str, family_id: &str) -> Result<()> {
        let updated = {
            let mut storage = self.templates.write().await;

            if let Some(family_templates) = storage.get_mut(family_id)
                && let Some(template) = family_templates.get_mut(template_id)
            {
                template.metadata.usage_count += 1;
                template.updated_at = Utc::now();
                true
            } else {
                false
            }
        };

        if updated {
            tracing::debug!("Incremented usage for template '{}'", template_id);
            Ok(())
        } else {
            Err(NestGateError::not_found("Template not found"))
        }
    }

    /// Update success rate for a template
    ///
    /// # Errors
    ///
    /// Returns [`NestGateError`](nestgate_types::error::NestGateError) as invalid input when
    /// `success_rate` is outside `0.0..=1.0`, or as not found when the template does not exist
    /// under `family_id`.
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

        let updated = {
            let mut storage = self.templates.write().await;

            if let Some(family_templates) = storage.get_mut(family_id)
                && let Some(template) = family_templates.get_mut(template_id)
            {
                template.metadata.success_rate = success_rate;
                template.updated_at = Utc::now();
                true
            } else {
                false
            }
        };

        if updated {
            tracing::debug!(
                "Updated success rate for template '{}' to {}",
                template_id,
                success_rate
            );
            Ok(())
        } else {
            Err(NestGateError::not_found("Template not found"))
        }
    }
}
