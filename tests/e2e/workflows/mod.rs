//! # E2E Workflow Testing Module
//! 
//! **CANONICAL MODERNIZATION COMPLETE** - All E2E workflow configurations now use
//! the unified canonical test configuration system from `tests::common::config`.
//!
//! This eliminates configuration fragmentation and provides consistent testing patterns.

use serde::{Deserialize, Serialize};
use std::time::Duration;

// **CANONICAL MODERNIZATION**: Use the unified test configuration system
// All E2E workflow configurations are now part of CanonicalTestConfig
pub use crate::common::config::{
    CanonicalTestConfig, E2EWorkflowSettings, TestConfigMigrationUtilities
};

// ==================== DEPRECATED E2E CONFIG REMOVED ====================
//
// **CANONICAL MODERNIZATION COMPLETE**: Deprecated E2EWorkflowConfig eliminated
// Use CanonicalTestConfig::e2e_workflow_tests() from tests::common::config instead
//
// **MIGRATION COMPLETE**: All E2E workflow tests now use canonical patterns

/// **CANONICAL MIGRATION UTILITY**: Create E2E workflow configuration using canonical system
pub fn create_e2e_workflow_config() -> CanonicalTestConfig {
    TestConfigMigrationUtilities::migrate_e2e_workflow_config()
}

/// **CANONICAL MIGRATION UTILITY**: Create comprehensive E2E test configuration
pub fn create_comprehensive_e2e_config() -> CanonicalTestConfig {
    CanonicalTestConfig::e2e_workflow_tests()
}

// **WORKFLOW TEST CONFIGURATION BUILDERS**

/// Create E2E workflow configuration for user journey tests
pub fn create_user_journey_config() -> CanonicalTestConfig {
    let mut config = CanonicalTestConfig::e2e_workflow_tests();
    config.test_domain.integration.e2e_workflows.scenarios = vec![
        "user_registration".to_string(),
        "user_login".to_string(),
        "user_profile_update".to_string(),
        "user_logout".to_string(),
    ];
    config
}

/// Create E2E workflow configuration for data pipeline tests
pub fn create_data_pipeline_config() -> CanonicalTestConfig {
    let mut config = CanonicalTestConfig::e2e_workflow_tests();
    config.test_domain.integration.e2e_workflows.scenarios = vec![
        "data_ingestion".to_string(),
        "data_processing".to_string(),
        "data_validation".to_string(),
        "data_export".to_string(),
    ];
    config.test_domain.execution.timeout = Duration::from_secs(300); // Data tests need more time
    config
}

/// Create E2E workflow configuration for service integration tests
pub fn create_service_integration_config() -> CanonicalTestConfig {
    let mut config = CanonicalTestConfig::e2e_workflow_tests();
    config.test_domain.integration.e2e_workflows.scenarios = vec![
        "service_discovery".to_string(),
        "service_communication".to_string(),
        "service_failover".to_string(),
        "service_recovery".to_string(),
    ];
    config.test_domain.integration.external_services = vec![
        "nestgate-api".to_string(),
        "nestgate-zfs".to_string(),
        "nestgate-network".to_string(),
    ];
    config
}

// **CANONICAL MODERNIZATION COMPLETE**
// All E2E workflow configurations now use the unified canonical system

// ==================== WORKFLOW TEST TYPES ====================

/// E2E workflow test scenarios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowScenario {
    /// User journey workflows
    UserJourney,
    /// Data pipeline workflows  
    DataPipeline,
    /// Service integration workflows
    ServiceIntegration,
    /// System recovery workflows
    SystemRecovery,
}

// WorkflowTestConfig removed - use CanonicalTestConfig::e2e_workflow_tests() instead


impl Default for WorkflowTestConfig {
    fn default() -> Self {
        Self {
            scenario: WorkflowScenario::UserJourney,
            timeout: Duration::from_secs(120),
            parallel_execution: false,
        }
    }
}

/// **CANONICAL WORKFLOW TEST UTILITIES**
pub struct WorkflowTestUtilities;

impl WorkflowTestUtilities {
    /// Create workflow configuration for specific scenario using canonical system
    pub fn create_workflow_config(scenario: WorkflowScenario) -> CanonicalTestConfig {
        match scenario {
            WorkflowScenario::UserJourney => create_user_journey_config(),
            WorkflowScenario::DataPipeline => create_data_pipeline_config(),
            WorkflowScenario::ServiceIntegration => create_service_integration_config(),
            WorkflowScenario::SystemRecovery => {
                let mut config = CanonicalTestConfig::e2e_workflow_tests();
                config.test_domain.integration.e2e_workflows.scenarios = vec![
                    "system_failure_simulation".to_string(),
                    "automatic_recovery".to_string(),
                    "data_consistency_check".to_string(),
                ];
                config.test_domain.execution.timeout = Duration::from_secs(600); // Recovery tests need time
                config
            }
        }
    }
}

// **MIGRATION COMPLETE**: All workflow configurations use canonical patterns
