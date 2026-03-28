// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Storage-related models and types
//!
//! This module contains models for storage configuration, benchmarking,
//! auto-configuration, and performance analysis.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Storage configuration and management structures

/// Storage backend configuration and performance characteristics
///
/// Represents a specific storage backend implementation with its
/// configuration, performance metrics, and operational parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagebackend
pub struct StorageBackend {
    /// Type of storage backend (ZFS, Memory, Local, etc.)
    pub backend_type: crate::rest::models::types::StorageBackendType,
    /// Human-readable name for the storage backend
    pub name: String,
    /// Backend-specific configuration parameters as key-value pairs
    pub config: HashMap<String, String>,
    /// Performance characteristics of this storage backend
    pub performance: StoragePerformance,
}

/// Complete storage system configuration
///
/// Encompasses all storage backends, tier configuration, and
/// performance requirements for a complete storage solution.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::StorageConfiguration;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::StorageConfiguration; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Storageconfiguration
pub struct StorageConfiguration {
    /// Descriptive name for this storage configuration
    pub name: String,
    /// List of configured storage backends
    pub backends: Vec<StorageBackend>,
    /// Storage tier classification (Hot, Warm, Cold, Archive)
    pub tier: StorageTier,
    /// Performance requirements this configuration must meet
    pub performance_requirements: PerformanceRequirements,
}

/// Storage tier classification for data lifecycle management
///
/// Defines different storage tiers based on access patterns,
/// performance requirements, and cost considerations.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagetier
pub enum StorageTier {
    /// Hot tier: Frequently accessed data requiring high performance
    Hot,
    /// Warm tier: Moderately accessed data with balanced performance/cost
    Warm,
    /// Cold tier: Infrequently accessed data prioritizing cost over performance
    Cold,
    /// Archive tier: Long-term storage with minimal access requirements
    Archive,
}

/// Performance requirements specification
///
/// Defines minimum performance thresholds that a storage
/// configuration must meet for acceptable operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancerequirements
pub struct PerformanceRequirements {
    /// Minimum IOPS (Input/Output Operations Per Second) required
    pub min_iops: u64,
    /// Minimum sustained throughput in megabytes per second
    pub min_throughput_mbps: f64,
    /// Maximum acceptable latency in milliseconds
    pub max_latency_ms: f64,
    /// Required availability percentage (e.g., 99.9 for 99.9% uptime)
    pub availability_percent: f64,
}

/// Data reliability and protection requirements
///
/// Specifies durability, availability, backup, and replication
/// requirements for data protection and business continuity.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Reliabilityrequirements
pub struct ReliabilityRequirements {
    /// Required durability in "nines" (e.g., 11 = 99.999999999% durability)
    pub durability_nines: u8,
    /// Required availability in "nines" (e.g., 3 = 99.9% availability)
    pub availability_nines: u8,
    /// Backup frequency in hours (e.g., 24 = daily backups)
    pub backup_frequency_hours: u32,
    /// Number of data replicas to maintain (e.g., 3 = triple replication)
    pub replication_factor: u8,
}

/// Storage performance projection and forecasting
///
/// Predicts expected performance characteristics based on
/// configuration, workload patterns, and historical data.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performanceprojection
pub struct PerformanceProjection {
    /// Projected IOPS under typical workload conditions
    pub expected_iops: u64,
    /// Projected throughput in megabytes per second
    pub expected_throughput_mbps: f64,
    /// Projected average latency in milliseconds
    pub expected_latency_ms: f64,
    /// Confidence level in the projection as percentage (0-100)
    pub confidence_percent: f64,
}

/// Storage scalability analysis and planning
///
/// Analyzes current capacity, growth projections, and
/// scaling options for future capacity planning.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Scalabilityanalysis
pub struct ScalabilityAnalysis {
    /// Current storage capacity in gigabytes
    pub current_capacity_gb: u64,
    /// Maximum capacity achievable with current configuration
    pub max_capacity_gb: u64,
    /// Available scale-out options for capacity expansion
    pub scale_out_options: Vec<String>,
    /// Identified performance or capacity bottlenecks
    pub bottlenecks: Vec<String>,
}

/// Storage benchmark configuration parameters
///
/// Defines parameters for storage performance benchmarking
/// including test scenarios, duration, and load characteristics.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::BenchmarkConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::BenchmarkConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for Benchmark
pub struct BenchmarkConfig {
    /// Type of benchmark scenario to execute
    pub scenario: BenchmarkScenario,
    /// Benchmark duration in seconds
    pub duration_seconds: u32,
    /// Number of concurrent threads to use for testing
    pub thread_count: u32,
    /// I/O block size in kilobytes
    pub block_size_kb: u32,
}

/// Available benchmark scenarios for performance testing
///
/// Defines different I/O patterns and access methods for
/// comprehensive storage performance evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Benchmarkscenario
pub enum BenchmarkScenario {
    /// Random read operations test
    RandomRead,
    /// Random write operations test
    RandomWrite,
    /// Sequential read operations test
    SequentialRead,
    /// Sequential write operations test
    SequentialWrite,
    /// Mixed read/write operations test
    Mixed,
}

/// Benchmark execution results and performance metrics
///
/// Contains the results of a completed storage benchmark
/// including performance metrics and test configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Benchmarkresults
pub struct BenchmarkResults {
    /// Benchmark scenario that was executed
    pub scenario: BenchmarkScenario,
    /// Storage backend that was tested
    pub backend: crate::rest::models::types::StorageBackendType,
    /// Measured performance metrics during the test
    pub performance: crate::rest::models::performance::PerformanceMetrics,
    /// Actual test duration in seconds
    pub duration_seconds: u32,
}

/// Auto-configuration request parameters
///
/// Provides all necessary information for automated storage
/// configuration generation based on requirements and constraints.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::AutoConfigRequest;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::AutoConfigRequest; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Request parameters for AutoConfig operation
pub struct AutoConfigRequest {
    /// Recommended storage configuration
    pub storage_config: StorageConfiguration,
    /// Benchmark configuration for performance validation
    pub benchmark_config: BenchmarkConfig,
    /// Preferred storage backend type
    pub backend: crate::rest::models::types::StorageBackendType,
    /// Optional test duration override in seconds
    pub duration_seconds: Option<u32>,
    /// Optional test data size override in megabytes
    pub test_size_mb: Option<u32>,
}

/// Storage benchmark request configuration
///
/// Specifies parameters for executing storage performance
/// benchmarks against specific backend configurations.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for BenchmarkStorage operation
pub struct BenchmarkStorageRequest {
    /// Storage configuration to benchmark
    pub storage_config: StorageConfiguration,
    /// Benchmark parameters and test configuration
    pub benchmark_config: BenchmarkConfig,
    /// Target storage backend for benchmarking
    pub backend: crate::rest::models::types::StorageBackendType,
    /// Benchmark duration in seconds
    pub duration_seconds: Option<u32>,
    /// Test data size in megabytes
    pub test_size_mb: Option<u32>,
}

/// Storage performance characteristics and metrics
///
/// Comprehensive performance profile including IOPS, throughput,
/// and latency measurements for storage evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storageperformance
pub struct StoragePerformance {
    /// Read operations per second capability
    pub read_iops: u64,
    /// Write operations per second capability
    pub write_iops: u64,
    /// Read throughput in megabytes per second
    pub read_throughput_mbps: f64,
    /// Write throughput in megabytes per second
    pub write_throughput_mbps: f64,
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
}

/// Hardware specification requirements
///
/// Defines minimum hardware requirements for storage
/// configuration including CPU, memory, and network resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Hardwarespec
pub struct HardwareSpec {
    /// Minimum CPU cores required
    pub cpu_cores: u32,
    /// Minimum memory in gigabytes required
    pub memory_gb: u32,
    /// Required storage _devices or drive specifications
    pub storage_devices: Vec<String>,
    /// Minimum network bandwidth in gigabits per second
    pub network_bandwidth_gbps: f64,
}

/// Budget constraints for storage solutions
///
/// Defines financial limits and cost optimization preferences
/// for storage configuration recommendations.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Budgetconstraints
pub struct BudgetConstraints {
    /// Maximum acceptable setup/initial cost
    pub max_setup_cost: f64,
    /// Maximum acceptable monthly operational cost
    pub max_monthly_cost: f64,
    /// Cost vs performance optimization priority (0.0 = performance, 1.0 = cost)
    pub cost_optimization_priority: f32, // 0.0 = performance, 1.0 = cost
}

/// Workload pattern analysis and characteristics
///
/// Describes expected data access patterns, usage timing,
/// and growth projections for optimal storage configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Workloadpattern
pub struct WorkloadPattern {
    /// Read to write ratio (0.0 = all writes, 1.0 = all reads)
    pub read_write_ratio: f32, // 0.0 = all writes, 1.0 = all reads
    /// Data access pattern description ("random", "sequential", "mixed")
    pub access_pattern: String, // "random", "sequential", "mixed"
    /// Peak usage hours of the day (0-23)
    pub peak_hours: Vec<u8>, // Hours of day (0-23)
    /// Expected data growth in gigabytes per month
    pub data_growth_gb_per_month: f64,
}

/// Load testing and capacity planning parameters
///
/// Defines expected load characteristics for capacity planning
/// and performance validation under realistic conditions.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Loadtestingparams
pub struct LoadTestingParams {
    /// Number of concurrent users expected
    pub concurrent_users: u32,
    /// Expected requests per second at peak load
    pub requests_per_second: u32,
    /// Distribution of data sizes for realistic testing
    pub data_size_distribution: HashMap<String, f64>,
}

/// Comprehensive auto-configuration input parameters
///
/// Complete specification for automated storage configuration
/// including all requirements, constraints, and preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::AutoConfigInput;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::AutoConfigInput; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Autoconfiginput
pub struct AutoConfigInput {
    /// Hardware specifications and constraints
    pub hardware: HardwareSpec,
    /// Performance requirements to be met
    pub performance_requirements: PerformanceRequirements,
    /// Data reliability and protection requirements
    pub reliability_requirements: ReliabilityRequirements,
    /// Budget constraints and cost preferences
    pub budget_constraints: BudgetConstraints,
    /// Expected workload patterns and characteristics
    pub workload_pattern: WorkloadPattern,
    /// Primary use case description
    pub use_case: String,
    /// Minimum required capacity in gigabytes
    pub min_capacity_gb: Option<u64>,
}

/// Auto-configuration result with recommendations
///
/// Complete result of automated configuration analysis including
/// recommendations, alternatives, projections, and cost estimates.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::AutoConfigResult;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::AutoConfigResult; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Autoconfigresult
pub struct AutoConfigResult {
    /// Primary recommended storage configuration
    pub recommended_config: StorageConfiguration,
    /// Alternative configuration options
    pub alternatives: Vec<AlternativeConfiguration>,
    /// Performance projections for the recommended configuration
    pub performance_projection: PerformanceProjection,
    /// Cost estimation for the recommended solution
    pub cost_estimate: crate::rest::models::costs::CostEstimate,
}

/// Alternative storage configuration option
///
/// Represents an alternative storage configuration with different
/// trade-offs between performance, cost, and features.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::AlternativeConfiguration;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::AlternativeConfiguration; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Alternativeconfiguration
pub struct AlternativeConfiguration {
    /// Alternative storage configuration
    pub config: StorageConfiguration,
    /// Performance projection for this alternative
    pub performance_projection: PerformanceProjection,
    /// Cost estimate for this alternative
    pub cost_estimate: crate::rest::models::costs::CostEstimate,
    /// Trade-offs and considerations for this alternative
    pub trade_offs: Vec<String>,
}

/// Detailed cost breakdown by category
///
/// Provides granular cost analysis broken down by different
/// cost categories for detailed financial planning.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Costbreakdown
pub struct CostBreakdown {
    /// Hardware acquisition and setup costs
    pub hardware_cost: f64,
    /// Software licensing and subscription costs
    pub software_cost: f64,
    /// Ongoing operational costs (power, cooling, etc.)
    pub operational_cost: f64,
    /// Maintenance and support costs
    pub maintenance_cost: f64,
}

/// Storage system scanning and discovery parameters
///
/// Configuration for automated discovery and analysis of
/// existing storage systems and available resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for ScanStorage operation
pub struct ScanStorageRequest {
    /// Scanning depth level (1 = shallow, 5 = deep analysis)
    pub scan_depth: u8, // 1 = shallow, 5 = deep
    /// Include performance benchmarking in scan
    pub include_performance: bool,
    /// Include cost analysis in scan results
    pub include_costs: bool,
    /// Target storage backend types to scan for
    pub target_backends: Vec<crate::rest::models::types::StorageBackendType>,
    /// Optional path to scan (for filesystem-based backends)
    pub path: Option<String>,
    /// Include cloud storage options in scan
    pub include_cloud: Option<bool>,
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Alternativeconfigurationcanonical
pub type AlternativeConfigurationCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using AlternativeConfiguration (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Autoconfiginputcanonical
pub type AutoConfigInputCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using AutoConfigInput (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Autoconfigrequestcanonical
pub type AutoConfigRequestCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using AutoConfigRequest (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Autoconfigresultcanonical
pub type AutoConfigResultCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using AutoConfigResult (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Benchmarkconfigcanonical
pub type BenchmarkConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using BenchmarkConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Storageconfigurationcanonical
pub type StorageConfigurationCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StorageConfiguration (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.
