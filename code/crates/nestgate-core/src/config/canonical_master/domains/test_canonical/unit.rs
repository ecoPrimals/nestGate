//! Unit test configuration module
//! Provides unified unit testing configuration and settings.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

// ==================== UNIT TEST CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnitTestConfig {
    /// Test execution settings
    pub execution: TestExecutionConfig,

    /// Coverage settings
    pub coverage: CoverageConfig,

    /// Assertion settings
    pub assertions: AssertionConfig,

    /// Test data settings
    pub test_data: TestDataConfig,

    /// Parallel execution
    pub parallel: ParallelTestConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestExecutionConfig {
    /// Test timeout
    pub timeout: Duration,

    /// Retry failed tests
    pub retry_failed: bool,

    /// Max retry attempts
    pub max_retries: u32,

    /// Fail fast
    pub fail_fast: bool,

    /// Test isolation
    pub isolation: TestIsolation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestIsolation {
    None,
    Process,
    Container,
    Vm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageConfig {
    /// Enable coverage collection
    pub enabled: bool,

    /// Coverage threshold
    pub threshold: f64,

    /// Coverage types
    pub types: Vec<CoverageType>,

    /// Coverage output format
    pub output_format: CoverageFormat,

    /// Coverage report path
    pub report_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoverageType {
    Line,
    Branch,
    Function,
    Statement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoverageFormat {
    Html,
    Xml,
    Json,
    Lcov,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssertionConfig {
    /// Assertion style
    pub style: AssertionStyle,

    /// Custom assertions
    pub custom_assertions: Vec<CustomAssertion>,

    /// Assertion timeout
    pub timeout: Duration,

    /// Detailed assertion messages
    pub detailed_messages: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssertionStyle {
    Traditional,
    Fluent,
    Bdd,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAssertion {
    /// Assertion name
    pub name: String,

    /// Assertion implementation
    pub implementation: String,

    /// Assertion description
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDataConfig {
    /// Test data directory
    pub data_dir: PathBuf,

    /// Test fixtures
    pub fixtures: Vec<TestFixture>,

    /// Data generation settings
    pub generation: DataGenerationConfig,

    /// Test database settings
    pub database: TestDatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestFixture {
    /// Fixture name
    pub name: String,

    /// Fixture path
    pub path: PathBuf,

    /// Fixture type
    pub fixture_type: FixtureType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FixtureType {
    Json,
    Yaml,
    Toml,
    Sql,
    Binary,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataGenerationConfig {
    /// Enable data generation
    pub enabled: bool,

    /// Generation strategies
    pub strategies: Vec<GenerationStrategy>,

    /// Seed for reproducible data
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationStrategy {
    /// Strategy name
    pub name: String,

    /// Strategy type
    pub strategy_type: StrategyType,

    /// Strategy parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrategyType {
    Random,
    Sequential,
    Pattern,
    Template,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDatabaseConfig {
    /// Database URL
    pub url: String,

    /// Migration settings
    pub migrations: MigrationConfig,

    /// Cleanup settings
    pub cleanup: CleanupConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationConfig {
    /// Enable migrations
    pub enabled: bool,

    /// Migration path
    pub path: PathBuf,

    /// Auto rollback
    pub auto_rollback: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupConfig {
    /// Cleanup strategy
    pub strategy: CleanupStrategy,

    /// Cleanup after each test
    pub after_each: bool,

    /// Cleanup after all tests
    pub after_all: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CleanupStrategy {
    Truncate,
    Delete,
    Rollback,
    Recreate,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelTestConfig {
    /// Enable parallel execution
    pub enabled: bool,

    /// Number of parallel threads
    pub threads: Option<usize>,

    /// Thread pool settings
    pub thread_pool: ThreadPoolConfig,

    /// Resource sharing settings
    pub resource_sharing: ResourceSharingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadPoolConfig {
    /// Core pool size
    pub core_size: usize,

    /// Maximum pool size
    pub max_size: usize,

    /// Keep alive time
    pub keep_alive: Duration,

    /// Queue capacity
    pub queue_capacity: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceSharingConfig {
    /// Shared resources
    pub shared_resources: Vec<SharedResource>,

    /// Resource locks
    pub locks: Vec<ResourceLock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedResource {
    /// Resource name
    pub name: String,

    /// Resource type
    pub resource_type: ResourceType,

    /// Access mode
    pub access_mode: AccessMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    Database,
    File,
    Network,
    Memory,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessMode {
    ReadOnly,
    WriteOnly,
    ReadWrite,
    Exclusive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLock {
    /// Lock name
    pub name: String,

    /// Lock type
    pub lock_type: LockType,

    /// Lock timeout
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LockType {
    Mutex,
    RwLock,
    Semaphore(usize),
    Custom(String),
}

// ==================== DEFAULT IMPLEMENTATIONS ====================

impl Default for TestExecutionConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            retry_failed: false,
            max_retries: 0,
            fail_fast: false,
            isolation: TestIsolation::None,
        }
    }
}

impl Default for CoverageConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            threshold: 80.0,
            types: vec![CoverageType::Line, CoverageType::Branch],
            output_format: CoverageFormat::Html,
            report_path: PathBuf::from("target/coverage"),
        }
    }
}

impl Default for AssertionConfig {
    fn default() -> Self {
        Self {
            style: AssertionStyle::Traditional,
            custom_assertions: Vec::new(),
            timeout: Duration::from_secs(5),
            detailed_messages: true,
        }
    }
}

impl Default for TestDataConfig {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("tests/data"),
            fixtures: Vec::new(),
            generation: DataGenerationConfig::default(),
            database: TestDatabaseConfig::default(),
        }
    }
}

impl Default for TestDatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite://memory:".to_string(),
            migrations: MigrationConfig::default(),
            cleanup: CleanupConfig::default(),
        }
    }
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            path: PathBuf::from("migrations"),
            auto_rollback: true,
        }
    }
}

impl Default for CleanupConfig {
    fn default() -> Self {
        Self {
            strategy: CleanupStrategy::Rollback,
            after_each: true,
            after_all: true,
        }
    }
}

impl Default for ParallelTestConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            threads: None, // Auto-detect
            thread_pool: ThreadPoolConfig::default(),
            resource_sharing: ResourceSharingConfig::default(),
        }
    }
}

impl Default for ThreadPoolConfig {
    fn default() -> Self {
        Self {
            core_size: num_cpus::get(),
            max_size: num_cpus::get() * 2,
            keep_alive: Duration::from_secs(60),
            queue_capacity: 1000,
        }
    }
}

// ==================== BUILDER METHODS ====================

impl UnitTestConfig {
    /// Create a configuration optimized for CI/CD environments
    #[must_use]
    pub const fn ci_optimized() -> Self {
        Self {
            execution: TestExecutionConfig {
                timeout: Duration::from_secs(60),
                retry_failed: true,
                max_retries: 2,
                fail_fast: true,
                isolation: TestIsolation::Process,
            },
            coverage: CoverageConfig {
                enabled: true,
                threshold: 90.0,
                types: vec![
                    CoverageType::Line,
                    CoverageType::Branch,
                    CoverageType::Function,
                ],
                output_format: CoverageFormat::Xml,
                report_path: PathBuf::from("target/coverage-ci"),
            },
            parallel: ParallelTestConfig {
                enabled: true,
                threads: Some(4), // Limited for CI resources
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Create a configuration optimized for local development
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self {
            execution: TestExecutionConfig {
                timeout: Duration::from_secs(10),
                retry_failed: false,
                max_retries: 0,
                fail_fast: true,
                isolation: TestIsolation::None,
            },
            coverage: CoverageConfig {
                enabled: true,
                threshold: 70.0,
                types: vec![CoverageType::Line],
                output_format: CoverageFormat::Html,
                report_path: PathBuf::from("target/coverage-dev"),
            },
            parallel: ParallelTestConfig {
                enabled: true,
                threads: None, // Use all available cores
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Merge with another configuration
    #[must_use]
    pub const fn merge(self, other: Self) -> Self {
        // Simple merge - other takes precedence
        // In a real implementation, you might want more sophisticated merging
        other
    }
}
