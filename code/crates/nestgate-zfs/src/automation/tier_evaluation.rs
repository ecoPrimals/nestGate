// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// This module provides sophisticated algorithms for evaluating the optimal
// storage tier for datasets based on access patterns, size, performance
// requirements, and policy rules.

//! Tier Evaluation module

use std::time::SystemTime;
// Removed unused tracing import

use super::types::{AutomationPolicy, DatasetMetadata};
use crate::error::ZfsError;
use crate::types::StorageTier;
use tracing::debug;
use tracing::info;

/// Intelligent tier scoring system for dataset placement
#[derive(Debug)]
/// Tierscoring
pub struct TierScoring {
    /// Hot Score
    pub hot_score: f64,
    /// Warm Score
    pub warm_score: f64,
    /// Cold Score
    pub cold_score: f64,
    /// Hot Reasons
    pub hot_reasons: Vec<String>,
    /// Warm Reasons
    pub warm_reasons: Vec<String>,
    /// Cold Reasons
    pub cold_reasons: Vec<String>,
}
impl TierScoring {
    /// Create a new tier scoring instance
    #[must_use]
    pub const fn new() -> Self {
        Self {
            hot_score: 0.0,
            warm_score: 0.0,
            cold_score: 0.0,
            hot_reasons: Vec::new(),
            warm_reasons: Vec::new(),
            cold_reasons: Vec::new(),
        }
    }

    /// Add weight towards hot tier placement with reasoning
    pub fn add_hot_weight(&mut self, weight: f64, reason: &str) {
        self.hot_score += weight;
        self.hot_reasons.push(reason.to_string());
    }

    /// Add weight towards warm tier placement with reasoning
    pub fn add_warm_weight(&mut self, weight: f64, reason: &str) {
        self.warm_score += weight;
        self.warm_reasons.push(reason.to_string());
    }

    /// Add weight towards cold tier placement with reasoning
    pub fn add_cold_weight(&mut self, weight: f64, reason: &str) {
        self.cold_score += weight;
        self.cold_reasons.push(reason.to_string());
    }

    /// Get the recommended tier based on scoring
    #[must_use]
    pub fn get_recommendation(&self) -> StorageTier {
        if self.hot_score >= self.warm_score && self.hot_score >= self.cold_score {
            StorageTier::Hot
        } else if self.warm_score >= self.cold_score {
            StorageTier::Warm
        } else {
            StorageTier::Cold
        }
    }
}

impl Default for TierScoring {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

/// Evaluate the best tier for a dataset based on intelligent rules
pub fn evaluate_tier_by_intelligent_rules(
    dataset_name: &str,
    metadata: &DatasetMetadata,
    policies: &std::collections::HashMap<String, AutomationPolicy>,
) -> Result<StorageTier, ZfsError> {
    let mut tier_score = TierScoring::new();
    // 1. Size-based scoring (larger files tend toward cold storage)
    if metadata.size_bytes > 10 * 1024 * 1024 * 1024 {
        // >10GB
        tier_score.add_cold_weight(0.3, "Large dataset size");
    } else if metadata.size_bytes < 100 * 1024 * 1024 {
        // <100MB
        tier_score.add_hot_weight(0.4, "Small dataset size");
    } else {
        tier_score.add_warm_weight(0.2, "Medium dataset size");
    }

    // 2. Access frequency analysis
    let days_since_access = metadata
        .last_accessed
        .and_then(|last| SystemTime::now().duration_since(last).ok())
        .map_or(365, |d| d.as_secs() / (24 * 3600));

    match days_since_access {
        0..=1 => tier_score.add_hot_weight(0.5, "Accessed within 24 hours"),
        2..=7 => tier_score.add_warm_weight(0.4, "Accessed within week"),
        8..=30 => tier_score.add_warm_weight(0.2, "Accessed within month"),
        _ => tier_score.add_cold_weight(0.4, "Rarely accessed"),
    }

    // 3. Access frequency scoring
    match metadata.access_frequency as u32 {
        freq if freq > 100 => tier_score.add_hot_weight(0.6, "Very high access frequency"),
        freq if freq > 20 => tier_score.add_hot_weight(0.3, "High access frequency"),
        freq if freq > 5 => tier_score.add_warm_weight(0.3, "Moderate access frequency"),
        freq if freq > 1 => tier_score.add_warm_weight(0.1, "Low access frequency"),
        _ => tier_score.add_cold_weight(0.3, "Very low access frequency"),
    }

    // 4. Dataset name pattern analysis
    let dataset_lower = dataset_name.to_lowercase();
    if dataset_lower.contains("cache") || dataset_lower.contains("temp") {
        tier_score.add_hot_weight(0.4, "Cache/temp dataset pattern");
    } else if dataset_lower.contains("archive") || dataset_lower.contains("backup") {
        tier_score.add_cold_weight(0.5, "Archive/backup dataset pattern");
    } else if dataset_lower.contains("log") || dataset_lower.contains("data") {
        tier_score.add_warm_weight(0.2, "Log/data dataset pattern");
    }

    // 5. File type analysis
    for file_type in &metadata.file_types {
        match file_type.to_lowercase().as_str() {
            "mp4" | "avi" | "mkv" | "mov" => {
                tier_score.add_cold_weight(0.2, "Video file type");
            }
            "jpg" | "png" | "gif" | "bmp" => {
                tier_score.add_warm_weight(0.1, "Image file type");
            }
            "txt" | "log" | "json" | "xml" => {
                tier_score.add_hot_weight(0.2, "Text/config file type");
            }
            "zip" | "tar" | "gz" | "bz2" => {
                tier_score.add_cold_weight(0.3, "Archive file type");
            }
            _ => {}
        }
    }

    // 6. Performance policy evaluation
    for (policy_id, policy) in policies {
        if policy.enabled && dataset_matches_policy_pattern(dataset_name, policy) {
            debug!("Dataset {} matches policy {}", dataset_name, policy_id);

            for tier_rule in &policy.conditions.tier_rules {
                match tier_rule.target_tier {
                    StorageTier::Hot => {
                        tier_score.add_hot_weight(0.2, "fixed");
                    }
                    StorageTier::Warm => {
                        tier_score.add_warm_weight(0.2, "fixed");
                    }
                    StorageTier::Cold => {
                        tier_score.add_cold_weight(0.2, "fixed");
                    }
                    StorageTier::Cache => {
                        tier_score.add_hot_weight(0.3, "fixed");
                    }
                    StorageTier::Archive => {
                        tier_score.add_cold_weight(0.1, "fixed");
                    }
                }
            }
        }
    }

    // 7. Get final recommendation
    let recommended_tier = tier_score.get_recommendation();

    info!(
        "Tier evaluation for {}: {} (scoring: hot={:.2}, warm={:.2}, cold={:.2})",
        dataset_name,
        match recommended_tier {
            StorageTier::Hot => "Hot",
            StorageTier::Warm => "Warm",
            StorageTier::Cold => "Cold",
            StorageTier::Cache => "Cache",
            StorageTier::Archive => "Archive",
        },
        tier_score.hot_score,
        tier_score.warm_score,
        tier_score.cold_score
    );

    Ok(recommended_tier)
}

/// Check if dataset matches policy patterns
fn dataset_matches_policy_pattern(dataset_name: &str, policy: &AutomationPolicy) -> bool {
    for tier_rule in &policy.conditions.tier_rules {
        if tier_rule.condition == "*" {
            return true;
        }
        if dataset_name.contains(&tier_rule.condition) {
            return true;
        }
        // Add regex pattern matching
        if tier_rule.condition.starts_with("regex:") {
            let pattern = &tier_rule.condition[6..];
            if let Ok(regex) = regex::Regex::new(pattern)
                && regex.is_match(dataset_name)
            {
                return true;
            }
        }
    }
    false
}
