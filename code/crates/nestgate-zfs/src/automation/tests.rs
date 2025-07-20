//! Tests for ZFS automation components
//!
//! This module contains comprehensive tests for all automation functionality
//! including policy management, lifecycle automation, tier evaluation,
//! and integration testing.

#[cfg(test)]
mod test_suite {
    use super::super::{
        engine::DatasetAutomation,
        types::{
            AutomationPolicy, DatasetMetadata, LifecycleStage, PolicyConditions, PolicyPriority,
            TierRule,
        },
    };
    use crate::{
        config::{AiAutomationSettings, DatasetAutomationConfig},
        ZfsDatasetManager, ZfsPoolManager,
    };
    use nestgate_core::StorageTier;
    use std::sync::Arc;
    use std::time::SystemTime;
    use tokio::sync::RwLock;

    fn create_test_config() -> DatasetAutomationConfig {
        DatasetAutomationConfig {
            enabled: true,
            scan_interval_seconds: 300,
            learning_period_days: 7,
            default_policy: "balanced".to_string(),
            ai_settings: AiAutomationSettings {
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

        // Create a mock analyzer
        let analyzer = Arc::new(nestgate_automation::DatasetAnalyzer::new());

        let migration_engine = Arc::new(RwLock::new(crate::migration::MigrationEngine::new(
            migration_config,
            zfs_config.clone(),
            pool_manager.clone(),
            dataset_manager.clone(),
            analyzer,
        )));

        DatasetAutomation::new(
            pool_manager,
            dataset_manager,
            migration_engine,
            create_test_config(),
        )
        .await
        .expect("Failed to create automation engine in test")
    }

    #[tokio::test]
    async fn test_automation_engine_creation() {
        let automation = create_test_automation().await;
        let status = automation.get_automation_status().await;

        assert!(status.enabled);
        assert!(status.active_policies > 0); // Should have default policy
    }

    #[tokio::test]
    async fn test_policy_serialization() {
        let policy = AutomationPolicy {
            policy_id: "test_policy".to_string(),
            name: "Test Policy".to_string(),
            description: "A test policy".to_string(),
            enabled: true,
            priority: PolicyPriority::High,
            conditions: PolicyConditions {
                tier_rules: vec![TierRule {
                    condition: "tank/data/*".to_string(),
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

    #[tokio::test]
    async fn test_tier_evaluation_with_metadata() {
        let automation = create_test_automation().await;

        let metadata = DatasetMetadata {
            size_bytes: 1024 * 1024 * 100, // 100MB
            last_accessed: Some(SystemTime::now()),
            access_frequency: 50.0,
            file_types: vec!["txt".to_string(), "json".to_string()],
        };

        let result = automation
            .evaluate_tier_for_dataset("tank/active/config", &metadata)
            .await;
        assert!(result.is_ok());

        let tier = result.unwrap();
        // Should recommend hot or warm tier for frequently accessed config files
        assert!(matches!(tier, StorageTier::Hot | StorageTier::Warm));
    }

    #[tokio::test]
    async fn test_lifecycle_stage_display() {
        assert_eq!(LifecycleStage::New.to_string(), "New");
        assert_eq!(LifecycleStage::Active.to_string(), "Active");
        assert_eq!(LifecycleStage::Aging.to_string(), "Aging");
        assert_eq!(LifecycleStage::Archived.to_string(), "Archived");
        assert_eq!(LifecycleStage::Obsolete.to_string(), "Obsolete");
    }

    #[tokio::test]
    async fn test_automation_status() {
        let automation = create_test_automation().await;
        let status = automation.get_automation_status().await;

        assert!(status.enabled);
        assert_eq!(status.tracked_datasets, 0); // No datasets tracked initially
        assert_eq!(status.total_migrations_performed, 0);
    }
}
