// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// This module contains the core DatasetAutomation engine that orchestrates
// all automation activities including policy evaluation, lifecycle management,
// and tier optimization.

//! Engine module

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use tokio::sync::RwLock;

// Type aliases to reduce complexity
type PolicyMap = Arc<RwLock<HashMap<String, AutomationPolicy>>>;
/// Type alias for `LifecycleMap`
type LifecycleMap = Arc<RwLock<HashMap<String, DatasetLifecycle>>>;
use tracing::debug;
use tracing::info;
use tracing::warn;
// Removed unused tracing import

use super::{
    actions, lifecycle, tier_evaluation,
    types::{
        AutomationPolicy, AutomationStatus, DatasetLifecycle, DatasetMetadata, PolicyConditions,
        PolicyPriority,
    },
};
use crate::config::DatasetAutomationConfig;
use crate::error::ZfsResult as Result;
use crate::types::StorageTier;
use crate::{dataset::ZfsDatasetManager, pool::ZfsPoolManager};
use nestgate_core::NestGateError;
use nestgate_core::error::NestGateUnifiedError;
// Migration engine placeholder - not yet implemented

/// Intelligent dataset automation engine
#[derive(Debug)]
/// Datasetautomation
pub struct DatasetAutomation {
    /// Pool management
    pool_manager: Arc<ZfsPoolManager>,
    /// Dataset management
    dataset_manager: Arc<ZfsDatasetManager>,
    /// Migration engine for tier movement (placeholder - not yet implemented)
    // migration_engine: Arc<RwLock<MigrationEngine>>,
    /// Active automation policies
    policies: PolicyMap,
    /// Lifecycle tracking
    lifecycle_tracker: LifecycleMap,
    /// Configuration
    config: DatasetAutomationConfig,
}
impl DatasetAutomation {
    /// Create new dataset automation engine
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn new(
        pool_manager: Arc<ZfsPoolManager>,
        dataset_manager: Arc<ZfsDatasetManager>,
        // migration_engine: Arc<RwLock<MigrationEngine>>, // MigrationEngine not yet implemented
        config: DatasetAutomationConfig,
    ) -> Result<Self> {
        info!("Initializing Dataset Automation Engine");

        let policies = Arc::new(RwLock::new(HashMap::new()));
        let lifecycle_tracker = Arc::new(RwLock::new(HashMap::new()));

        let automation = Self {
            pool_manager,
            dataset_manager,
            // migration_engine, // Commented out - not yet implemented
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
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
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

        let policies: Vec<(String, AutomationPolicy)> = self
            .policies
            .read()
            .await
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        // Process each active policy
        for (policy_id, policy) in &policies {
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
        lifecycle::update_lifecycle_stage(&mut lifecycle, policy)?;

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

        // Process all lifecycle rules for the current stage
        for lifecycle_rule in &policy.conditions.lifecycle_rules {
            if lifecycle_rule.stage == lifecycle.lifecycle_stage {
                // Check if conditions are met
                if lifecycle::evaluate_lifecycle_conditions(
                    dataset_name,
                    lifecycle,
                    &lifecycle_rule.conditions,
                )? {
                    // Apply each action in the rule
                    for action in &lifecycle_rule.actions {
                        match actions::execute_lifecycle_action(dataset_name, lifecycle, action) {
                            Ok(action_result) => {
                                let success = action_result.success;
                                let msg = action_result.message.clone();
                                if success {
                                    info!(
                                        "Applied action '{}' to dataset {}",
                                        action, dataset_name
                                    );
                                } else {
                                    warn!(
                                        "Action '{}' did not succeed for dataset {}: {}",
                                        action, dataset_name, msg
                                    );
                                }
                            }
                            Err(e) => {
                                warn!(
                                    "Failed to apply action '{}' to dataset {}: {}",
                                    action, dataset_name, e
                                );
                            }
                        }
                    }

                    // Check for stage transition
                    if let Some(next_stage) = &lifecycle_rule.next_stage
                        && lifecycle::should_transition_to_stage(dataset_name, lifecycle)
                    {
                        info!(
                            "Transitioning dataset {} from {:?} to {:?}",
                            dataset_name, lifecycle.lifecycle_stage, next_stage
                        );
                        let mut lifecycle_tracker = self.lifecycle_tracker.write().await;
                        lifecycle::transition_lifecycle_stage(
                            dataset_name,
                            next_stage.clone(),
                            &mut lifecycle_tracker,
                        )?;
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
        // Default balanced policy
        let default_policy = AutomationPolicy {
            policy_id: "default".to_string(),
            name: "Default Automation Policy".to_string(),
            description: "Default policy for new datasets".to_string(),
            priority: PolicyPriority::Normal,
            enabled: true,
            conditions: PolicyConditions {
                tier_rules: vec![],
                migration_rules: vec![],
                lifecycle_rules: vec![],
            },
            created: SystemTime::now(),
            last_modified: SystemTime::now(),
        };

        self.policies
            .write()
            .await
            .insert(default_policy.policy_id.clone(), default_policy);
        Ok(())
    }

    /// Update lifecycle tracking
    async fn update_lifecycle_tracking(
        &self,
        dataset_name: &str,
        lifecycle: DatasetLifecycle,
    ) -> Result<()> {
        {
            let mut lifecycle_tracker = self.lifecycle_tracker.write().await;
            lifecycle_tracker.insert(dataset_name.to_string(), lifecycle);
        }
        Ok(())
    }

    /// Get current automation status
    pub async fn get_automation_status(&self) -> AutomationStatus {
        let policies = self.policies.read().await;
        let lifecycle_tracker = self.lifecycle_tracker.read().await;

        AutomationStatus {
            enabled: self.config.enabled,
            active_policies: u32::try_from(policies.values().filter(|p| p.enabled).count())
                .unwrap_or(u32::MAX),
            tracked_datasets: u32::try_from(lifecycle_tracker.len()).unwrap_or(u32::MAX),
            total_migrations_performed: lifecycle_tracker
                .values()
                .map(|l| l.total_migrations)
                .sum(),
            last_automation_cycle: SystemTime::now(),
        }
    }

    /// Validate a policy before adding it
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub const fn validate_policy(&self, _policy: &AutomationPolicy) -> Result<()> {
        // Basic validation - could be enhanced
        Ok(())
    }

    /// Evaluate the best tier for a dataset based on current policies
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn evaluate_tier_for_dataset(
        &self,
        dataset_name: &str,
        metadata: &DatasetMetadata,
    ) -> Result<StorageTier> {
        debug!("Evaluating optimal tier for dataset: {}", dataset_name);

        let policies = self.policies.read().await;
        tier_evaluation::evaluate_tier_by_intelligent_rules(dataset_name, metadata, &policies)
            .map_err(|e| {
                NestGateUnifiedError::storage_error(format!("Tier evaluation failed: {e}"))
            })
    }

    /// Reserved for orchestrated tier migration; the migration engine is not wired yet.
    pub async fn migrate_dataset_to_tier(
        &self,
        _dataset_name: &str,
        _target_tier: crate::types::StorageTier,
    ) -> Result<()> {
        Err(NestGateError::not_implemented(
            "ZFS tier migration engine not yet wired; coordinate with migration IPC or use native ZFS tooling",
        ))
    }
}

// Enable cloning for background tasks using Arc patterns for zero-copy sharing
impl Clone for DatasetAutomation {
    /// Clone
    fn clone(&self) -> Self {
        Self {
            pool_manager: Arc::clone(&self.pool_manager),
            dataset_manager: Arc::clone(&self.dataset_manager),
            policies: self.policies.clone(),
            lifecycle_tracker: self.lifecycle_tracker.clone(),
            config: self.config.clone(),
        }
    }
}

#[cfg(test)]
impl DatasetAutomation {
    /// Replace the policy map for unit tests covering [`DatasetAutomation::run_automation_cycle`].
    pub(crate) async fn replace_policies_for_test(
        &self,
        policies: HashMap<String, AutomationPolicy>,
    ) {
        *self.policies.write().await = policies;
    }
}

#[cfg(test)]
mod internal_tests {
    use super::DatasetAutomation;
    use crate::automation::types::{
        AutomationPolicy, BandwidthLimits, LifecycleRule, LifecycleStage, MigrationRule,
        PolicyConditions, PolicyPriority, TierRule,
    };
    use crate::config::{DatasetAutomationConfig, ZfsConfig};
    use crate::dataset::ZfsDatasetManager;
    use crate::pool::ZfsPoolManager;
    use crate::types::StorageTier;
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::time::SystemTime;

    async fn test_engine() -> DatasetAutomation {
        let config = ZfsConfig::default();
        let pool_manager = Arc::new(ZfsPoolManager::new(&config).await.expect("pool new"));
        let dataset_manager =
            Arc::new(ZfsDatasetManager::new(config.clone(), pool_manager.clone()));
        DatasetAutomation::new(
            pool_manager,
            dataset_manager,
            DatasetAutomationConfig::default(),
        )
        .await
        .expect("automation new")
    }

    #[tokio::test]
    async fn migrate_dataset_to_tier_returns_not_implemented() {
        let automation = test_engine().await;
        let err = automation
            .migrate_dataset_to_tier("pool/ds", StorageTier::Cold)
            .await
            .expect_err("expected not implemented");
        let msg = err.to_string().to_lowercase();
        assert!(msg.contains("not") && msg.contains("implement"), "{err}");
    }

    #[tokio::test]
    async fn run_automation_cycle_handles_tier_rules() {
        let automation = test_engine().await;
        let policy = AutomationPolicy {
            policy_id: "tier_policy".into(),
            name: "Tier policy".into(),
            description: "test".into(),
            priority: PolicyPriority::Normal,
            enabled: true,
            conditions: PolicyConditions {
                tier_rules: vec![TierRule {
                    condition: "tank/tier_rule_ds".into(),
                    target_tier: StorageTier::Warm,
                    priority: 1,
                }],
                migration_rules: vec![],
                lifecycle_rules: vec![],
            },
            created: SystemTime::now(),
            last_modified: SystemTime::now(),
        };
        let mut map = HashMap::new();
        map.insert("tier_policy".into(), policy);
        automation.replace_policies_for_test(map).await;
        automation.run_automation_cycle().await.expect("cycle");
    }

    #[tokio::test]
    async fn run_automation_cycle_skips_when_policy_disabled() {
        let automation = test_engine().await;
        let policy = AutomationPolicy {
            policy_id: "off".into(),
            name: "Off".into(),
            description: "test".into(),
            priority: PolicyPriority::Normal,
            enabled: false,
            conditions: PolicyConditions {
                tier_rules: vec![TierRule {
                    condition: "tank/x".into(),
                    target_tier: StorageTier::Hot,
                    priority: 1,
                }],
                migration_rules: vec![],
                lifecycle_rules: vec![],
            },
            created: SystemTime::now(),
            last_modified: SystemTime::now(),
        };
        let mut map = HashMap::new();
        map.insert("off".into(), policy);
        automation.replace_policies_for_test(map).await;
        automation.run_automation_cycle().await.expect("cycle");
    }

    #[tokio::test]
    async fn run_automation_cycle_runs_lifecycle_rule_actions() {
        let automation = test_engine().await;
        let policy = AutomationPolicy {
            policy_id: "lc".into(),
            name: "LC".into(),
            description: "test".into(),
            priority: PolicyPriority::Normal,
            enabled: true,
            conditions: PolicyConditions {
                tier_rules: vec![TierRule {
                    condition: "tank/lifecycle_ds".into(),
                    target_tier: StorageTier::Cold,
                    priority: 1,
                }],
                migration_rules: vec![],
                lifecycle_rules: vec![LifecycleRule {
                    stage: LifecycleStage::New,
                    next_stage: None,
                    conditions: vec!["always".into()],
                    actions: vec!["unknown_action_for_test".into()],
                }],
            },
            created: SystemTime::now(),
            last_modified: SystemTime::now(),
        };
        let mut map = HashMap::new();
        map.insert("lc".into(), policy);
        automation.replace_policies_for_test(map).await;
        automation.run_automation_cycle().await.expect("cycle");
    }

    #[tokio::test]
    async fn run_automation_cycle_with_empty_policy_map() {
        let automation = test_engine().await;
        automation.replace_policies_for_test(HashMap::new()).await;
        automation.run_automation_cycle().await.expect("cycle");
    }

    #[tokio::test]
    async fn run_automation_cycle_skips_when_only_migration_rules_present() {
        let automation = test_engine().await;
        let policy = AutomationPolicy {
            policy_id: "mig_only".into(),
            name: "Migration only".into(),
            description: "test".into(),
            priority: PolicyPriority::Normal,
            enabled: true,
            conditions: PolicyConditions {
                tier_rules: vec![],
                migration_rules: vec![MigrationRule {
                    source_tier: StorageTier::Hot,
                    target_tier: StorageTier::Cold,
                    condition: "age > 1".into(),
                    bandwidth_limits: BandwidthLimits::default(),
                    schedule: "daily".into(),
                }],
                lifecycle_rules: vec![],
            },
            created: SystemTime::now(),
            last_modified: SystemTime::now(),
        };
        let mut map = HashMap::new();
        map.insert("mig_only".into(), policy);
        automation.replace_policies_for_test(map).await;
        automation.run_automation_cycle().await.expect("cycle");
        let st = automation.get_automation_status().await;
        assert_eq!(st.tracked_datasets, 0);
    }

    #[tokio::test]
    async fn run_automation_cycle_tracks_dataset_after_tier_processing() {
        let automation = test_engine().await;
        let policy = AutomationPolicy {
            policy_id: "track".into(),
            name: "Track".into(),
            description: "test".into(),
            priority: PolicyPriority::Normal,
            enabled: true,
            conditions: PolicyConditions {
                tier_rules: vec![TierRule {
                    condition: "tank/tracked_ds".into(),
                    target_tier: StorageTier::Warm,
                    priority: 1,
                }],
                migration_rules: vec![],
                lifecycle_rules: vec![],
            },
            created: SystemTime::now(),
            last_modified: SystemTime::now(),
        };
        let mut map = HashMap::new();
        map.insert("track".into(), policy);
        automation.replace_policies_for_test(map).await;
        automation.run_automation_cycle().await.expect("cycle");
        let st = automation.get_automation_status().await;
        assert_eq!(st.tracked_datasets, 1);
        assert!(st.active_policies >= 1);
    }

    #[tokio::test]
    async fn run_automation_cycle_runs_compress_lifecycle_action() {
        let automation = test_engine().await;
        let policy = AutomationPolicy {
            policy_id: "compress_lc".into(),
            name: "Compress".into(),
            description: "test".into(),
            priority: PolicyPriority::Normal,
            enabled: true,
            conditions: PolicyConditions {
                tier_rules: vec![TierRule {
                    condition: "tank/compress_ds".into(),
                    target_tier: StorageTier::Warm,
                    priority: 1,
                }],
                migration_rules: vec![],
                lifecycle_rules: vec![LifecycleRule {
                    stage: LifecycleStage::New,
                    next_stage: None,
                    conditions: vec!["always".into()],
                    actions: vec!["compress".into()],
                }],
            },
            created: SystemTime::now(),
            last_modified: SystemTime::now(),
        };
        let mut map = HashMap::new();
        map.insert("compress_lc".into(), policy);
        automation.replace_policies_for_test(map).await;
        automation.run_automation_cycle().await.expect("cycle");
    }
}
