//! Main automation engine for ZFS dataset management
//!
//! This module contains the core DatasetAutomation engine that orchestrates
//! all automation activities including policy evaluation, lifecycle management,
//! and tier optimization.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use super::{
    actions, lifecycle, tier_evaluation,
    types::{
        AutomationPolicy, AutomationStatus, DatasetLifecycle, DatasetMetadata, PolicyConditions,
        PolicyPriority, TierRule,
    },
};
use crate::config::DatasetAutomationConfig;
use crate::{dataset::ZfsDatasetManager, migration::MigrationEngine, pool::ZfsPoolManager};
use nestgate_core::{Result, StorageTier};

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
        let mut lifecycle = {
            let lifecycle_tracker = self.lifecycle_tracker.read().await;
            lifecycle::get_or_create_lifecycle(dataset_name, &lifecycle_tracker)
        };

        // Update lifecycle stage based on policy
        lifecycle::update_lifecycle_stage(&mut lifecycle, policy).await?;

        // Apply lifecycle rules
        self.apply_lifecycle_rules(dataset_name, &lifecycle, policy)
            .await?;

        // Update tracking
        self.update_lifecycle_tracking(dataset_name, lifecycle)
            .await?;

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
                if lifecycle::evaluate_lifecycle_conditions(
                    dataset_name,
                    lifecycle,
                    &lifecycle_rule.conditions,
                )
                .await?
                {
                    // Apply each action in the rule
                    for action in &lifecycle_rule.actions {
                        match actions::execute_lifecycle_action(dataset_name, lifecycle, action)
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
                        if lifecycle::should_transition_to_stage(dataset_name, lifecycle).await {
                            info!(
                                "🔄 Transitioning dataset {} from {:?} to {:?}",
                                dataset_name, lifecycle.lifecycle_stage, next_stage
                            );
                            let mut lifecycle_tracker = self.lifecycle_tracker.write().await;
                            lifecycle::transition_lifecycle_stage(
                                dataset_name,
                                next_stage.clone(),
                                &mut lifecycle_tracker,
                            )
                            .await?;
                        }
                    }
                }
            }
        }

        // Apply stage-specific automatic rules
        actions::apply_automatic_stage_rules(dataset_name, lifecycle).await?;

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

        let policies = self.policies.read().await;
        tier_evaluation::evaluate_tier_by_intelligent_rules(dataset_name, metadata, &policies).await
    }
}

// Enable cloning for background tasks using Arc patterns for zero-copy sharing
impl Clone for DatasetAutomation {
    fn clone(&self) -> Self {
        Self {
            pool_manager: Arc::clone(&self.pool_manager),
            dataset_manager: Arc::clone(&self.dataset_manager),
            migration_engine: Arc::clone(&self.migration_engine),
            policies: Arc::clone(&self.policies),
            lifecycle_tracker: Arc::clone(&self.lifecycle_tracker),
            config: self.config.clone(), // Keep config clone as it's lightweight
        }
    }
}
