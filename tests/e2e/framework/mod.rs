//! # NestGate End-to-End Testing Framework
//!
//! **COMPREHENSIVE E2E TESTING** for validating complete system functionality
//!
//! This framework provides end-to-end testing capabilities that validate NestGate's
//! functionality across all components, from API endpoints to storage backends,
//! following canonical modernization principles.

// Module declarations
pub mod runner;
pub mod scenarios;
pub mod types;

// Re-export all public items for backward compatibility
pub use runner::E2ETestingFramework;
pub use scenarios::{
    ApiValidationRunner, ConfigValidationRunner, DataFlowRunner, LoadTestingRunner,
    ScenarioRunner, SecurityValidationRunner, ServiceIntegrationRunner, UserLifecycleRunner,
};
pub use types::{
    AttackScenario, ConfigVariation, E2EConfig, E2EEndpoints, E2EMetrics, E2EScenario,
    E2ETestResult, IntegrationDepth, TestStepResult,
};

// Re-export security tests module
mod security_tests;
pub use security_tests::SecurityTester; 