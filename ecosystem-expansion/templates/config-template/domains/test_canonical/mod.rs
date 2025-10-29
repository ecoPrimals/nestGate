//! **CANONICAL TEST CONFIGURATION**
//!
//! This module consolidates ALL test configuration variants across the NestGate ecosystem
//! into a single, authoritative configuration structure.
//!
//! **CONSOLIDATES**:
//! - 40+ scattered test configurations across all crates
//! - Unit test configs, integration test configs, E2E test configs
//! - Performance test configs, chaos test configs, load test configs
//! - Security test configs, mocking configs, environment test configs
//!
//! **MODULAR STRUCTURE**:
//! - `unit`: Unit test configurations
//! - `integration`: Integration test configurations  
//! - `e2e`: End-to-end test configurations
//! - `performance`: Performance test configurations
//! - `load`: Load test configurations
//! - `chaos`: Chaos test configurations
//! - `security`: Security test configurations
//! - `mocking`: Mock configurations
//! - `environment`: Test environment configurations
//! - `global`: Global test settings

use serde::{Deserialize, Serialize};

// Import all test configuration modules
pub mod unit;
pub mod integration;
pub mod e2e;
pub mod performance;
pub mod load;
pub mod chaos;
pub mod security;
pub mod mocking;
pub mod environment;
pub mod global;

// Re-export all configuration types
pub use unit::{UnitTestConfig, TestExecutionConfig, CoverageConfig, AssertionConfig, TestDataConfig, ParallelTestConfig};
pub use integration::{IntegrationTestConfig, DatabaseTestConfig, ServiceTestConfig, ApiTestConfig};
pub use e2e::{E2eTestConfig, BrowserTestConfig, UserJourneyConfig, ScenarioTestConfig};
pub use performance::{PerformanceTestConfig, BenchmarkConfig, ProfilingConfig, MetricsTestConfig};
pub use load::{LoadTestConfig, LoadTestScenario, LoadTestStep, RampUpConfig};
pub use chaos::{ChaosTestConfig, ChaosExperimentConfig, FailureInjectionConfig, ResilienceTestConfig};
pub use security::{SecurityTestConfig, PenetrationTestConfig, VulnerabilityTestConfig, ComplianceTestConfig};
pub use mocking::{MockingConfig, MockServiceConfig, TestDoubleConfig, StubConfig};
pub use environment::{TestEnvironmentConfig, TestInfrastructureConfig, TestResourceConfig};
pub use global::{GlobalTestConfig, TestReportingConfig, TestMetricsConfig};

// ==================== CANONICAL TEST CONFIGURATION ====================

/// **THE** canonical test configuration for the entire NestGate ecosystem
/// This replaces ALL other test configuration variants
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct CanonicalTestConfigs {
    /// Unit test configuration
    pub unit: UnitTestConfig,
    
    /// Integration test configuration
    pub integration: IntegrationTestConfig,
    
    /// End-to-end test configuration
    pub e2e: E2eTestConfig,
    
    /// Performance test configuration
    pub performance: PerformanceTestConfig,
    
    /// Load test configuration
    pub load: LoadTestConfig,
    
    /// Chaos test configuration
    pub chaos: ChaosTestConfig,
    
    /// Security test configuration
    pub security: SecurityTestConfig,
    
    /// Mocking configuration
    pub mocking: MockingConfig,
    
    /// Test environment configuration
    pub environment: TestEnvironmentConfig,
    
    /// Global test settings
    pub global: GlobalTestConfig,
}


impl CanonicalTestConfigs {
    /// Create a new canonical test configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a configuration optimized for CI/CD environments
    pub fn ci_optimized() -> Self {
        Self {
            unit: UnitTestConfig::ci_optimized(),
            integration: IntegrationTestConfig::ci_optimized(),
            e2e: E2eTestConfig::ci_optimized(),
            performance: PerformanceTestConfig::ci_optimized(),
            load: LoadTestConfig::ci_optimized(),
            chaos: ChaosTestConfig::ci_optimized(),
            security: SecurityTestConfig::ci_optimized(),
            mocking: MockingConfig::ci_optimized(),
            environment: TestEnvironmentConfig::ci_optimized(),
            global: GlobalTestConfig::ci_optimized(),
        }
    }

    /// Create a configuration optimized for local development
    pub fn development_optimized() -> Self {
        Self {
            unit: UnitTestConfig::development_optimized(),
            integration: IntegrationTestConfig::development_optimized(),
            e2e: E2eTestConfig::development_optimized(),
            performance: PerformanceTestConfig::development_optimized(),
            load: LoadTestConfig::development_optimized(),
            chaos: ChaosTestConfig::development_optimized(),
            security: SecurityTestConfig::development_optimized(),
            mocking: MockingConfig::development_optimized(),
            environment: TestEnvironmentConfig::development_optimized(),
            global: GlobalTestConfig::development_optimized(),
        }
    }

    /// Merge with another configuration
    pub fn merge(mut self, other: Self) -> Self {
        self.unit = self.unit.merge(other.unit);
        self.integration = self.integration.merge(other.integration);
        self.e2e = self.e2e.merge(other.e2e);
        self.performance = self.performance.merge(other.performance);
        self.load = self.load.merge(other.load);
        self.chaos = self.chaos.merge(other.chaos);
        self.security = self.security.merge(other.security);
        self.mocking = self.mocking.merge(other.mocking);
        self.environment = self.environment.merge(other.environment);
        self.global = self.global.merge(other.global);
        self
    }
}

// ==================== BACKWARD COMPATIBILITY ALIASES ====================

/// Backward compatibility alias for existing TestConfig usage
pub type TestConfig = CanonicalTestConfigs;

/// Backward compatibility alias for UnifiedTestConfig
pub type UnifiedTestConfig = CanonicalTestConfigs;

/// Backward compatibility alias for TestConfigs
pub type TestConfigs = CanonicalTestConfigs; 