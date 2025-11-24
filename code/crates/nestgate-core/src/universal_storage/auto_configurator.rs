use crate::error::Result;
use crate::unified_enums::storage_types::UnifiedStorageCapability;
use std::collections::HashMap;
//
// Analyzes detected storage systems and automatically creates optimal configurations
// for different use cases. Handles:
// - Performance optimization (hot/warm/cold tiers)
// - Redundancy and reliability (RAID, mirroring, replication)
// - Cost optimization for cloud storage
// - Hybrid architectures (local + cloud, multi-cloud)
// - ZFS-like feature mapping across different backends

use crate::universal_storage::DetectedStorage;
// Removed unused imports - using unified types
use serde::{Deserialize, Serialize};

#[cfg(test)]
#[path = "auto_configurator_tests.rs"]
mod auto_configurator_tests;

// ==================== SECTION ====================

/// **INTELLIGENT STORAGE AUTO-CONFIGURATOR**
/// Creates optimal storage configurations from detected storage systems
pub struct AutoConfigurator {
    /// Configuration preferences and constraints
    config: ConfiguratorSettings,
    /// Detected storage systems to work with
    available_storage: Vec<DetectedStorage>,
}
impl AutoConfigurator {
    /// Create new auto-configurator with detected storage
    #[must_use]
    pub fn new(available_storage: Vec<DetectedStorage>) -> Self {
        Self {
            config: ConfiguratorSettings::default(),
            available_storage,
        }
    }

    /// Create configurator with custom settings
    #[must_use]
    pub fn with_settings(
        available_storage: Vec<DetectedStorage>,
        config: ConfiguratorSettings,
    ) -> Self {
        Self {
            config,
            available_storage,
        }
    }

    /// Get configuration settings
    #[must_use]
    pub fn config(&self) -> &ConfiguratorSettings {
        &self.config
    }

    /// Update configuration settings
    pub fn update_config(&mut self, config: ConfiguratorSettings) {
        self.config = config;
    }

    /// Check if auto-tuning is enabled
    #[must_use]
    pub fn is_auto_tuning_enabled(&self) -> bool {
        self.config.enable_auto_tuning
    }

    /// **MAIN CONFIGURATION METHOD**
    /// Analyzes requirements and creates optimal storage configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    ///
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    #[allow(deprecated)]
    pub async fn create_optimal_config(
        &self,
        requirements: StorageRequirements,
    ) -> Result<OptimalStorageConfig> {
        // 1. Analyze available storage capabilities
        let storage_analysis = self.analyze_storage_landscape().await?;

        // 2. Match requirements to available storage
        let storage_mapping = self
            .map_requirements_to_storage(&requirements, &storage_analysis)
            .await?;

        // 3. Create tiered storage architecture
        let tier_config = self
            .create_storage_tiers(&storage_mapping, &requirements)
            .await?;

        // 4. Configure redundancy and reliability
        let redundancy_config = self
            .configure_redundancy(&tier_config, &requirements)
            .await?;

        // 5. Optimize for cost and performance
        let optimized_config = self
            .optimize_configuration(redundancy_config, &requirements)
            .await?;

        // 6. Generate implementation plan
        let implementation_plan = self.generate_implementation_plan(&optimized_config).await?;

        Ok(OptimalStorageConfig {
            tier_configuration: optimized_config.tier_configuration,
            redundancy_strategy: optimized_config.redundancy_strategy,
            performance_profile: optimized_config.performance_profile,
            cost_estimation: optimized_config.cost_estimation,
            zfs_feature_mapping: optimized_config.zfs_feature_mapping,
            implementation_plan,
            confidence_score: optimized_config.confidence_score,
        })
    }

    /// Analyze the landscape of available storage
    async fn analyze_storage_landscape(&self) -> Result<StorageLandscapeAnalysis> {
        let mut analysis = StorageLandscapeAnalysis::default();

        // Categorize storage by performance tiers
        for storage in &self.available_storage {
            let tier = self.classify_performance_tier(storage);
            analysis
                .performance_tiers
                .entry(tier)
                .or_insert_with(Vec::new)
                .push(storage.clone());
        }

        // Analyze redundancy options
        analysis.redundancy_options = self.analyze_redundancy_options().await?;

        // Calculate total capacity and cost
        analysis.total_capacity = self
            .available_storage
            .iter()
            .map(|s| s.available_space)
            .sum();
        analysis.total_monthly_cost = self
            .available_storage
            .iter()
            .map(|s| {
                s.cost_profile.storage_cost_per_gb_month
                    * (s.available_space as f64 / 1_000_000_000.0)
            })
            .sum();

        // Identify unique capabilities
        let mut all_capabilities = std::collections::HashSet::new();
        for storage in &self.available_storage {
            for cap in &storage.capabilities {
                all_capabilities.insert(cap.clone());
            }
        }
        analysis.available_capabilities = all_capabilities.into_iter().collect();

        Ok(analysis)
    }

    /// Map storage requirements to available storage systems
    async fn map_requirements_to_storage(
        &self,
        requirements: &StorageRequirements,
        analysis: &StorageLandscapeAnalysis,
    ) -> Result<StorageMapping> {
        let mut mapping = StorageMapping::default();

        // Map performance requirements
        if let Some(min_throughput) = requirements.min_throughput_mbps {
            mapping.performance_storage = analysis
                .performance_tiers
                .get(&PerformanceTier::High)
                .unwrap_or(&vec![])
                .iter()
                .filter(|s| s.performance_profile.read_throughput_mbps >= min_throughput)
                .cloned()
                .collect();
        }

        // Map capacity requirements
        if let Some(min_capacity) = requirements.min_capacity_gb {
            let min_bytes = min_capacity * 1_000_000_000;
            mapping.capacity_storage = self
                .available_storage
                .iter()
                .filter(|s| s.available_space >= min_bytes)
                .cloned()
                .collect();
        }

        // Map reliability requirements
        if let Some(min_reliability) = requirements.min_reliability_score {
            mapping.reliable_storage = self
                .available_storage
                .iter()
                .filter(|s| s.reliability_score >= min_reliability)
                .cloned()
                .collect();
        }

        // Map cost requirements
        if let Some(max_cost) = requirements.max_monthly_cost_usd {
            mapping.cost_effective_storage = self
                .available_storage
                .iter()
                .filter(|s| {
                    let monthly_cost = s.cost_profile.storage_cost_per_gb_month
                        * (s.available_space as f64 / 1_000_000_000.0);
                    monthly_cost <= max_cost
                })
                .cloned()
                .collect();
        }

        // Map ZFS feature requirements
        mapping.zfs_capable_storage =
            self.find_zfs_capable_storage(&requirements.required_zfs_features);

        Ok(mapping)
    }

    /// Create storage tier configuration
    async fn create_storage_tiers(
        &self,
        mapping: &StorageMapping,
        requirements: &StorageRequirements,
    ) -> Result<TierConfiguration> {
        // Configure tiered storage with explicit initialization
        let tier_config = TierConfiguration {
            hot_tier: self.select_hot_tier_storage(mapping, requirements).await?,
            warm_tier: self.select_warm_tier_storage(mapping, requirements).await?,
            cold_tier: self.select_cold_tier_storage(mapping, requirements).await?,
            tiering_rules: self.create_tiering_rules(requirements),
        };

        Ok(tier_config)
    }

    /// Configure redundancy strategy
    async fn configure_redundancy(
        &self,
        tier_config: &TierConfiguration,
        requirements: &StorageRequirements,
    ) -> Result<RedundancyConfiguration> {
        // Determine optimal redundancy strategy based on requirements
        let strategy = match requirements.redundancy_level {
            Some(RedundancyLevel::None) => RedundancyStrategy::None,
            Some(RedundancyLevel::Mirror) => self.configure_mirroring(tier_config).await?,
            Some(RedundancyLevel::RaidZ1) => self.configure_raid_z1(tier_config).await?,
            Some(RedundancyLevel::RaidZ2) => self.configure_raid_z2(tier_config).await?,
            Some(RedundancyLevel::RaidZ3) => self.configure_raid_z3(tier_config).await?,
            None => {
                self.auto_select_redundancy(tier_config, requirements)
                    .await?
            }
        };

        let mut redundancy_config = RedundancyConfiguration {
            strategy,
            ..Default::default()
        };

        // Configure cross-tier redundancy if needed
        if requirements.cross_tier_redundancy.unwrap_or(false) {
            redundancy_config.cross_tier_strategy =
                Some(self.configure_cross_tier_redundancy(tier_config).await?);
        }

        Ok(redundancy_config)
    }

    /// Optimize final configuration for performance and cost
    async fn optimize_configuration(
        &self,
        config: RedundancyConfiguration,
        requirements: &StorageRequirements,
    ) -> Result<OptimizedConfiguration> {
        // Create optimized configuration with explicit field initialization
        let tier_config = self.extract_tier_configuration(&config).await?;
        let optimized = OptimizedConfiguration {
            tier_configuration: tier_config,
            redundancy_strategy: config.strategy.clone(),
            performance_profile: self.calculate_expected_performance(&config).await?,
            cost_estimation: self.calculate_cost_estimation(&config).await?,
            zfs_feature_mapping: self.map_zfs_features(&config, requirements).await?,
            confidence_score: self
                .calculate_confidence_score(&config, requirements)
                .await?,
        };

        Ok(optimized)
    }

    /// Extract tier configuration from redundancy configuration
    async fn extract_tier_configuration(
        &self,
        _config: &RedundancyConfiguration,
    ) -> Result<TierConfiguration> {
        // Create a basic tier configuration with default storage configurations
        // This would typically be populated from actual storage detection
        Ok(TierConfiguration {
            hot_tier: vec![],      // Default empty - would be populated by storage detection
            warm_tier: vec![],     // Default empty - would be populated by storage detection
            cold_tier: vec![],     // Default empty - would be populated by storage detection
            tiering_rules: vec![], // Default empty rules
        })
    }

    /// Generate step-by-step implementation plan
    async fn generate_implementation_plan(
        &self,
        config: &OptimizedConfiguration,
    ) -> Result<ImplementationPlan> {
        let mut plan = ImplementationPlan::default();

        // Phase 1: Setup base storage
        plan.phases.push(ImplementationPhase {
            phase_number: 1,
            name: "Base Storage Setup".to_string(),
            description: "Initialize and configure base storage backends".to_string(),
            steps: self.generate_base_setup_steps(config).await?,
            estimated_duration_minutes: 30,
            dependencies: vec![],
        });

        // Phase 2: Configure redundancy
        if !matches!(config.redundancy_strategy, RedundancyStrategy::None) {
            plan.phases.push(ImplementationPhase {
                phase_number: 2,
                name: "Redundancy Configuration".to_string(),
                description: "Set up RAID-Z, mirroring, or other redundancy".to_string(),
                steps: self.generate_redundancy_setup_steps(config).await?,
                estimated_duration_minutes: 45,
                dependencies: vec![1],
            });
        }

        // Phase 3: Enable ZFS features
        plan.phases.push(ImplementationPhase {
            phase_number: 3,
            name: "ZFS Features Activation".to_string(),
            description: "Enable compression, deduplication, snapshots, etc.".to_string(),
            steps: self.generate_zfs_features_steps(config).await?,
            estimated_duration_minutes: 20,
            dependencies: vec![2],
        });

        // Phase 4: Performance tuning
        plan.phases.push(ImplementationPhase {
            phase_number: 4,
            name: "Performance Optimization".to_string(),
            description: "Tune configuration for optimal performance".to_string(),
            steps: self.generate_performance_tuning_steps(config).await?,
            estimated_duration_minutes: 15,
            dependencies: vec![3],
        });

        plan.total_estimated_duration_minutes = plan
            .phases
            .iter()
            .map(|p| p.estimated_duration_minutes)
            .sum();

        Ok(plan)
    }

    // ==================== HELPER METHODS ====================

    fn classify_performance_tier(&self, storage: &DetectedStorage) -> PerformanceTier {
        let throughput = storage.performance_profile.read_throughput_mbps;
        let latency = storage.performance_profile.read_latency_us;
        let iops = storage.performance_profile.iops;

        // High performance: NVMe, high-end SSDs, memory
        if throughput > 1000.0 && latency < 100.0 && iops > 100_000 {
            PerformanceTier::High
        }
        // Medium performance: SATA SSDs, fast HDDs
        else if throughput > 100.0 && latency < 10_000.0 && iops > 1000 {
            PerformanceTier::Medium
        }
        // Low performance: HDDs, network storage, cloud storage
        else {
            PerformanceTier::Low
        }
    }

    fn find_zfs_capable_storage(&self, required_features: &[ZfsFeature]) -> Vec<DetectedStorage> {
        self.available_storage
            .iter()
            .filter(|storage| {
                // Check if storage can support the required ZFS features
                // Either natively or through software implementation
                required_features
                    .iter()
                    .all(|feature| self.can_support_zfs_feature(storage, feature))
            })
            .cloned()
            .collect()
    }

    fn can_support_zfs_feature(&self, storage: &DetectedStorage, feature: &ZfsFeature) -> bool {
        match feature {
            ZfsFeature::Compression => {
                // Any storage can support software compression
                true
            }
            ZfsFeature::Deduplication => {
                // Need sufficient space for hash index
                storage.available_space > 10_000_000_000 // > 10GB
            }
            ZfsFeature::Snapshots => {
                // Any storage with COW capability or sufficient space
                storage
                    .capabilities
                    .contains(&UnifiedStorageCapability::Snapshots)
                    || storage.available_space > 1_000_000_000 // > 1GB
            }
            ZfsFeature::Checksumming => {
                // Any storage can support checksumming
                true
            }
            ZfsFeature::Encryption => {
                // Any storage can support software encryption
                true
            }
            ZfsFeature::RaidZ => {
                // Need multiple storage backends or block-level access
                storage
                    .capabilities
                    .contains(&UnifiedStorageCapability::Replication)
                    || self.available_storage.len() > 2
            }
        }
    }

    // Placeholder implementations for complex operations
    async fn analyze_redundancy_options(&self) -> Result<Vec<RedundancyOption>> {
        Ok(vec![])
    }
    async fn select_hot_tier_storage(
        &self,
        _mapping: &StorageMapping,
        _requirements: &StorageRequirements,
    ) -> Result<Vec<DetectedStorage>> {
        Ok(vec![])
    }
    async fn select_warm_tier_storage(
        &self,
        _mapping: &StorageMapping,
        _requirements: &StorageRequirements,
    ) -> Result<Vec<DetectedStorage>> {
        Ok(vec![])
    }
    async fn select_cold_tier_storage(
        &self,
        _mapping: &StorageMapping,
        _requirements: &StorageRequirements,
    ) -> Result<Vec<DetectedStorage>> {
        Ok(vec![])
    }
    fn create_tiering_rules(&self, _requirements: &StorageRequirements) -> Vec<TieringRule> {
        vec![]
    }
    async fn configure_mirroring(
        &self,
        _tier_config: &TierConfiguration,
    ) -> Result<RedundancyStrategy> {
        Ok(RedundancyStrategy::Mirror)
    }
    async fn configure_raid_z1(
        &self,
        _tier_config: &TierConfiguration,
    ) -> Result<RedundancyStrategy> {
        Ok(RedundancyStrategy::RaidZ1)
    }
    async fn configure_raid_z2(
        &self,
        _tier_config: &TierConfiguration,
    ) -> Result<RedundancyStrategy> {
        Ok(RedundancyStrategy::RaidZ2)
    }
    async fn configure_raid_z3(
        &self,
        _tier_config: &TierConfiguration,
    ) -> Result<RedundancyStrategy> {
        Ok(RedundancyStrategy::RaidZ3)
    }
    async fn auto_select_redundancy(
        &self,
        _tier_config: &TierConfiguration,
        _requirements: &StorageRequirements,
    ) -> Result<RedundancyStrategy> {
        Ok(RedundancyStrategy::Mirror)
    }
    async fn configure_cross_tier_redundancy(
        &self,
        _tier_config: &TierConfiguration,
    ) -> Result<CrossTierRedundancyStrategy> {
        Ok(CrossTierRedundancyStrategy::None)
    }
    async fn calculate_expected_performance(
        &self,
        _config: &RedundancyConfiguration,
    ) -> Result<ExpectedPerformanceProfile> {
        Ok(ExpectedPerformanceProfile::default())
    }
    async fn calculate_cost_estimation(
        &self,
        _config: &RedundancyConfiguration,
    ) -> Result<CostEstimation> {
        Ok(CostEstimation::default())
    }
    async fn map_zfs_features(
        &self,
        _config: &RedundancyConfiguration,
        _requirements: &StorageRequirements,
    ) -> Result<ZfsFeatureMapping> {
        Ok(ZfsFeatureMapping::default())
    }
    async fn calculate_confidence_score(
        &self,
        _config: &RedundancyConfiguration,
        _requirements: &StorageRequirements,
    ) -> Result<f64> {
        Ok(0.85)
    }
    async fn generate_base_setup_steps(
        &self,
        _config: &OptimizedConfiguration,
    ) -> Result<Vec<ImplementationStep>> {
        Ok(vec![])
    }
    async fn generate_redundancy_setup_steps(
        &self,
        _config: &OptimizedConfiguration,
    ) -> Result<Vec<ImplementationStep>> {
        Ok(vec![])
    }
    async fn generate_zfs_features_steps(
        &self,
        _config: &OptimizedConfiguration,
    ) -> Result<Vec<ImplementationStep>> {
        Ok(vec![])
    }
    async fn generate_performance_tuning_steps(
        &self,
        _config: &OptimizedConfiguration,
    ) -> Result<Vec<ImplementationStep>> {
        Ok(vec![])
    }
}

// ==================== SECTION ====================

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
/// ZFS features that can be required
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZfsFeature {
    Compression,
    Deduplication,
    Snapshots,
    Checksumming,
    Encryption,
    RaidZ,
}
/// Redundancy level options
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RedundancyLevel {
    None,
    Mirror,
    RaidZ1,
    RaidZ2,
    RaidZ3,
}
/// Storage use case categories
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageUseCase {
    HomeNas,
    SmallBusiness,
    Enterprise,
    CloudNative,
    HighPerformance,
    Archive,
    Development,
}
/// Final optimal storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::OptimalStorageConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::OptimalStorageConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct OptimalStorageConfig {
    pub tier_configuration: TierConfiguration,
    pub redundancy_strategy: RedundancyStrategy,
    pub performance_profile: ExpectedPerformanceProfile,
    pub cost_estimation: CostEstimation,
    pub zfs_feature_mapping: ZfsFeatureMapping,
    pub implementation_plan: ImplementationPlan,
    pub confidence_score: f64,
}
// Supporting data structures (simplified for brevity)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StorageLandscapeAnalysis {
    pub performance_tiers: HashMap<PerformanceTier, Vec<DetectedStorage>>,
    pub redundancy_options: Vec<RedundancyOption>,
    pub total_capacity: u64,
    pub total_monthly_cost: f64,
    pub available_capabilities: Vec<UnifiedStorageCapability>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StorageMapping {
    pub performance_storage: Vec<DetectedStorage>,
    pub capacity_storage: Vec<DetectedStorage>,
    pub reliable_storage: Vec<DetectedStorage>,
    pub cost_effective_storage: Vec<DetectedStorage>,
    pub zfs_capable_storage: Vec<DetectedStorage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TierConfiguration {
    pub hot_tier: Vec<DetectedStorage>,
    pub warm_tier: Vec<DetectedStorage>,
    pub cold_tier: Vec<DetectedStorage>,
    pub tiering_rules: Vec<TieringRule>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PerformanceTier {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RedundancyStrategy {
    None,
    Mirror,
    RaidZ1,
    RaidZ2,
    RaidZ3,
    Custom(String),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RedundancyConfiguration {
    pub strategy: RedundancyStrategy,
    pub cross_tier_strategy: Option<CrossTierRedundancyStrategy>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OptimizedConfiguration {
    pub tier_configuration: TierConfiguration,
    pub redundancy_strategy: RedundancyStrategy,
    pub performance_profile: ExpectedPerformanceProfile,
    pub cost_estimation: CostEstimation,
    pub zfs_feature_mapping: ZfsFeatureMapping,
    pub confidence_score: f64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ImplementationPlan {
    pub phases: Vec<ImplementationPhase>,
    pub total_estimated_duration_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationPhase {
    pub phase_number: u32,
    pub name: String,
    pub description: String,
    pub steps: Vec<ImplementationStep>,
    pub estimated_duration_minutes: u32,
    pub dependencies: Vec<u32>,
}

// Additional supporting types (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedundancyOption {
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TieringRule {
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrossTierRedundancyStrategy {
    None,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExpectedPerformanceProfile {
    pub throughput_mbps: f64,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CostEstimation {
    pub monthly_cost_usd: f64,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ZfsFeatureMapping {
    pub enabled_features: Vec<ZfsFeature>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationStep {
    pub description: String,
}

impl Default for RedundancyStrategy {
    fn default() -> Self {
        Self::None
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type OptimalStorageConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using OptimalStorageConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.
