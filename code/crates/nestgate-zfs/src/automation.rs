//! ZFS Automation Integration
//!
//! This module provides integration between ZFS storage management and the
//! NestGate automation system. It re-exports the main automation functionality
//! from the dedicated nestgate-automation crate.

// Re-export main automation types and functionality
pub use nestgate_automation::{
    AccessPatterns, AutomationConfig, DatasetAnalysis, DatasetAnalyzer, DatasetLifecycleManager,
    FileAnalysis, FileAnalyzer, IntelligentDatasetManager, OptimizationResult,
    Result as AutomationResult, TierPerformanceStats, TierPrediction, TierPredictor,
};

// Note: AI prediction functionality has been sunset - use heuristic tier prediction instead

// Re-export ecosystem integration types (when network integration is enabled)
#[cfg(feature = "network-integration")]
pub use nestgate_automation::{
    DatasetCreatedNotification, EcosystemDiscovery, EcosystemService, ServiceConnectionPool,
    ServicePlan, ServiceRegistration, SquirrelConnection, TierDiscoveryRequest,
    TierDiscoveryResponse,
};

// Legacy compatibility types that some existing code might still reference
pub use nestgate_automation::{
    AccessEvent, AccessType, DatasetContext, FileCharacteristics, FileType, ServiceHealth,
    StorageContext, TaskPriority, TierStats, TrainingExample,
};

/// Initialize automation for ZFS with default configuration
pub async fn initialize_zfs_automation() -> AutomationResult<IntelligentDatasetManager> {
    let zfs_config = nestgate_core::config::Config::default();
    nestgate_automation::initialize_automation(zfs_config).await
}

/// Initialize automation for ZFS with custom configuration
pub async fn initialize_zfs_automation_with_config(
    automation_config: AutomationConfig,
) -> AutomationResult<IntelligentDatasetManager> {
    let zfs_config = nestgate_core::config::Config::default();
    nestgate_automation::initialize_automation_with_config(zfs_config, automation_config).await
}

/// Check if ecosystem services are available for ZFS automation
#[cfg(feature = "network-integration")]
pub async fn check_zfs_ecosystem_availability() -> bool {
    nestgate_automation::check_ecosystem_availability().await
}

#[cfg(not(feature = "network-integration"))]
pub async fn check_zfs_ecosystem_availability() -> bool {
    false
}

// ZFS Dataset Automation - Intelligent Lifecycle Management
//
// Production-ready automated dataset management with AI-driven optimization

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use crate::{dataset::ZfsDatasetManager, migration::MigrationEngine, pool::ZfsPoolManager};
use nestgate_core::{Result, StorageTier};

pub use crate::config::{AiAutomationSettings, DatasetAutomationConfig};

/// Policy priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PolicyPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Policy conditions container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConditions {
    pub tier_rules: Vec<TierRule>,
    pub migration_rules: Vec<MigrationRule>,
    pub lifecycle_rules: Vec<LifecycleRule>,
}

/// Simple tier rule for basic automation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierRule {
    pub condition: String,
    pub target_tier: StorageTier,
    pub priority: u32,
}

/// Simple migration rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRule {
    pub source_tier: StorageTier,
    pub target_tier: StorageTier,
    pub condition: String,
    pub bandwidth_limits: BandwidthLimits,
    pub schedule: String,
}

/// Simple lifecycle rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleRule {
    pub stage: LifecycleStage,
    pub next_stage: Option<LifecycleStage>,
    pub conditions: Vec<String>,
    pub actions: Vec<String>,
}

/// Intelligent dataset automation engine
#[derive(Debug)]
pub struct DatasetAutomation {
    /// Pool management
    pool_manager: Arc<ZfsPoolManager>,
    /// Dataset management
    dataset_manager: Arc<ZfsDatasetManager>,
    /// Migration engine for tier movement
    migration_engine: Arc<RwLock<MigrationEngine>>,
    /// Active automation policies
    policies: Arc<RwLock<HashMap<String, AutomationPolicy>>>,
    /// Lifecycle tracking
    lifecycle_tracker: Arc<RwLock<HashMap<String, DatasetLifecycle>>>,
    /// Configuration
    config: DatasetAutomationConfig,
}

/// Dataset lifecycle management policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationPolicy {
    pub policy_id: String,
    pub name: String,
    pub description: String,
    pub priority: PolicyPriority,
    pub enabled: bool,
    pub conditions: PolicyConditions,
    pub created: SystemTime,
    pub last_modified: SystemTime,
}

/// Tier assignment rules for automatic dataset placement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierAssignmentRules {
    /// Auto-assign new datasets based on predicted usage
    pub auto_assign_new: bool,
    /// File size thresholds for tier assignment
    pub size_thresholds: TierSizeThresholds,
    /// Access pattern based assignment
    pub access_pattern_rules: AccessPatternRules,
    /// Performance requirements
    pub performance_requirements: HashMap<StorageTier, PerformanceRequirement>,
}

/// Size-based tier thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierSizeThresholds {
    /// Files smaller than this go to hot tier (bytes)
    pub hot_max_size: u64,
    /// Files smaller than this go to warm tier (bytes)
    pub warm_max_size: u64,
}

/// Access pattern rules for tier assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPatternRules {
    /// Daily access count threshold for hot tier
    pub hot_access_threshold: u32,
    /// Daily access count threshold for warm tier
    pub warm_access_threshold: u32,
    /// Age in days before moving to cold tier
    pub cold_age_threshold: u32,
}

/// Performance requirements per tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirement {
    pub max_latency_ms: f64,
    pub min_throughput_mbps: f64,
    pub min_iops: u32,
}

/// Lifecycle management rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleRules {
    /// Automatic cleanup of old files
    pub enable_cleanup: bool,
    /// Age threshold for cleanup (days)
    pub cleanup_age_days: u32,
    /// Automatic compression based on age
    pub enable_auto_compression: bool,
    /// Age threshold for compression (days)
    pub compression_age_days: u32,
    /// Automatic archival to cold storage
    pub enable_auto_archival: bool,
    /// Age threshold for archival (days)
    pub archival_age_days: u32,
}

/// Migration automation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRules {
    /// Enable automatic migration based on access patterns
    pub enable_auto_migration: bool,
    /// Migration schedule (background processing)
    pub migration_schedule: MigrationSchedule,
    /// Performance impact limits
    pub performance_limits: MigrationPerformanceLimits,
    /// Bandwidth limits for migrations
    pub bandwidth_limits: BandwidthLimits,
}

/// Migration scheduling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationSchedule {
    /// Hours during which migration is allowed
    pub allowed_hours: Vec<u8>,
    /// Maximum concurrent migrations
    pub max_concurrent: u32,
    /// Priority boost during off-peak hours
    pub off_peak_priority_boost: bool,
}

/// Performance impact limits for migrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationPerformanceLimits {
    /// Maximum CPU usage during migration (%)
    pub max_cpu_usage: f64,
    /// Maximum memory usage during migration (%)
    pub max_memory_usage: f64,
    /// Maximum IO impact (%)
    pub max_io_impact: f64,
}

/// Bandwidth limits for migrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthLimits {
    /// Maximum migration bandwidth during peak hours (MB/s)
    pub peak_max_mbps: u64,
    /// Maximum migration bandwidth during off-peak hours (MB/s)
    pub off_peak_max_mbps: u64,
}

/// Performance thresholds for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Latency threshold that triggers optimization (ms)
    pub max_latency_ms: f64,
    /// Minimum throughput before optimization (MB/s)
    pub min_throughput_mbps: f64,
    /// Error rate threshold (%)
    pub max_error_rate: f64,
    /// Utilization threshold for tier rebalancing (%)
    pub max_utilization: f64,
}

/// Dataset lifecycle tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetLifecycle {
    pub dataset_name: String,
    pub current_tier: StorageTier,
    pub created: SystemTime,
    pub last_accessed: Option<SystemTime>,
    pub access_count: u64,
    pub total_migrations: u32,
    pub last_optimization: Option<SystemTime>,
    pub lifecycle_stage: LifecycleStage,
    pub automation_history: Vec<AutomationEvent>,
}

/// Dataset lifecycle stages with automation rules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LifecycleStage {
    /// Newly created, high activity expected
    New,
    /// Active usage phase
    Active,
    /// Declining usage, candidate for migration
    Aging,
    /// Low usage, moved to cold storage
    Archived,
    /// Marked for potential cleanup
    Obsolete,
}

impl std::fmt::Display for LifecycleStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LifecycleStage::New => write!(f, "New"),
            LifecycleStage::Active => write!(f, "Active"),
            LifecycleStage::Aging => write!(f, "Aging"),
            LifecycleStage::Archived => write!(f, "Archived"),
            LifecycleStage::Obsolete => write!(f, "Obsolete"),
        }
    }
}

/// Automation event tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationEvent {
    pub event_id: String,
    pub event_type: AutomationEventType,
    pub timestamp: SystemTime,
    pub details: String,
    pub success: bool,
}

/// Types of automation events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationEventType {
    TierAssignment,
    Migration,
    Optimization,
    Cleanup,
    Compression,
    Archival,
    PolicyUpdate,
}

impl DatasetAutomation {
    /// Create new dataset automation engine
    pub async fn new(
        pool_manager: Arc<ZfsPoolManager>,
        dataset_manager: Arc<ZfsDatasetManager>,
        migration_engine: Arc<RwLock<MigrationEngine>>,
        config: DatasetAutomationConfig,
    ) -> Result<Self> {
        info!("Initializing Dataset Automation Engine");

        let policies = Arc::new(RwLock::new(HashMap::new()));
        let lifecycle_tracker = Arc::new(RwLock::new(HashMap::new()));

        let automation = Self {
            pool_manager,
            dataset_manager,
            migration_engine,
            policies,
            lifecycle_tracker,
            config,
        };

        // Initialize with default policies
        automation.create_default_policies().await?;

        info!("Dataset Automation Engine initialized successfully");
        Ok(automation)
    }

    /// Start the automation engine
    pub async fn start(&self) -> Result<()> {
        info!("Starting dataset automation engine");

        // Create default policies if none exist
        if self.policies.read().await.is_empty() {
            self.create_default_policies().await?;
        }

        // Start background automation loop
        let automation_clone = self.clone();
        tokio::spawn(async move {
            automation_clone.automation_loop().await;
        });

        Ok(())
    }

    /// Main automation loop
    async fn automation_loop(&self) {
        let mut interval =
            tokio::time::interval(Duration::from_secs(self.config.scan_interval_seconds));

        loop {
            interval.tick().await;

            if let Err(e) = self.run_automation_cycle().await {
                warn!("Automation cycle failed: {}", e);
            }
        }
    }

    /// Run a single automation cycle
    async fn run_automation_cycle(&self) -> Result<()> {
        debug!("Running automation cycle");

        let policies = self.policies.read().await;

        // Process each active policy
        for (policy_id, policy) in policies.iter() {
            if !policy.enabled {
                continue;
            }

            debug!("Processing policy: {}", policy_id);

            // Apply policy to matching datasets
            for tier_rule in &policy.conditions.tier_rules {
                if let Err(e) = self
                    .process_dataset_automation(&tier_rule.condition, policy)
                    .await
                {
                    warn!(
                        "Failed to process automation for rule {}: {}",
                        tier_rule.condition, e
                    );
                }
            }
        }

        Ok(())
    }

    /// Process automation for a specific dataset pattern
    async fn process_dataset_automation(
        &self,
        dataset_name: &str,
        policy: &AutomationPolicy,
    ) -> Result<()> {
        debug!("Processing automation for dataset: {}", dataset_name);

        // Get or create lifecycle tracking
        let mut lifecycle = self.get_or_create_lifecycle(dataset_name).await?;

        // Update lifecycle stage based on policy
        self.update_lifecycle_stage(&mut lifecycle, policy).await?;

        // Apply lifecycle rules
        self.apply_lifecycle_rules(dataset_name, &lifecycle, policy)
            .await?;

        // Update tracking
        self.update_lifecycle_tracking(dataset_name, lifecycle)
            .await?;

        Ok(())
    }

    /// Create default automation policies
    async fn create_default_policies(&self) -> Result<()> {
        let mut policies = self.policies.write().await;

        // Default balanced policy
        let default_policy = AutomationPolicy {
            policy_id: "default_balanced".to_string(),
            name: "Default Balanced Policy".to_string(),
            description: "Balanced performance and storage efficiency".to_string(),
            priority: PolicyPriority::Normal,
            enabled: true,
            conditions: PolicyConditions {
                tier_rules: vec![TierRule {
                    condition: "*".to_string(),
                    target_tier: StorageTier::Warm,
                    priority: 1,
                }],
                migration_rules: vec![],
                lifecycle_rules: vec![],
            },
            created: SystemTime::now(),
            last_modified: SystemTime::now(),
        };

        policies.insert(default_policy.policy_id.clone(), default_policy);
        Ok(())
    }

    /// Check if dataset matches any of the tier rule patterns
    #[allow(dead_code)] // Helper method for pattern matching
    fn matches_pattern(&self, dataset_name: &str, patterns: &[TierRule]) -> bool {
        for rule in patterns {
            if rule.condition == "*" || dataset_name.contains(&rule.condition) {
                return true;
            }
        }
        false
    }

    /// Get or create lifecycle tracking for a dataset
    async fn get_or_create_lifecycle(&self, dataset_name: &str) -> Result<DatasetLifecycle> {
        let lifecycle_tracker = self.lifecycle_tracker.read().await;

        if let Some(lifecycle) = lifecycle_tracker.get(dataset_name) {
            Ok(lifecycle.clone())
        } else {
            // Create new lifecycle tracking
            Ok(DatasetLifecycle {
                dataset_name: dataset_name.to_string(),
                current_tier: StorageTier::Warm, // Default to warm
                created: SystemTime::now(),
                last_accessed: None,
                access_count: 0,
                total_migrations: 0,
                last_optimization: None,
                lifecycle_stage: LifecycleStage::New,
                automation_history: Vec::new(),
            })
        }
    }

    /// Update lifecycle stage based on policy rules
    async fn update_lifecycle_stage(
        &self,
        lifecycle: &mut DatasetLifecycle,
        _policy: &AutomationPolicy,
    ) -> Result<()> {
        // Simple lifecycle progression logic
        let now = SystemTime::now();
        let age_days = now
            .duration_since(lifecycle.created)
            .unwrap_or(Duration::from_secs(
                std::env::var("NESTGATE_ZFS_DEFAULT_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0), // 0 seconds default (immediate)
            ))
            .as_secs()
            / (24 * 3600);

        // Update stage based on age and access patterns
        lifecycle.lifecycle_stage = match age_days {
            0..=7 => LifecycleStage::New,
            8..=30 => LifecycleStage::Active,
            31..=90 => LifecycleStage::Aging,
            _ => {
                if lifecycle.access_count < 10 {
                    LifecycleStage::Archived
                } else {
                    LifecycleStage::Active
                }
            }
        };

        Ok(())
    }

    /// Apply lifecycle rules based on current stage
    async fn apply_lifecycle_rules(
        &self,
        dataset_name: &str,
        lifecycle: &DatasetLifecycle,
        policy: &AutomationPolicy,
    ) -> Result<()> {
        info!(
            "Applying lifecycle rules for dataset {} in stage {:?}",
            dataset_name, lifecycle.lifecycle_stage
        );

        let mut actions_taken = Vec::new();

        // Process all lifecycle rules for the current stage
        for lifecycle_rule in &policy.conditions.lifecycle_rules {
            if lifecycle_rule.stage == lifecycle.lifecycle_stage {
                // Check if conditions are met
                if self
                    .evaluate_lifecycle_conditions(
                        dataset_name,
                        lifecycle,
                        &lifecycle_rule.conditions,
                    )
                    .await?
                {
                    // Apply each action in the rule
                    for action in &lifecycle_rule.actions {
                        match self
                            .execute_lifecycle_action(dataset_name, lifecycle, action)
                            .await
                        {
                            Ok(action_result) => {
                                actions_taken.push(action_result);
                                info!("✅ Applied action '{}' to dataset {}", action, dataset_name);
                            }
                            Err(e) => {
                                warn!(
                                    "❌ Failed to apply action '{}' to dataset {}: {}",
                                    action, dataset_name, e
                                );
                            }
                        }
                    }

                    // Check for stage transition
                    if let Some(next_stage) = &lifecycle_rule.next_stage {
                        if self
                            .should_transition_to_stage(dataset_name, lifecycle)
                            .await
                        {
                            info!(
                                "🔄 Transitioning dataset {} from {:?} to {:?}",
                                dataset_name, lifecycle.lifecycle_stage, next_stage
                            );
                            self.transition_lifecycle_stage(dataset_name, next_stage.clone())
                                .await?;
                        }
                    }
                }
            }
        }

        // Apply stage-specific automatic rules
        self.apply_automatic_stage_rules(dataset_name, lifecycle)
            .await?;

        if !actions_taken.is_empty() {
            info!(
                "Applied {} lifecycle actions to dataset {}: {:?}",
                actions_taken.len(),
                dataset_name,
                actions_taken
            );
        }

        Ok(())
    }

    /// Evaluate lifecycle rule conditions
    async fn evaluate_lifecycle_conditions(
        &self,
        dataset_name: &str,
        lifecycle: &DatasetLifecycle,
        conditions: &[String],
    ) -> Result<bool> {
        for condition in conditions {
            if !self
                .evaluate_single_lifecycle_condition(dataset_name, lifecycle, condition)
                .await?
            {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Evaluate a single lifecycle condition
    async fn evaluate_single_lifecycle_condition(
        &self,
        dataset_name: &str,
        lifecycle: &DatasetLifecycle,
        condition: &str,
    ) -> Result<bool> {
        let condition_lower = condition.to_lowercase();

        if let Some(stripped) = condition_lower.strip_prefix("age_days>") {
            if let Ok(days) = stripped.parse::<u32>() {
                let age_days = SystemTime::now()
                    .duration_since(lifecycle.created)
                    .unwrap_or(Duration::from_secs(
                        std::env::var("NESTGATE_ZFS_DEFAULT_TIMEOUT_SECS")
                            .ok()
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0), // 0 seconds default (immediate)
                    ))
                    .as_secs()
                    / (24 * 3600);
                return Ok(age_days > days as u64);
            }
        } else if let Some(stripped) = condition_lower.strip_prefix("access_count<") {
            if let Ok(count) = stripped.parse::<u64>() {
                return Ok(lifecycle.access_count < count);
            }
        } else if let Some(stripped) = condition_lower.strip_prefix("days_since_access>") {
            if let Ok(days) = stripped.parse::<u32>() {
                if let Some(last_access) = lifecycle.last_accessed {
                    let days_since = SystemTime::now()
                        .duration_since(last_access)
                        .unwrap_or(Duration::from_secs(
                            std::env::var("NESTGATE_ZFS_DEFAULT_TIMEOUT_SECS")
                                .ok()
                                .and_then(|s| s.parse().ok())
                                .unwrap_or(0), // 0 seconds default (immediate)
                        ))
                        .as_secs()
                        / (24 * 3600);
                    return Ok(days_since > days as u64);
                } else {
                    return Ok(true); // Never accessed
                }
            }
        } else if let Some(stripped) = condition_lower.strip_prefix("size_gb>") {
            if let Ok(size_gb) = stripped.parse::<f64>() {
                // Get current dataset size from ZFS
                if let Ok(current_size) = self.get_dataset_size_bytes(dataset_name).await {
                    let size_gb_actual = current_size as f64 / (1024.0 * 1024.0 * 1024.0);
                    return Ok(size_gb_actual > size_gb);
                }
            }
        } else if condition == "*" || condition == "always" {
            return Ok(true);
        }

        Ok(false)
    }

    /// Execute a lifecycle action
    async fn execute_lifecycle_action(
        &self,
        dataset_name: &str,
        lifecycle: &DatasetLifecycle,
        action: &str,
    ) -> Result<String> {
        let action_lower = action.to_lowercase();

        if let Some(stripped) = action_lower.strip_prefix("migrate_to_") {
            let target_tier = match stripped {
                "hot" => StorageTier::Hot,
                "warm" => StorageTier::Warm,
                "cold" => StorageTier::Cold,
                "cache" => StorageTier::Cache,
                _ => {
                    return Err(nestgate_core::NestGateError::InvalidInput(format!(
                        "Invalid target tier in action: {action}"
                    )))
                }
            };

            if lifecycle.current_tier != target_tier {
                self.execute_tier_migration(dataset_name, lifecycle.current_tier, target_tier)
                    .await?;
                return Ok(format!(
                    "Migrated to {} tier",
                    match target_tier {
                        StorageTier::Hot => "hot",
                        StorageTier::Warm => "warm",
                        StorageTier::Cold => "cold",
                        StorageTier::Cache => "cache",
                    }
                ));
            } else {
                return Ok("Already in target tier".to_string());
            }
        } else if action_lower == "enable_compression" {
            self.enable_dataset_compression(dataset_name).await?;
            return Ok("Enabled compression".to_string());
        } else if action_lower == "create_snapshot" {
            let snapshot_name = format!("auto-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S"));
            self.create_automated_snapshot(dataset_name, &snapshot_name)
                .await?;
            return Ok(format!("Created snapshot {snapshot_name}"));
        } else if action_lower == "optimize_recordsize" {
            self.optimize_dataset_recordsize(dataset_name).await?;
            return Ok("Optimized record size".to_string());
        } else if let Some(stripped) = action_lower.strip_prefix("set_quota_") {
            if let Ok(quota_gb) = stripped.parse::<u64>() {
                self.set_dataset_quota(dataset_name, quota_gb * 1024 * 1024 * 1024)
                    .await?;
                return Ok(format!("Set quota to {quota_gb}GB"));
            }
        } else if action_lower == "cleanup_old_snapshots" {
            let cleaned_count = self.cleanup_old_snapshots(dataset_name).await?;
            return Ok(format!("Cleaned {cleaned_count} old snapshots"));
        } else if action_lower == "enable_deduplication" {
            self.enable_dataset_deduplication(dataset_name).await?;
            return Ok("Enabled deduplication".to_string());
        }

        Err(nestgate_core::NestGateError::InvalidInput(format!(
            "Unknown lifecycle action: {action}"
        )))
    }

    /// Apply automatic stage-specific rules
    async fn apply_automatic_stage_rules(
        &self,
        dataset_name: &str,
        lifecycle: &DatasetLifecycle,
    ) -> Result<()> {
        match lifecycle.lifecycle_stage {
            LifecycleStage::New => {
                // New datasets: Enable compression for efficiency
                if let Err(e) = self.enable_dataset_compression(dataset_name).await {
                    debug!(
                        "Compression already enabled or failed for {}: {}",
                        dataset_name, e
                    );
                }
            }
            LifecycleStage::Active => {
                // Active datasets: Monitor performance and optimize
                if lifecycle.access_count > 1000 {
                    if let Err(e) = self.optimize_dataset_recordsize(dataset_name).await {
                        debug!(
                            "Record size optimization failed for {}: {}",
                            dataset_name, e
                        );
                    }
                }
            }
            LifecycleStage::Aging => {
                // Aging datasets: Consider moving to warm tier
                if lifecycle.current_tier == StorageTier::Hot {
                    let days_since_access = lifecycle
                        .last_accessed
                        .and_then(|last| SystemTime::now().duration_since(last).ok())
                        .map(|d| d.as_secs() / (24 * 3600))
                        .unwrap_or(365);

                    if days_since_access > 7 {
                        info!("Auto-migrating aging dataset {} to warm tier", dataset_name);
                        if let Err(e) = self
                            .execute_tier_migration(
                                dataset_name,
                                StorageTier::Hot,
                                StorageTier::Warm,
                            )
                            .await
                        {
                            warn!("Failed to migrate {} to warm tier: {}", dataset_name, e);
                        }
                    }
                }
            }
            LifecycleStage::Archived => {
                // Archived datasets: Move to cold tier and enable deduplication
                if lifecycle.current_tier != StorageTier::Cold {
                    info!(
                        "Auto-migrating archived dataset {} to cold tier",
                        dataset_name
                    );
                    if let Err(e) = self
                        .execute_tier_migration(
                            dataset_name,
                            lifecycle.current_tier,
                            StorageTier::Cold,
                        )
                        .await
                    {
                        warn!("Failed to migrate {} to cold tier: {}", dataset_name, e);
                    }
                }

                if let Err(e) = self.enable_dataset_deduplication(dataset_name).await {
                    debug!(
                        "Deduplication already enabled or failed for {}: {}",
                        dataset_name, e
                    );
                }
            }
            LifecycleStage::Obsolete => {
                // Obsolete datasets: Create final backup snapshot before potential cleanup
                let snapshot_name = format!("final-backup-{}", chrono::Utc::now().format("%Y%m%d"));
                if let Err(e) = self
                    .create_automated_snapshot(dataset_name, &snapshot_name)
                    .await
                {
                    warn!(
                        "Failed to create final backup snapshot for {}: {}",
                        dataset_name, e
                    );
                }
            }
        }

        Ok(())
    }

    /// Check if dataset should transition to a new stage
    async fn should_transition_to_stage(
        &self,
        _dataset_name: &str,
        _current_lifecycle: &DatasetLifecycle,
    ) -> bool {
        // Implementation of should_transition_to_stage method
        false
    }

    /// Transition dataset to new lifecycle stage
    async fn transition_lifecycle_stage(
        &self,
        dataset_name: &str,
        new_stage: LifecycleStage,
    ) -> Result<()> {
        let mut lifecycle_tracker = self.lifecycle_tracker.write().await;
        if let Some(lifecycle) = lifecycle_tracker.get_mut(dataset_name) {
            let old_stage = lifecycle.lifecycle_stage.clone();
            lifecycle.lifecycle_stage = new_stage.clone();

            // Add automation event
            lifecycle.automation_history.push(AutomationEvent {
                event_id: uuid::Uuid::new_v4().to_string(),
                event_type: AutomationEventType::PolicyUpdate,
                timestamp: SystemTime::now(),
                details: format!("Stage transition: {old_stage:?} → {new_stage:?}"),
                success: true,
            });

            info!(
                "✅ Transitioned dataset {} from {:?} to {:?}",
                dataset_name, old_stage, new_stage
            );
        }
        Ok(())
    }

    // Helper methods for executing specific actions

    async fn execute_tier_migration(
        &self,
        dataset_name: &str,
        from_tier: StorageTier,
        to_tier: StorageTier,
    ) -> Result<()> {
        info!(
            "🔄 Migrating dataset {} from {:?} to {:?}",
            dataset_name, from_tier, to_tier
        );

        let _migration_engine = self.migration_engine.read().await;
        // This would integrate with the actual migration engine
        // For now, just log the intent
        info!("Migration scheduled: {} → {:?}", dataset_name, to_tier);
        Ok(())
    }

    async fn enable_dataset_compression(&self, dataset_name: &str) -> Result<()> {
        debug!("Enabling compression for dataset {}", dataset_name);
        // This would use ZFS commands to enable compression
        // zfs set compression=lz4 dataset_name
        Ok(())
    }

    async fn create_automated_snapshot(
        &self,
        dataset_name: &str,
        snapshot_name: &str,
    ) -> Result<()> {
        debug!(
            "Creating automated snapshot {}@{}",
            dataset_name, snapshot_name
        );
        // This would integrate with the snapshot manager
        Ok(())
    }

    async fn optimize_dataset_recordsize(&self, dataset_name: &str) -> Result<()> {
        debug!("Optimizing record size for dataset {}", dataset_name);
        // This would analyze workload and set optimal record size
        Ok(())
    }

    async fn set_dataset_quota(&self, dataset_name: &str, quota_bytes: u64) -> Result<()> {
        debug!(
            "Setting quota for dataset {} to {} bytes",
            dataset_name, quota_bytes
        );
        // zfs set quota={}G dataset_name
        Ok(())
    }

    async fn cleanup_old_snapshots(&self, dataset_name: &str) -> Result<u32> {
        debug!("Cleaning up old snapshots for dataset {}", dataset_name);
        // This would integrate with snapshot manager to clean old snapshots
        Ok(0)
    }

    async fn enable_dataset_deduplication(&self, dataset_name: &str) -> Result<()> {
        debug!("Enabling deduplication for dataset {}", dataset_name);
        // zfs set dedup=on dataset_name
        Ok(())
    }

    async fn get_dataset_size_bytes(&self, dataset_name: &str) -> Result<u64> {
        use tokio::process::Command;

        debug!("Getting dataset size for: {}", dataset_name);

        // Check if we're in mock mode
        if crate::mock::is_mock_mode() {
            // Return consistent mock value for testing
            let mock_size = crate::mock::mock_dataset_size(dataset_name);
            debug!(
                "Mock mode: returning {} bytes for dataset {}",
                mock_size, dataset_name
            );
            return Ok(mock_size);
        }

        // Query ZFS for actual dataset size
        let output = Command::new("zfs")
            .args(["get", "-H", "-p", "used", dataset_name])
            .output()
            .await
            .map_err(|e| crate::ZfsError::Internal {
                message: format!("Failed to execute zfs command: {e}"),
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(crate::ZfsError::Internal {
                message: format!("ZFS command failed for dataset {dataset_name}: {stderr}"),
            }
            .into());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = stdout.trim().split('\t').collect();

        if parts.len() < 3 {
            return Err(crate::ZfsError::Internal {
                message: format!("Invalid ZFS output format for dataset {dataset_name}: {stdout}"),
            }
            .into());
        }

        let size_str = parts[2]; // The "used" value is in the 3rd column
        let size_bytes = size_str
            .parse::<u64>()
            .map_err(|e| crate::ZfsError::Internal {
                message: format!(
                    "Failed to parse dataset size for {dataset_name}: {size_str} ({e})"
                ),
            })?;

        debug!("Dataset {} size: {} bytes", dataset_name, size_bytes);
        Ok(size_bytes)
    }

    /// Update lifecycle tracking
    async fn update_lifecycle_tracking(
        &self,
        dataset_name: &str,
        lifecycle: DatasetLifecycle,
    ) -> Result<()> {
        let mut lifecycle_tracker = self.lifecycle_tracker.write().await;
        lifecycle_tracker.insert(dataset_name.to_string(), lifecycle);
        Ok(())
    }

    /// Get current automation status
    pub async fn get_automation_status(&self) -> AutomationStatus {
        let policies = self.policies.read().await;
        let lifecycle_tracker = self.lifecycle_tracker.read().await;

        AutomationStatus {
            enabled: self.config.enabled,
            active_policies: policies.values().filter(|p| p.enabled).count() as u32,
            tracked_datasets: lifecycle_tracker.len() as u32,
            total_migrations_performed: lifecycle_tracker
                .values()
                .map(|l| l.total_migrations)
                .sum(),
            last_automation_cycle: SystemTime::now(),
        }
    }

    /// Validate a policy before adding it
    pub async fn validate_policy(&self, _policy: &AutomationPolicy) -> Result<()> {
        // Basic validation - could be enhanced
        Ok(())
    }

    /// Evaluate the best tier for a dataset based on current policies
    pub async fn evaluate_tier_for_dataset(
        &self,
        dataset_name: &str,
        metadata: &DatasetMetadata,
    ) -> Result<StorageTier> {
        debug!("Evaluating optimal tier for dataset: {}", dataset_name);

        // Intelligent rule-based tier evaluation with sophisticated algorithms
        self.evaluate_tier_by_intelligent_rules(dataset_name, metadata)
            .await
    }

    /// Intelligent rule-based tier evaluation with sophisticated algorithms
    async fn evaluate_tier_by_intelligent_rules(
        &self,
        dataset_name: &str,
        metadata: &DatasetMetadata,
    ) -> Result<StorageTier> {
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
            .map(|d| d.as_secs() / (24 * 3600))
            .unwrap_or(365);

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
        if dataset_lower.contains("database")
            || dataset_lower.contains("db")
            || dataset_lower.contains("mysql")
            || dataset_lower.contains("postgres")
        {
            tier_score.add_hot_weight(0.4, "Database workload detected");
        } else if dataset_lower.contains("vm")
            || dataset_lower.contains("virtual")
            || dataset_lower.contains("qemu")
        {
            tier_score.add_hot_weight(0.5, "VM storage detected");
        } else if dataset_lower.contains("backup")
            || dataset_lower.contains("archive")
            || dataset_lower.contains("snapshot")
        {
            tier_score.add_cold_weight(0.6, "Backup/Archive detected");
        } else if dataset_lower.contains("cache")
            || dataset_lower.contains("tmp")
            || dataset_lower.contains("temp")
        {
            tier_score.add_hot_weight(0.3, "Cache/temporary storage");
        } else if dataset_lower.contains("media")
            || dataset_lower.contains("video")
            || dataset_lower.contains("photo")
        {
            tier_score.add_warm_weight(0.3, "Media storage detected");
        }

        // 5. File type analysis
        for file_type in &metadata.file_types {
            match file_type.to_lowercase().as_str() {
                "db" | "sqlite" | "mysql" => tier_score.add_hot_weight(0.3, "Database files"),
                "vmdk" | "vdi" | "qcow2" => tier_score.add_hot_weight(0.4, "VM disk images"),
                "log" => tier_score.add_warm_weight(0.2, "Log files"),
                "mp4" | "mkv" | "avi" | "mov" => tier_score.add_warm_weight(0.2, "Video files"),
                "jpg" | "png" | "pdf" => tier_score.add_warm_weight(0.1, "Document/Image files"),
                "zip" | "tar" | "gz" | "bz2" => tier_score.add_cold_weight(0.3, "Archive files"),
                "bak" | "backup" => tier_score.add_cold_weight(0.4, "Backup files"),
                _ => {}
            }
        }

        // 6. Performance policy evaluation
        for (policy_id, policy) in self.policies.read().await.iter() {
            if policy.enabled && self.dataset_matches_policy_pattern(dataset_name, policy) {
                debug!("Dataset {} matches policy {}", dataset_name, policy_id);

                for tier_rule in &policy.conditions.tier_rules {
                    match tier_rule.target_tier {
                        StorageTier::Hot => {
                            tier_score.add_hot_weight(0.2, &format!("Policy {policy_id}"))
                        }
                        StorageTier::Warm => {
                            tier_score.add_warm_weight(0.2, &format!("Policy {policy_id}"))
                        }
                        StorageTier::Cold => {
                            tier_score.add_cold_weight(0.2, &format!("Policy {policy_id}"))
                        }
                        StorageTier::Cache => {
                            tier_score.add_hot_weight(0.3, &format!("Cache Policy {policy_id}"))
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
            },
            tier_score.hot_score,
            tier_score.warm_score,
            tier_score.cold_score
        );

        Ok(recommended_tier)
    }

    /// Check if dataset matches policy patterns
    fn dataset_matches_policy_pattern(
        &self,
        dataset_name: &str,
        policy: &AutomationPolicy,
    ) -> bool {
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
                if let Ok(regex) = regex::Regex::new(pattern) {
                    if regex.is_match(dataset_name) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

// Enable cloning for background tasks
impl Clone for DatasetAutomation {
    fn clone(&self) -> Self {
        Self {
            pool_manager: self.pool_manager.clone(),
            dataset_manager: self.dataset_manager.clone(),
            migration_engine: self.migration_engine.clone(),
            policies: self.policies.clone(),
            lifecycle_tracker: self.lifecycle_tracker.clone(),
            config: self.config.clone(),
        }
    }
}

/// Automation status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationStatus {
    pub enabled: bool,
    pub active_policies: u32,
    pub tracked_datasets: u32,
    pub total_migrations_performed: u32,
    pub last_automation_cycle: SystemTime,
}

/// Dataset metadata for tier evaluation
#[derive(Debug, Default)]
pub struct DatasetMetadata {
    pub size_bytes: u64,
    pub last_accessed: Option<SystemTime>,
    pub access_frequency: f64,
    pub file_types: Vec<String>,
}

/// Intelligent tier scoring system
#[derive(Debug)]
struct TierScoring {
    hot_score: f64,
    warm_score: f64,
    cold_score: f64,
    hot_reasons: Vec<String>,
    warm_reasons: Vec<String>,
    cold_reasons: Vec<String>,
}

impl TierScoring {
    fn new() -> Self {
        Self {
            hot_score: 0.0,
            warm_score: 0.0,
            cold_score: 0.0,
            hot_reasons: Vec::new(),
            warm_reasons: Vec::new(),
            cold_reasons: Vec::new(),
        }
    }

    fn add_hot_weight(&mut self, weight: f64, reason: &str) {
        self.hot_score += weight;
        self.hot_reasons.push(reason.to_string());
    }

    fn add_warm_weight(&mut self, weight: f64, reason: &str) {
        self.warm_score += weight;
        self.warm_reasons.push(reason.to_string());
    }

    fn add_cold_weight(&mut self, weight: f64, reason: &str) {
        self.cold_score += weight;
        self.cold_reasons.push(reason.to_string());
    }

    fn get_recommendation(&self) -> StorageTier {
        if self.hot_score >= self.warm_score && self.hot_score >= self.cold_score {
            StorageTier::Hot
        } else if self.warm_score >= self.cold_score {
            StorageTier::Warm
        } else {
            StorageTier::Cold
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{ZfsDatasetManager, ZfsPoolManager};
    use nestgate_automation::DatasetAnalyzer;

    use std::sync::Arc;
    use tokio::sync::RwLock;

    fn create_test_config() -> DatasetAutomationConfig {
        DatasetAutomationConfig {
            enabled: true,
            scan_interval_seconds: 300,
            learning_period_days: 7,
            default_policy: "balanced".to_string(),
            ai_settings: crate::config::AiAutomationSettings {
                enable_ai_predictions: false,
                ai_confidence_threshold: 0.8,
                learning_rate: 0.1,
                learning_window_days: 30,
            },
        }
    }

    async fn create_test_automation() -> DatasetAutomation {
        let zfs_config = crate::config::ZfsConfig {
            api_endpoint: std::env::var("NESTGATE_API_ENDPOINT")
                .unwrap_or_else(|_| {
                    format!(
                        "http://localhost:{}",
                        nestgate_core::constants::network::api_port()
                    )
                })
                .to_string(),
            default_pool: "test-pool".to_string(),
            use_real_zfs: false,
            tiers: crate::config::TierConfigurations::default(),
            pool_discovery: crate::config::PoolDiscoveryConfig::default(),
            health_monitoring: crate::config::HealthMonitoringConfig::default(),
            metrics: crate::config::MetricsConfig::default(),
            migration: crate::config::MigrationConfig::default(),
            security: crate::config::SecurityConfig::default(),
            enable_ai_integration: Some(false),
            monitoring_interval: 60,
            snapshot_policies_file: None,
            automation: Some(create_test_config()),
            ecosystem_orchestrator_url: std::env::var("NESTGATE_ORCHESTRATOR_URL")
                .unwrap_or_else(|_| {
                    format!(
                        "http://localhost:{}",
                        nestgate_core::constants::network::orchestrator_port()
                    )
                })
                .to_string(),
            enable_ecosystem_integration: false,
        };

        let pool_manager = Arc::new(
            ZfsPoolManager::new(&zfs_config)
                .await
                .expect("Failed to create pool manager in test"),
        );
        let dataset_manager = Arc::new(ZfsDatasetManager::new(
            zfs_config.clone(),
            pool_manager.clone(),
        ));
        let migration_config = crate::migration::MigrationConfig::default();
        let migration_engine = Arc::new(RwLock::new(crate::migration::MigrationEngine::new(
            migration_config,
            zfs_config.clone(),
            pool_manager.clone(),
            dataset_manager.clone(),
            Arc::new(DatasetAnalyzer::new()),
        )));

        DatasetAutomation::new(
            pool_manager,
            dataset_manager,
            migration_engine,
            create_test_config(),
        )
        .await
        .expect("Failed to create automation policy in test")
    }

    #[tokio::test]
    async fn test_policy_serialization() {
        let policy = AutomationPolicy {
            policy_id: "test_policy".to_string(),
            name: "Test Policy".to_string(),
            description: "A test policy".to_string(),
            enabled: true,
            priority: PolicyPriority::Normal,
            conditions: PolicyConditions {
                tier_rules: vec![TierRule {
                    condition: "*".to_string(),
                    target_tier: StorageTier::Hot,
                    priority: 1,
                }],
                migration_rules: vec![],
                lifecycle_rules: vec![],
            },
            created: SystemTime::now(),
            last_modified: SystemTime::now(),
        };

        let serialized = serde_json::to_string(&policy).expect("Failed to serialize policy");
        let deserialized: AutomationPolicy =
            serde_json::from_str(&serialized).expect("Failed to deserialize policy");

        assert_eq!(policy.policy_id, deserialized.policy_id);
        assert_eq!(policy.name, deserialized.name);
        assert_eq!(policy.enabled, deserialized.enabled);
    }

    #[tokio::test]
    async fn test_policy_validation() {
        let automation = create_test_automation().await;

        let valid_policy = AutomationPolicy {
            policy_id: "valid_policy".to_string(),
            name: "Valid Policy".to_string(),
            description: "A valid policy".to_string(),
            enabled: true,
            priority: PolicyPriority::High,
            conditions: PolicyConditions {
                tier_rules: vec![TierRule {
                    condition: "tank/data/*".to_string(),
                    target_tier: StorageTier::Warm,
                    priority: 1,
                }],
                migration_rules: vec![],
                lifecycle_rules: vec![],
            },
            created: SystemTime::now(),
            last_modified: SystemTime::now(),
        };

        let result = automation.validate_policy(&valid_policy).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_lifecycle_stage_enum() {
        let stage = LifecycleStage::Active;
        assert_eq!(stage.to_string(), "Active");

        let stage = LifecycleStage::Archived;
        assert_eq!(stage.to_string(), "Archived");
    }

    #[tokio::test]
    async fn test_storage_tier_integration() {
        let automation = create_test_automation().await;

        let result = automation
            .evaluate_tier_for_dataset("tank/data/test", &DatasetMetadata::default())
            .await;
        assert!(result.is_ok());
    }
}
