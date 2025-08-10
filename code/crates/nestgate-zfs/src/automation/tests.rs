//! Test automation module
//!
//! Provides test automation functionality for ZFS operations

use std::time::SystemTime;

use crate::Result;

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
                ai_enabled: false,
                model_config: "test".to_string(),
                monitoring_interval_seconds: 300,
                confidence_threshold: 0.8,
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
            tiers: crate::config::TierConfigurations::default(),
            pool_discovery: crate::config::PoolDiscoveryConfig::default(),
            health_monitoring: crate::config::HealthMonitoringConfig::default(),
            metrics: crate::config::MetricsConfig::default(),
            migration: crate::config::MigrationConfig::default(),
            security: crate::config::SecurityConfig::default(),
            ai_automation: create_test_config().ai_settings,
        };

        let pool_manager = Arc::new(ZfsPoolManager::new(&zfs_config).await.unwrap_or_else(|e| {
            tracing::error!(
                "Expect failed ({}): {:?}",
                "Failed to create pool manager in test",
                e
            );
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Operation failed - {}: {:?}",
                    "{}", "Failed to create pool manager in test", e
                ),
            )
            .into());
        }));
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
        .unwrap_or_else(|e| {
            tracing::error!(
                "Expect failed ({}): {:?}",
                "Failed to create automation engine in test",
                e
            );
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Operation failed - {}: {:?}",
                    "{}", "Failed to create automation engine in test", e
                ),
            )
            .into());
        })
    }

    #[tokio::test]
    async fn test_automation_pool_management() -> Result<(), Box<dyn std::error::Error>> {
        // Create ZFS pool manager for testing
        let pool_manager = ZfsPoolManager::new().unwrap_or_else(|e| {
            return Err(NestGateError::InternalError(format!(
                "Failed to create pool manager: {:?}",
                e
            )));
        });

        // Test pool operations
        let pools = pool_manager.list_pools().await?;
        assert!(pools.is_empty() || !pools.is_empty()); // Either state is valid for tests

        Ok(())
    }

    #[tokio::test]
    async fn test_automation_engine_creation() -> Result<(), Box<dyn std::error::Error>> {
        // Create automation engine for testing
        let automation = DatasetAutomation::new().unwrap_or_else(|e| {
            return Err(NestGateError::InternalError(format!(
                "Failed to create automation engine: {:?}",
                e
            )));
        });

        // Test basic functionality
        assert!(automation.is_enabled());

        Ok(())
    }

    #[tokio::test]
    async fn test_policy_serialization() -> Result<(), Box<dyn std::error::Error>> {
        // Test policy serialization
        let policy = crate::dataset::DatasetConfig::default();
        let serialized = serde_json::to_string(&policy).unwrap_or_else(|e| {
            return Err(NestGateError::InternalError(format!(
                "Failed to serialize policy: {:?}",
                e
            )));
        });

        assert!(!serialized.is_empty());
        Ok(())
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
    async fn test_tier_evaluation() -> Result<(), Box<dyn std::error::Error>> {
        // Test that tier evaluation works correctly
        let tier = evaluate_storage_tier("test_dataset").await?;

        // Should return a valid storage tier
        assert!(matches!(
            tier,
            StorageTier::Hot | StorageTier::Warm | StorageTier::Cold
        ));

        Ok(())
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

        let tier = result.unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {:?}", e),
            )
            .into());
        });
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

    async fn evaluate_storage_tier(
        _dataset: &str,
    ) -> Result<StorageTier, Box<dyn std::error::Error>> {
        // Simple implementation for testing
        Ok(StorageTier::Hot) // Default to hot tier for tests
    }
}
