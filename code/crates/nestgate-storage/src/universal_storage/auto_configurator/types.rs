// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Type definitions for the auto-configurator
//!
//! This module contains all data structures used for storage configuration,
//! including requirements, configurations, analysis results, and implementation plans.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::enums::{
    CrossTierRedundancyStrategy, PerformanceTier, RedundancyLevel, RedundancyStrategy,
    StorageUseCase, ZfsFeature,
};
use crate::universal_storage::DetectedStorage;
use nestgate_types::unified_enums::storage_types::UnifiedStorageCapability;

// ==================== CONFIGURATION SETTINGS ====================

/// Configuration settings for the auto-configurator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfiguratorSettings {
    /// Prefer performance over cost
    pub prioritize_performance: bool,
    /// Prefer cost savings over performance
    pub prioritize_cost: bool,
    /// Enable aggressive optimization
    pub aggressive_optimization: bool,
    /// Maximum acceptable risk level
    pub max_risk_level: f64,
    /// Enable auto-tuning
    pub enable_auto_tuning: bool,
}

impl Default for ConfiguratorSettings {
    fn default() -> Self {
        Self {
            prioritize_performance: false,
            prioritize_cost: false,
            aggressive_optimization: false,
            max_risk_level: 0.1, // Conservative by default
            enable_auto_tuning: true,
        }
    }
}

// ==================== REQUIREMENTS ====================

/// Storage requirements specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageRequirements {
    /// Minimum throughput in MB/s
    pub min_throughput_mbps: Option<f64>,
    /// Minimum capacity in GB
    pub min_capacity_gb: Option<u64>,
    /// Minimum reliability score (0.0 - 1.0)
    pub min_reliability_score: Option<f64>,
    /// Maximum monthly cost in USD
    pub max_monthly_cost_usd: Option<f64>,
    /// Required ZFS features
    pub required_zfs_features: Vec<ZfsFeature>,
    /// Redundancy level requirement
    pub redundancy_level: Option<RedundancyLevel>,
    /// Whether to enable cross-tier redundancy
    pub cross_tier_redundancy: Option<bool>,
    /// Use case description
    pub use_case: StorageUseCase,
}

// ==================== CONFIGURATIONS ====================

/// Final optimal storage configuration
///
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use nestgate_config::config::OptimalStorageConfig;
///
/// // NEW (canonical):
/// use nestgate_config::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_config::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct OptimalStorageConfig {
    /// Tier configuration
    pub tier_configuration: TierConfiguration,
    /// Redundancy strategy
    pub redundancy_strategy: RedundancyStrategy,
    /// Performance profile
    pub performance_profile: ExpectedPerformanceProfile,
    /// Cost estimation
    pub cost_estimation: CostEstimation,
    /// ZFS feature mapping
    pub zfs_feature_mapping: ZfsFeatureMapping,
    /// Implementation plan
    pub implementation_plan: ImplementationPlan,
    /// Confidence score
    pub confidence_score: f64,
}

/// Tier configuration for hot/warm/cold tiers
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TierConfiguration {
    /// Hot tier storage
    pub hot_tier: Vec<DetectedStorage>,
    /// Warm tier storage
    pub warm_tier: Vec<DetectedStorage>,
    /// Cold tier storage
    pub cold_tier: Vec<DetectedStorage>,
    /// Tiering rules
    pub tiering_rules: Vec<TieringRule>,
}

/// Redundancy configuration
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RedundancyConfiguration {
    /// Redundancy strategy
    pub strategy: RedundancyStrategy,
    /// Cross-tier strategy
    pub cross_tier_strategy: Option<CrossTierRedundancyStrategy>,
}

/// Optimized configuration (complete)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OptimizedConfiguration {
    /// Tier configuration
    pub tier_configuration: TierConfiguration,
    /// Redundancy strategy
    pub redundancy_strategy: RedundancyStrategy,
    /// Performance profile
    pub performance_profile: ExpectedPerformanceProfile,
    /// Cost estimation
    pub cost_estimation: CostEstimation,
    /// ZFS feature mapping
    pub zfs_feature_mapping: ZfsFeatureMapping,
    /// Confidence score
    pub confidence_score: f64,
}

// ==================== ANALYSIS RESULTS ====================

/// Storage landscape analysis results
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StorageLandscapeAnalysis {
    /// Performance tiers mapping
    pub performance_tiers: HashMap<PerformanceTier, Vec<DetectedStorage>>,
    /// Available redundancy options
    pub redundancy_options: Vec<RedundancyOption>,
    /// Total capacity across all storage
    pub total_capacity: u64,
    /// Total monthly cost
    pub total_monthly_cost: f64,
    /// Available storage capabilities
    pub available_capabilities: Vec<UnifiedStorageCapability>,
}

/// Storage mapping for requirements
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StorageMapping {
    /// High-performance storage
    pub performance_storage: Vec<DetectedStorage>,
    /// High-capacity storage
    pub capacity_storage: Vec<DetectedStorage>,
    /// High-reliability storage
    pub reliable_storage: Vec<DetectedStorage>,
    /// Cost-effective storage
    pub cost_effective_storage: Vec<DetectedStorage>,
    /// ZFS-capable storage
    pub zfs_capable_storage: Vec<DetectedStorage>,
}

// ==================== IMPLEMENTATION ====================

/// Implementation plan with phases
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ImplementationPlan {
    /// Implementation phases
    pub phases: Vec<ImplementationPhase>,
    /// Total estimated duration in minutes
    pub total_estimated_duration_minutes: u32,
}

/// Single implementation phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationPhase {
    /// Phase number
    pub phase_number: u32,
    /// Phase name
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Implementation steps
    pub steps: Vec<ImplementationStep>,
    /// Estimated duration in minutes
    pub estimated_duration_minutes: u32,
    /// Dependencies (phase numbers)
    pub dependencies: Vec<u32>,
}

/// Single implementation step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationStep {
    /// Human-readable description
    pub description: String,
}

// ==================== SUPPORTING TYPES ====================

/// Redundancy option (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedundancyOption {
    /// Option name
    pub name: String,
}

/// Tiering rule (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TieringRule {
    /// Rule name
    pub name: String,
}

/// Expected performance profile
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExpectedPerformanceProfile {
    /// Expected throughput in MB/s
    pub throughput_mbps: f64,
}

/// Cost estimation
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CostEstimation {
    /// Monthly cost in USD
    pub monthly_cost_usd: f64,
}

/// ZFS feature mapping
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ZfsFeatureMapping {
    /// Enabled ZFS features
    pub enabled_features: Vec<ZfsFeature>,
}
