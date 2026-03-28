// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **CANONICAL TEST CONFIGURATION**
//! Module definitions and exports.
// This module consolidates ALL test configuration variants across the NestGate ecosystem
//! into a single, authoritative configuration structure.
//! Module definitions and exports.
// **CONSOLIDATES**:
//! - 40+ scattered test configurations across all crates
//! - Unit test configs, integration test configs, E2E test configs
//! - Performance test configs, chaos test configs, load test configs
//! - Security test configs, mocking configs, environment test configs
//!
//! Module definitions and exports.
// **MODULAR STRUCTURE**:
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
//!
//! **⚠️ TEST ONLY**: This entire module is only available with `dev-stubs` feature

use serde::{Deserialize, Serialize};

// Import all test configuration modules

/// Chaos engineering and fault injection test configurations
pub mod chaos;

/// End-to-end integration test configurations
pub mod e2e;

/// Test environment setup and configuration
pub mod environment;

/// Global test settings and defaults
pub mod global;

/// Integration test configurations
pub mod integration;

/// Load and stress testing configurations
pub mod load;

/// Mock and stub configurations for testing
pub mod mocking;

/// Performance and benchmark test configurations
pub mod performance;

/// Security and authentication test configurations
pub mod security;

/// Unit test configurations and helpers
pub mod unit;

// Re-export all configuration types
pub use chaos::{
    ChaosExperimentConfig, ChaosTestConfig, FailureInjectionConfig, ResilienceTestConfig,
};
pub use e2e::{BrowserTestConfig, E2eTestConfig, ScenarioTestConfig, UserJourneyConfig};
pub use environment::{TestEnvironmentConfig, TestInfrastructureConfig, TestResourceConfig};
pub use global::{GlobalTestConfig, TestMetricsConfig, TestReportingConfig};
pub use integration::{
    ApiTestConfig, DatabaseTestConfig, IntegrationTestConfig, ServiceTestConfig,
};
pub use load::{LoadTestConfig, LoadTestScenario, LoadTestStep, RampUpConfig};
pub use mocking::{MockServiceConfig, MockingConfig, StubConfig, TestDoubleConfig};
pub use performance::{BenchmarkConfig, MetricsTestConfig, PerformanceTestConfig, ProfilingConfig};
pub use security::{
    ComplianceTestConfig, PenetrationTestConfig, SecurityTestConfig, VulnerabilityTestConfig,
};
pub use unit::{
    AssertionConfig, CoverageConfig, ParallelTestConfig, TestDataConfig, TestExecutionConfig,
    UnitTestConfig,
};

// ==================== CANONICAL TEST CONFIGURATION ====================

// **THE** canonical test configuration for the entire NestGate ecosystem
// This replaces ALL other test configuration variants
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Canonicaltestconfigs
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
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a configuration optimized for CI/CD environments
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
///
/// **Deprecated**: Use `CanonicalTestConfigs` directly instead
pub type TestConfig = CanonicalTestConfigs;

/// Backward compatibility alias for UnifiedTestConfig
///
/// **Deprecated**: Use `CanonicalTestConfigs` directly instead
pub type UnifiedTestConfig = CanonicalTestConfigs;

/// Backward compatibility alias for TestConfigs
///
/// **Deprecated**: Use `CanonicalTestConfigs` directly instead
pub type TestConfigs = CanonicalTestConfigs;
