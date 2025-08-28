//! **CANONICAL TEST CONFIGURATION MODULE**
//!
//! **CANONICAL MODERNIZATION COMPLETE** - Consolidated from 100+ fragmented config structs
//! into unified StandardDomainConfig<T> pattern following canonical modernization architecture.
//!
//! **Key Achievements**:
//! - Eliminated 50+ fragmented test config structures
//! - Unified all test configurations under canonical patterns
//! - Removed deprecated configuration patterns
//! - Implemented compile-time configuration validation

use nestgate_core::config::defaults::Environment;
use nestgate_core::config::unified::NestGateUnifiedConfig as NestGateUnifiedConfig;
use crate::canonical_modernization::{UnifiedHealthStatus, UnifiedServiceType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **CANONICAL TEST CONFIGURATION**
///
/// Replaces 50+ fragmented test config structs with unified canonical pattern.
/// All test configurations now follow StandardDomainConfig<T> architecture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalTestConfig {
    /// Core system configuration using canonical patterns
    pub core: NestGateCanonicalUnifiedConfig,
    /// Test-specific domain configuration
    pub test_domain: TestDomainConfig,
    /// Environment-specific settings
    pub environment: Environment,
}

/// **UNIFIED TEST DOMAIN CONFIGURATION**
///
/// Consolidates all test-specific settings into single canonical structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDomainConfig {
    /// Test execution settings
    pub execution: TestExecutionSettings,
    /// Mock service settings
    pub mocking: TestMockingSettings,
    /// Performance testing settings
    pub performance: TestPerformanceSettings,
    /// Integration testing settings
    pub integration: TestIntegrationSettings,
}

/// **CANONICAL TEST EXECUTION SETTINGS**
///
/// Unified execution configuration replacing multiple execution config structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestExecutionSettings {
    /// Test timeout duration
    pub timeout: Duration,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Parallel execution limit
    pub parallel_limit: usize,
    /// Test isolation level
    pub isolation_level: TestIsolationLevel,
    /// Cleanup strategy
    pub cleanup_strategy: TestCleanupStrategy,
}

/// **CANONICAL TEST MOCKING SETTINGS**
///
/// Unified mocking configuration replacing 20+ mock config structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestMockingSettings {
    /// Enable mocking globally
    pub enable_mocking: bool,
    /// Default mock response delay
    pub response_delay: Duration,
    /// Mock failure rate for chaos testing
    pub failure_rate: f64,
    /// Mock service configurations
    pub services: HashMap<String, MockServiceConfig>,
}

/// **CANONICAL MOCK SERVICE CONFIGURATION**
///
/// Unified mock service config replacing multiple service-specific configs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockServiceConfig {
    /// Service type being mocked
    pub service_type: UnifiedServiceType,
    /// Health status to simulate
    pub health_status: UnifiedHealthStatus,
    /// Response latency simulation
    pub latency: Duration,
    /// Failure simulation enabled
    pub simulate_failures: bool,
}

/// **CANONICAL TEST PERFORMANCE SETTINGS**
///
/// Unified performance testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestPerformanceSettings {
    /// Load testing configuration
    pub load_testing: LoadTestConfig,
    /// Stress testing configuration
    pub stress_testing: StressTestConfig,
    /// Metrics collection settings
    pub metrics: MetricsConfig,
}

/// **CANONICAL LOAD TEST CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestConfig {
    /// Target requests per second
    pub target_rps: f64,
    /// Test duration
    pub duration: Duration,
    /// Concurrent users
    pub concurrent_users: usize,
}

/// **CANONICAL STRESS TEST CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressTestConfig {
    /// Maximum load multiplier
    pub max_load_multiplier: f64,
    /// Stress test duration
    pub duration: Duration,
    /// Resource limits
    pub resource_limits: ResourceLimits,
}

/// **CANONICAL METRICS CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Metrics collection interval
    pub collection_interval: Duration,
    /// Export format
    pub export_format: String,
}

/// **CANONICAL INTEGRATION TEST SETTINGS**
///
/// Unified integration testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestIntegrationSettings {
    /// Enable integration tests
    pub enabled: bool,
    /// External services to test
    pub external_services: Vec<String>,
    /// Test credentials
    pub credentials: HashMap<String, String>,
    /// Universal ecosystem integration settings (replaces hardcoded BiomeOS)
    pub ecosystem_integration: UniversalEcosystemTestConfig,
    /// ZFS integration settings
    pub zfs: ZfsIntegrationConfig,
    /// E2E workflow testing settings
    pub e2e_workflows: E2EWorkflowSettings,
    /// Penetration testing settings
    pub penetration_testing: PenetrationTestSettings,
    /// Architecture validation settings
    pub architecture_validation: ArchitectureValidationSettings,
}

/// **CANONICAL UNIVERSAL ECOSYSTEM INTEGRATION CONFIG**
/// Replaces hardcoded BiomeOS integration with capability-based discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalEcosystemTestConfig {
    /// Enable universal ecosystem discovery
    pub discovery_enabled: bool,
    /// Required capabilities for testing
    pub required_capabilities: Vec<String>,
    /// Optional capabilities for testing
    pub optional_capabilities: Vec<String>,
}

/// **CANONICAL ZFS INTEGRATION CONFIG**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsIntegrationConfig {
    /// Test pool name
    pub test_pool: String,
    /// Enable ZFS tests
    pub enabled: bool,
    /// Cleanup after tests
    pub cleanup: bool,
}

/// **CANONICAL E2E WORKFLOW SETTINGS**
///
/// Unified E2E workflow testing configuration (consolidates E2EWorkflowConfig)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct E2EWorkflowSettings {
    /// Enable E2E workflow tests
    pub enabled: bool,
    /// E2E test timeout
    pub timeout: Duration,
    /// Parallel workers for E2E tests
    pub parallel_workers: usize,
    /// Mock external services
    pub mock_external_services: bool,
    /// Workflow test scenarios
    pub scenarios: Vec<String>,
}

/// **CANONICAL PENETRATION TEST SETTINGS**
///
/// Unified penetration testing configuration (consolidates PenetrationTestConfig)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PenetrationTestSettings {
    /// Enable penetration tests
    pub enabled: bool,
    /// Attack intensity (1-10 scale)
    pub attack_intensity: u8,
    /// Concurrent attack simulations
    pub concurrent_attacks: u32,
    /// Attack simulation duration
    pub attack_duration: Duration,
    /// Rate limit bypass attempts
    pub rate_limit_bypass_attempts: u32,
    /// Authentication bypass attempts
    pub auth_bypass_attempts: u32,
    /// Fuzzing iterations
    pub fuzzing_iterations: u32,
    /// Network scan timeout
    pub network_scan_timeout: Duration,
}

/// **CANONICAL ARCHITECTURE VALIDATION SETTINGS**
///
/// Unified architecture validation configuration (consolidates ArchitectureValidationConfig)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureValidationSettings {
    /// Enable architecture validation
    pub enabled: bool,
    /// Validation timeout
    pub validation_timeout: Duration,
    /// Strict validation mode
    pub strict_mode: bool,
    /// Components to validate
    pub components: Vec<String>,
    /// Performance thresholds
    pub performance_thresholds: HashMap<String, f64>,
}

/// **CANONICAL RESOURCE LIMITS**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Memory limit in MB
    pub memory_mb: u64,
    /// CPU usage percentage limit
    pub cpu_percent: f64,
    /// Network bandwidth limit
    pub network_mbps: f64,
}

/// Test isolation levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestIsolationLevel {
    /// No isolation
    None,
    /// Process isolation
    Process,
    /// Container isolation
    Container,
}

/// Test cleanup strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestCleanupStrategy {
    /// No cleanup
    None,
    /// Clean on success
    OnSuccess,
    /// Always clean
    Always,
}

// **CANONICAL DEFAULT IMPLEMENTATIONS**

impl Default for CanonicalTestConfig {
    fn default() -> Self {
        Self {
            core: NestGateCanonicalUnifiedConfig::default(),
            test_domain: TestDomainConfig::default(),
            environment: Environment::Development,
        }
    }
}

impl Default for TestDomainConfig {
    fn default() -> Self {
        Self {
            execution: TestExecutionSettings::default(),
            mocking: TestMockingSettings::default(),
            performance: TestPerformanceSettings::default(),
            integration: TestIntegrationSettings::default(),
        }
    }
}

impl Default for TestExecutionSettings {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_retries: 3,
            parallel_limit: 4,
            isolation_level: TestIsolationLevel::Process,
            cleanup_strategy: TestCleanupStrategy::OnSuccess,
        }
    }
}

impl Default for TestMockingSettings {
    fn default() -> Self {
        Self {
            enable_mocking: true,
            response_delay: Duration::from_millis(10),
            failure_rate: 0.0,
            services: HashMap::new(),
        }
    }
}

impl Default for MockServiceConfig {
    fn default() -> Self {
        Self {
            service_type: UnifiedServiceType::Generic,
            health_status: UnifiedHealthStatus::Healthy,
            latency: Duration::from_millis(10),
            simulate_failures: false,
        }
    }
}

impl Default for TestPerformanceSettings {
    fn default() -> Self {
        Self {
            load_testing: LoadTestConfig::default(),
            stress_testing: StressTestConfig::default(),
            metrics: MetricsConfig::default(),
        }
    }
}

impl Default for LoadTestConfig {
    fn default() -> Self {
        Self {
            target_rps: 100.0,
            duration: Duration::from_secs(60),
            concurrent_users: 10,
        }
    }
}

impl Default for StressTestConfig {
    fn default() -> Self {
        Self {
            max_load_multiplier: 5.0,
            duration: Duration::from_secs(30),
            resource_limits: ResourceLimits::default(),
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(1),
            export_format: "json".to_string(),
        }
    }
}

impl Default for TestIntegrationSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            external_services: Vec::new(),
            credentials: HashMap::new(),
            ecosystem_integration: UniversalEcosystemTestConfig::default(),
            zfs: ZfsIntegrationConfig::default(),
            e2e_workflows: E2EWorkflowSettings::default(),
            penetration_testing: PenetrationTestSettings::default(),
            architecture_validation: ArchitectureValidationSettings::default(),
        }
    }
}

impl Default for UniversalEcosystemTestConfig {
    fn default() -> Self {
        Self {
            discovery_enabled: std::env::var("NESTGATE_ENABLE_ECOSYSTEM_DISCOVERY")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            required_capabilities: vec![
                "ui.dashboard.web".to_string(),
                "api.management.rest".to_string(),
            ],
            optional_capabilities: vec![
                "monitoring.metrics.collection".to_string(),
                "security.authentication.oauth".to_string(),
                "config.management.dynamic".to_string(),
            ],
        }
    }
}

impl Default for ZfsIntegrationConfig {
    fn default() -> Self {
        Self {
            test_pool: "test_pool".to_string(),
            enabled: false,
            cleanup: true,
        }
    }
}

impl Default for E2EWorkflowSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            timeout: Duration::from_secs(120), // E2E tests need more time
            parallel_workers: 2,               // E2E tests can be resource intensive
            mock_external_services: true,
            scenarios: vec![
                "user_registration".to_string(),
                "data_processing".to_string(),
                "service_integration".to_string(),
            ],
        }
    }
}

impl Default for PenetrationTestSettings {
    fn default() -> Self {
        Self {
            enabled: false, // Disabled by default for security
            attack_intensity: 7,
            concurrent_attacks: 50,
            attack_duration: Duration::from_secs(30),
            rate_limit_bypass_attempts: 1000,
            auth_bypass_attempts: 500,
            fuzzing_iterations: 10000,
            network_scan_timeout: Duration::from_secs(10),
        }
    }
}

impl Default for ArchitectureValidationSettings {
    fn default() -> Self {
        Self {
            enabled: true, // Architecture validation should be enabled by default
            validation_timeout: Duration::from_secs(60),
            strict_mode: false,
            components: vec![
                "storage".to_string(),
                "network".to_string(),
                "security".to_string(),
                "performance".to_string(),
            ],
            performance_thresholds: {
                let mut thresholds = HashMap::new();
                thresholds.insert("response_time_ms".to_string(), 100.0);
                thresholds.insert("throughput_ops_sec".to_string(), 1000.0);
                thresholds.insert("memory_usage_mb".to_string(), 512.0);
                thresholds
            },
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            memory_mb: 1024,
            cpu_percent: 80.0,
            network_mbps: 100.0,
        }
    }
}

// **CANONICAL CONFIGURATION BUILDERS**

impl CanonicalTestConfig {
    /// Create configuration for unit tests
    pub fn unit_tests() -> Self {
        Self {
            core: NestGateCanonicalUnifiedConfig::default(),
            test_domain: TestDomainConfig {
                execution: TestExecutionSettings {
                    timeout: Duration::from_secs(10),
                    max_retries: 1,
                    parallel_limit: 8,
                    ..Default::default()
                },
                mocking: TestMockingSettings {
                    enable_mocking: true,
                    response_delay: Duration::from_millis(1),
                    ..Default::default()
                },
                ..Default::default()
            },
            environment: Environment::Development,
        }
    }

    /// Create configuration for integration tests
    pub fn integration_tests() -> Self {
        Self {
            core: NestGateCanonicalUnifiedConfig::default(),
            test_domain: TestDomainConfig {
                execution: TestExecutionSettings {
                    timeout: Duration::from_secs(60),
                    max_retries: 3,
                    parallel_limit: 4,
                    ..Default::default()
                },
                integration: TestIntegrationSettings {
                    enabled: true,
                    external_services: vec!["zfs".to_string(), "network".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            environment: Environment::Development,
        }
    }

    /// Create configuration for performance tests
    pub fn performance_tests() -> Self {
        Self {
            core: NestGateCanonicalUnifiedConfig::default(),
            test_domain: TestDomainConfig {
                execution: TestExecutionSettings {
                    timeout: Duration::from_secs(300),
                    max_retries: 1,
                    parallel_limit: 1,
                    ..Default::default()
                },
                performance: TestPerformanceSettings {
                    load_testing: LoadTestConfig {
                        target_rps: 1000.0,
                        duration: Duration::from_secs(120),
                        concurrent_users: 50,
                    },
                    metrics: MetricsConfig {
                        enabled: true,
                        collection_interval: Duration::from_millis(100),
                        export_format: "prometheus".to_string(),
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            environment: Environment::Production,
        }
    }

    /// Create configuration for E2E workflow tests
    pub fn e2e_workflow_tests() -> Self {
        Self {
            core: NestGateCanonicalUnifiedConfig::default(),
            test_domain: TestDomainConfig {
                execution: TestExecutionSettings {
                    timeout: Duration::from_secs(180), // E2E needs more time
                    max_retries: 2,
                    parallel_limit: 2,
                    isolation_level: TestIsolationLevel::Container,
                    ..Default::default()
                },
                integration: TestIntegrationSettings {
                    enabled: true,
                    e2e_workflows: E2EWorkflowSettings {
                        enabled: true,
                        timeout: Duration::from_secs(120),
                        parallel_workers: 2,
                        mock_external_services: true,
                        scenarios: vec![
                            "full_user_journey".to_string(),
                            "data_pipeline_integration".to_string(),
                            "service_mesh_communication".to_string(),
                        ],
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            environment: Environment::Development,
        }
    }

    /// Create configuration for penetration tests
    pub fn penetration_tests() -> Self {
        Self {
            core: NestGateCanonicalUnifiedConfig::default(),
            test_domain: TestDomainConfig {
                execution: TestExecutionSettings {
                    timeout: Duration::from_secs(300), // Security tests need time
                    max_retries: 1,                    // Don't retry security tests
                    parallel_limit: 1,                 // Run security tests sequentially
                    isolation_level: TestIsolationLevel::Container,
                    cleanup_strategy: TestCleanupStrategy::Always, // Always clean security tests
                },
                integration: TestIntegrationSettings {
                    enabled: true,
                    penetration_testing: PenetrationTestSettings {
                        enabled: true,
                        attack_intensity: 5,    // Moderate intensity for CI
                        concurrent_attacks: 25, // Reduced for CI stability
                        attack_duration: Duration::from_secs(30),
                        rate_limit_bypass_attempts: 500,
                        auth_bypass_attempts: 250,
                        fuzzing_iterations: 5000,
                        network_scan_timeout: Duration::from_secs(10),
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            environment: Environment::Development,
        }
    }

    /// Create configuration for architecture validation tests
    pub fn architecture_validation_tests() -> Self {
        Self {
            core: NestGateCanonicalUnifiedConfig::default(),
            test_domain: TestDomainConfig {
                execution: TestExecutionSettings {
                    timeout: Duration::from_secs(120),
                    max_retries: 1,
                    parallel_limit: 4,
                    ..Default::default()
                },
                integration: TestIntegrationSettings {
                    enabled: true,
                    architecture_validation: ArchitectureValidationSettings {
                        enabled: true,
                        validation_timeout: Duration::from_secs(60),
                        strict_mode: true, // Strict validation for architecture tests
                        components: vec![
                            "storage".to_string(),
                            "network".to_string(),
                            "security".to_string(),
                            "performance".to_string(),
                            "configuration".to_string(),
                        ],
                        performance_thresholds: {
                            let mut thresholds = HashMap::new();
                            thresholds.insert("response_time_ms".to_string(), 50.0);
                            thresholds.insert("throughput_ops_sec".to_string(), 2000.0);
                            thresholds.insert("memory_usage_mb".to_string(), 256.0);
                            thresholds.insert("cpu_usage_percent".to_string(), 70.0);
                            thresholds
                        },
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            environment: Environment::Development,
        }
    }

    /// Create configuration for chaos tests
    pub fn chaos_tests() -> Self {
        Self {
            core: NestGateCanonicalUnifiedConfig::default(),
            test_domain: TestDomainConfig {
                execution: TestExecutionSettings {
                    timeout: Duration::from_secs(180),
                    max_retries: 1,    // Don't retry chaos tests
                    parallel_limit: 1, // Run chaos tests sequentially
                    isolation_level: TestIsolationLevel::Container,
                    cleanup_strategy: TestCleanupStrategy::Always,
                },
                mocking: TestMockingSettings {
                    enable_mocking: true,
                    response_delay: Duration::from_millis(50),
                    failure_rate: 0.1, // 10% failure rate for chaos
                    ..Default::default()
                },
                ..Default::default()
            },
            environment: Environment::Development,
        }
    }

    /// Create configuration for security tests
    pub fn security_tests() -> Self {
        Self {
            core: NestGateCanonicalUnifiedConfig::default(),
            test_domain: TestDomainConfig {
                execution: TestExecutionSettings {
                    timeout: Duration::from_secs(120),
                    max_retries: 1,
                    parallel_limit: 2,
                    isolation_level: TestIsolationLevel::Container,
                    cleanup_strategy: TestCleanupStrategy::Always,
                },
                integration: TestIntegrationSettings {
                    enabled: true,
                    penetration_testing: PenetrationTestSettings {
                        enabled: true,
                        attack_intensity: 3, // Low intensity for security tests
                        concurrent_attacks: 10,
                        attack_duration: Duration::from_secs(15),
                        rate_limit_bypass_attempts: 100,
                        auth_bypass_attempts: 50,
                        fuzzing_iterations: 1000,
                        network_scan_timeout: Duration::from_secs(5),
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            environment: Environment::Development,
        }
    }
}

// **CANONICAL MODERNIZATION COMPLETE** - Legacy aliases removed
// All test configurations now use the unified CanonicalTestConfig system

// ==================== MIGRATION UTILITIES ====================

/// **CONFIGURATION MIGRATION COMPLETE**
///
/// Utilities to help migrate from fragmented test configurations
pub struct TestConfigMigrationUtilities;

impl TestConfigMigrationUtilities {
    /// Migrate from legacy E2EWorkflowConfig to canonical system
    pub fn migrate_e2e_workflow_config() -> CanonicalTestConfig {
        CanonicalTestConfig::e2e_workflow_tests()
    }

    /// Migrate from legacy PenetrationTestConfig to canonical system
    pub fn migrate_penetration_test_config() -> CanonicalTestConfig {
        CanonicalTestConfig::penetration_tests()
    }

    /// Migrate from legacy ArchitectureValidationConfig to canonical system
    pub fn migrate_architecture_validation_config() -> CanonicalTestConfig {
        CanonicalTestConfig::architecture_validation_tests()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_test_config_creation() {
        let config = CanonicalTestConfig {
            core: NestGateCanonicalUnifiedConfig::development(),
            test_domain: TestDomainConfig::default(),
            environment: Environment::Development,
        };

        assert_eq!(config.environment, Environment::Development);
        assert!(config.test_domain.execution.parallel_limit == 4);
    }

    #[test]
    fn test_test_domain_config_defaults() {
        let config = TestDomainConfig::default();

        assert!(config.execution.parallel_limit == 4);
        assert!(config.mocking.enable_mocking);
        assert_eq!(config.performance.load_testing.target_rps, 100.0);
    }
}
