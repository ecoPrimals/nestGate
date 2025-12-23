//! Unit test configuration module
//! Provides unified unit testing configuration and settings.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

// ==================== UNIT TEST CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for UnitTest
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
/// Configuration for TestExecution
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
/// Testisolation
pub enum TestIsolation {
    /// None
    None,
    /// Process
    Process,
    /// Container
    Container,
    /// Vm
    Vm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Coverage
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
/// Types of Coverage
pub enum CoverageType {
    /// Line
    Line,
    /// Branch
    Branch,
    /// Function
    Function,
    /// Statement
    Statement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Coverageformat
pub enum CoverageFormat {
    /// Html
    Html,
    /// Xml
    Xml,
    /// Json
    Json,
    /// Lcov
    Lcov,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Assertion
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
/// Assertionstyle
pub enum AssertionStyle {
    /// Traditional
    Traditional,
    /// Fluent
    Fluent,
    /// Bdd
    Bdd,
    /// Custom assertion style with arbitrary descriptor
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Customassertion
pub struct CustomAssertion {
    /// Assertion name
    pub name: String,

    /// Assertion implementation
    pub implementation: String,

    /// Assertion description
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for TestData
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
/// Testfixture
pub struct TestFixture {
    /// Fixture name
    pub name: String,

    /// Fixture path
    pub path: PathBuf,

    /// Fixture type
    pub fixture_type: FixtureType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Fixture
pub enum FixtureType {
    /// Json
    Json,
    /// Yaml
    Yaml,
    /// Toml
    Toml,
    /// Sql
    Sql,
    /// Binary
    Binary,
    /// Custom fixture type with arbitrary descriptor
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for DataGeneration
pub struct DataGenerationConfig {
    /// Enable data generation
    pub enabled: bool,

    /// Generation strategies
    pub strategies: Vec<GenerationStrategy>,

    /// Seed for reproducible data
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Generationstrategy
pub struct GenerationStrategy {
    /// Strategy name
    pub name: String,

    /// Strategy type
    pub strategy_type: StrategyType,

    /// Strategy parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Strategy
pub enum StrategyType {
    /// Random
    Random,
    /// Sequential
    Sequential,
    /// Pattern
    Pattern,
    /// Template
    Template,
    /// Custom mock type with arbitrary descriptor
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for TestDatabase
pub struct TestDatabaseConfig {
    /// Database URL
    pub url: String,

    /// Migration settings
    pub migrations: MigrationConfig,

    /// Cleanup settings
    pub cleanup: CleanupConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Migration
pub struct MigrationConfig {
    /// Enable migrations
    pub enabled: bool,

    /// Migration path
    pub path: PathBuf,

    /// Auto rollback
    pub auto_rollback: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Cleanup
pub struct CleanupConfig {
    /// Cleanup strategy
    pub strategy: CleanupStrategy,

    /// Cleanup after each test
    pub after_each: bool,

    /// Cleanup after all tests
    pub after_all: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cleanupstrategy
pub enum CleanupStrategy {
    /// Truncate
    Truncate,
    /// Delete
    Delete,
    /// Rollback
    Rollback,
    /// Recreate
    Recreate,
    /// Custom cleanup strategy with arbitrary descriptor
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ParallelTest
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
/// Configuration for ThreadPool
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
/// Configuration for ResourceSharing
pub struct ResourceSharingConfig {
    /// Shared resources
    pub shared_resources: Vec<SharedResource>,

    /// Resource locks
    pub locks: Vec<ResourceLock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Sharedresource
pub struct SharedResource {
    /// Resource name
    pub name: String,

    /// Resource type
    pub resource_type: ResourceType,

    /// Access mode
    pub access_mode: AccessMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Resource
pub enum ResourceType {
    /// Database
    Database,
    /// File
    File,
    /// Network
    Network,
    /// Memory
    Memory,
    /// Custom database type with arbitrary descriptor
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Accessmode
pub enum AccessMode {
    /// Readonly
    ReadOnly,
    /// Writeonly
    WriteOnly,
    /// Readwrite
    ReadWrite,
    /// Exclusive
    Exclusive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Resourcelock
pub struct ResourceLock {
    /// Lock name
    pub name: String,

    /// Lock type
    pub lock_type: LockType,

    /// Lock timeout
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Lock
pub enum LockType {
    /// Mutex
    Mutex,
    /// Rwlock
    RwLock,
    /// Semaphore with maximum count
    Semaphore(usize),
    /// Custom isolation type with arbitrary descriptor
    Custom(String),
}

// ==================== DEFAULT IMPLEMENTATIONS ====================

impl Default for TestExecutionConfig {
    /// Returns the default instance
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
    /// Returns the default instance
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
    /// Returns the default instance
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
    /// Returns the default instance
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
    /// Returns the default instance
    fn default() -> Self {
        Self {
            url: "sqlite://memory:".to_string(),
            migrations: MigrationConfig::default(),
            cleanup: CleanupConfig::default(),
        }
    }
}

impl Default for MigrationConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            path: PathBuf::from("migrations"),
            auto_rollback: true,
        }
    }
}

impl Default for CleanupConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            strategy: CleanupStrategy::Rollback,
            after_each: true,
            after_all: true,
        }
    }
}

impl Default for ParallelTestConfig {
    /// Returns the default instance
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
    /// Returns the default instance
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
    pub fn ci_optimized() -> Self {
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
    pub fn development_optimized() -> Self {
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
    pub fn merge(self, other: Self) -> Self {
        // Simple merge - other takes precedence
        // In a real implementation, you might want more sophisticated merging
        other
    }
}
