// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage-related models and types
//!
//! This module contains models for storage configuration, benchmarking,
//! auto-configuration, and performance analysis.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Canonical network configuration placeholder for REST storage request bodies.
pub type StorageConfigurationCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

/// Canonical network configuration placeholder for REST benchmark request bodies.
pub type BenchmarkConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

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

/// Storage benchmark request configuration
///
/// Specifies parameters for executing storage performance
/// benchmarks against specific backend configurations.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for `BenchmarkStorage` operation
pub struct BenchmarkStorageRequest {
    /// Storage configuration to benchmark
    pub storage_config: StorageConfigurationCanonical,
    /// Benchmark parameters and test configuration
    pub benchmark_config: BenchmarkConfigCanonical,
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
/// Request parameters for `ScanStorage` operation
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

// Additional canonical aliases for auto-config and alternatives (same network config shape).

/// Canonical network configuration for alternative storage options.
pub type AlternativeConfigurationCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

/// Canonical network configuration for auto-config input payloads.
pub type AutoConfigInputCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

/// Canonical network configuration for auto-config requests.
pub type AutoConfigRequestCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

/// Canonical network configuration for auto-config results.
pub type AutoConfigResultCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
