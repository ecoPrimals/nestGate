// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Pure data layer handlers for storage backend operations.
// These handlers provide clean data access for storage management
// without any authentication or user management overhead.

//! Storage module

use axum::{
    extract::{Query, State},
    response::Json,
};
use std::collections::HashMap;
use tracing::{debug, info};

use crate::rest::models::{
    AutoConfigInput, AutoConfigResult, BenchmarkResults, BenchmarkScenario,
    BenchmarkStorageRequest, PerformanceProjection, PerformanceRequirements, ScanStorageRequest,
    StorageBackend, StorageBackendType, StorageConfiguration, StoragePerformance, StorageTier,
    costs::CostEstimate, performance::PerformanceMetrics,
};
use crate::rest::{ApiState, DataError, DataResponse, ListQuery};
use nestgate_core::universal_storage::AutoConfigurator;

// ==================== SECTION ====================
// STORAGE BACKEND DATA HANDLERS
// ==================== SECTION ====================

/// List available storage backends
/// GET /api/v1/storage/backends
pub async fn list_backends(
    State(state): State<ApiState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<DataResponse<Vec<StorageBackend>>>, Json<DataError>> {
    debug!("Listing available storage backends");
    let _detector = state.storage_detector.read().await;

    // Get detected storage backends
    let mut backends = Vec::new();

    // Add memory backend (always available)
    backends.push(StorageBackend {
        backend_type: StorageBackendType::Memory,
        name: "Memory Storage".to_string(),
        config: [
            (
                "description".to_string(),
                "High-speed in-memory storage for temporary data".to_string(),
            ),
            ("available_gb".to_string(), "1".to_string()),
            (
                "capabilities".to_string(),
                "volatile,atomic_writes".to_string(),
            ),
        ]
        .iter()
        .cloned()
        .collect(),
        performance: StoragePerformance {
            read_iops: 100_000,
            write_iops: 80_000,
            read_throughput_mbps: 1000.0,
            write_throughput_mbps: 800.0,
            avg_latency_ms: 0.1,
        },
    });

    // Add filesystem backend (check if available)
    if std::path::Path::new("/tmp").exists() {
        backends.push(StorageBackend {
            backend_type: StorageBackendType::Filesystem,
            name: "Local Filesystem".to_string(),
            config: [
                (
                    "description".to_string(),
                    "Local filesystem storage with persistence".to_string(),
                ),
                ("available_gb".to_string(), "10".to_string()),
                (
                    "capabilities".to_string(),
                    "durable,snapshots,checksumming".to_string(),
                ),
                ("path".to_string(), "/tmp".to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
            performance: StoragePerformance {
                read_iops: 5000,
                write_iops: 3000,
                read_throughput_mbps: 150.0,
                write_throughput_mbps: 100.0,
                avg_latency_ms: 2.0,
            },
        });
    }

    // Apply filtering if specified
    if let Some(filter) = &query.filter {
        backends.retain(|b| {
            b.name.to_lowercase().contains(&filter.to_lowercase())
                || b.backend_type.to_string().contains(filter)
        });
    }

    // Apply sorting
    if let Some(sort_field) = &query.sort {
        match sort_field.as_str() {
            "name" => backends.sort_by(|a, b| a.name.cmp(&b.name)),
            "type" => {
                backends
                    .sort_by(|a, b| a.backend_type.to_string().cmp(&b.backend_type.to_string()));
            }
            "performance" => backends.sort_by(|a, b| {
                a.performance
                    .read_throughput_mbps
                    .partial_cmp(&b.performance.read_throughput_mbps)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            _ => {} // Default order
        }

        if query.order.as_deref() == Some("desc") {
            backends.reverse();
        }
    }

    // Apply pagination
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(50);
    let total = backends.len() as u64;

    let start = ((page - 1) * per_page) as usize;
    let end = (start + per_page as usize).min(backends.len());
    let page_backends = backends[start..end].to_vec();

    info!("Listed {} storage backends", page_backends.len());
    Ok(Json(DataResponse::paginated(
        page_backends,
        total,
        page,
        per_page,
    )))
}

/// Scan for available storage systems
/// POST /api/v1/storage/scan
pub async fn scan_storage(
    State(state): State<ApiState>,
    Json(request): Json<ScanStorageRequest>,
) -> Result<Json<DataResponse<Vec<StorageBackend>>>, Json<DataError>> {
    info!("Scanning for available storage systems");
    let _detector = state.storage_detector.read().await;

    // Configure scan parameters
    // StorageDetectorConfig doesn't exist - using placeholder
    // let mut config = StorageDetectorConfig::default();
    // if let Some(path) = &request.path {
    //     config.base_path = std::path::PathBuf::from(path);
    // }
    // config.include_cloud = request.include_cloud.unwrap_or(false);
    // config.include_network = request.include_network.unwrap_or(false);
    // config.include_block = request.include_block.unwrap_or(false);

    // Perform scan (simplified for demo)
    let mut discovered_backends = Vec::new();

    // Always include memory and filesystem
    discovered_backends.push(StorageBackend {
        backend_type: StorageBackendType::Memory,
        name: "Memory Storage".to_string(),
        config: [
            (
                "description".to_string(),
                "Discovered in-memory storage".to_string(),
            ),
            ("available_gb".to_string(), "2".to_string()),
            ("capabilities".to_string(), "volatile".to_string()),
        ]
        .iter()
        .cloned()
        .collect(),
        performance: StoragePerformance {
            read_iops: 100_000,
            write_iops: 80_000,
            read_throughput_mbps: 1000.0,
            write_throughput_mbps: 800.0,
            avg_latency_ms: 0.1,
        },
    });

    if let Some(path) = &request.path
        && std::path::Path::new(path).exists()
    {
        discovered_backends.push(StorageBackend {
            backend_type: StorageBackendType::Filesystem,
            name: format!("Filesystem at {path}"),
            config: [
                (
                    "description".to_string(),
                    format!("Discovered filesystem storage at {path}"),
                ),
                ("available_gb".to_string(), "5".to_string()),
                ("capabilities".to_string(), "durable,snapshots".to_string()),
                ("path".to_string(), path.clone()),
            ]
            .iter()
            .cloned()
            .collect(),
            performance: StoragePerformance {
                read_iops: 5000,
                write_iops: 3000,
                read_throughput_mbps: 150.0,
                write_throughput_mbps: 100.0,
                avg_latency_ms: 2.0,
            },
        });
    }

    // Mock cloud storage if requested (using request parameters directly)
    if request.include_cloud.unwrap_or(false) {
        discovered_backends.push(StorageBackend {
            backend_type: StorageBackendType::Cloud,
            name: "Cloud Storage (Mock)".to_string(),
            config: [
                (
                    "description".to_string(),
                    "Simulated cloud storage backend".to_string(),
                ),
                ("available_gb".to_string(), "100".to_string()),
                (
                    "capabilities".to_string(),
                    "scalable,durable,backup".to_string(),
                ),
            ]
            .iter()
            .cloned()
            .collect(),
            performance: StoragePerformance {
                read_iops: 2000,
                write_iops: 1500,
                read_throughput_mbps: 50.0,
                write_throughput_mbps: 30.0,
                avg_latency_ms: 10.0,
            },
        });
    }

    info!("Discovered {} storage backends", discovered_backends.len());
    Ok(Json(DataResponse::new(discovered_backends)))
}

/// Benchmark storage performance
/// POST /api/v1/storage/benchmark
pub async fn benchmark_storage(
    State(_state): State<ApiState>,
    Json(request): Json<BenchmarkStorageRequest>,
) -> Result<Json<DataResponse<BenchmarkResults>>, Json<DataError>> {
    info!("Benchmarking storage backend: {:?}", request.backend);
    let duration = request.duration_seconds.unwrap_or(30);
    let _test_size_mb = request.test_size_mb.unwrap_or(100);

    // Simulate benchmark (in real implementation, would perform actual I/O tests)
    tokio::time::sleep(std::time::Duration::from_millis(500)).await; // Simulate test time

    let results = match request.backend {
        StorageBackendType::Memory => BenchmarkResults {
            scenario: BenchmarkScenario::Mixed,
            backend: StorageBackendType::Memory,
            performance: PerformanceMetrics {
                throughput_mbps: 8500.0,
                avg_latency_ms: 0.001,
                p95_latency_ms: 0.002,
                p99_latency_ms: 0.005,
                iops: 850_000,
                cpu_usage: 15.0,
                memory_usage: 25.0,
            },
            duration_seconds: duration,
        },
        StorageBackendType::Filesystem => BenchmarkResults {
            scenario: BenchmarkScenario::Mixed,
            backend: StorageBackendType::Filesystem,
            performance: PerformanceMetrics {
                throughput_mbps: 425.3,
                avg_latency_ms: 0.9,
                p95_latency_ms: 1.2,
                p99_latency_ms: 2.1,
                iops: 42530,
                cpu_usage: 12.0,
                memory_usage: 18.0,
            },
            duration_seconds: duration,
        },
        StorageBackendType::Cloud => BenchmarkResults {
            scenario: BenchmarkScenario::Mixed,
            backend: StorageBackendType::Cloud,
            performance: PerformanceMetrics {
                throughput_mbps: 95.2,
                avg_latency_ms: 45.0,
                p95_latency_ms: 120.0,
                p99_latency_ms: 250.0,
                iops: 9520,
                cpu_usage: 8.0,
                memory_usage: 12.0,
            },
            duration_seconds: duration,
        },
        _ => {
            return Err(Json(DataError::new(
                format!(
                    "Benchmarking not supported for backend: {:?}",
                    request.backend
                ),
                "BENCHMARK_NOT_SUPPORTED".to_string(),
            )));
        }
    };

    info!("Benchmark completed for {:?} backend", request.backend);
    Ok(Json(DataResponse::new(results)))
}

fn development_storage_configuration() -> StorageConfiguration {
    StorageConfiguration {
        name: "Development Setup".to_string(),
        backends: vec![StorageBackend {
            backend_type: StorageBackendType::Memory,
            name: "memory-dev".to_string(),
            config: HashMap::from([("capacity_gb".to_string(), "2".to_string())]),
            performance: StoragePerformance {
                read_iops: 100_000,
                write_iops: 80_000,
                read_throughput_mbps: 1000.0,
                write_throughput_mbps: 800.0,
                avg_latency_ms: 0.1,
            },
        }],
        tier: StorageTier::Hot,
        performance_requirements: PerformanceRequirements {
            min_iops: 800_000,
            min_throughput_mbps: 6000.0,
            max_latency_ms: 0.001,
            availability_percent: 99.9,
        },
    }
}

fn home_nas_storage_configuration(min_capacity_gb: Option<u64>) -> StorageConfiguration {
    StorageConfiguration {
        name: "Home NAS Setup".to_string(),
        backends: vec![StorageBackend {
            backend_type: StorageBackendType::Filesystem,
            name: "filesystem-nas".to_string(),
            config: HashMap::from([(
                "capacity_gb".to_string(),
                min_capacity_gb.unwrap_or(1000).to_string(),
            )]),
            performance: StoragePerformance {
                read_iops: 5000,
                write_iops: 3000,
                read_throughput_mbps: 150.0,
                write_throughput_mbps: 100.0,
                avg_latency_ms: 2.0,
            },
        }],
        tier: StorageTier::Warm,
        performance_requirements: PerformanceRequirements {
            min_iops: 40000,
            min_throughput_mbps: 300.0,
            max_latency_ms: 1.0,
            availability_percent: 99.5,
        },
    }
}

fn database_storage_configuration(min_capacity_gb: Option<u64>) -> StorageConfiguration {
    StorageConfiguration {
        name: "Database Storage Setup".to_string(),
        backends: vec![StorageBackend {
            backend_type: StorageBackendType::Filesystem,
            name: "database-storage".to_string(),
            config: HashMap::from([(
                "capacity_gb".to_string(),
                min_capacity_gb.unwrap_or(500).to_string(),
            )]),
            performance: StoragePerformance {
                read_iops: 8000,
                write_iops: 6000,
                read_throughput_mbps: 200.0,
                write_throughput_mbps: 150.0,
                avg_latency_ms: 1.0,
            },
        }],
        tier: StorageTier::Hot,
        performance_requirements: PerformanceRequirements {
            min_iops: 80000,
            min_throughput_mbps: 500.0,
            max_latency_ms: 0.5,
            availability_percent: 99.99,
        },
    }
}

fn generic_storage_configuration(min_capacity_gb: Option<u64>) -> StorageConfiguration {
    StorageConfiguration {
        name: "Generic Setup".to_string(),
        backends: vec![StorageBackend {
            backend_type: StorageBackendType::Filesystem,
            name: "generic-storage".to_string(),
            config: HashMap::from([(
                "capacity_gb".to_string(),
                min_capacity_gb.unwrap_or(100).to_string(),
            )]),
            performance: StoragePerformance {
                read_iops: 4000,
                write_iops: 2500,
                read_throughput_mbps: 120.0,
                write_throughput_mbps: 80.0,
                avg_latency_ms: 3.0,
            },
        }],
        tier: StorageTier::Warm,
        performance_requirements: PerformanceRequirements {
            min_iops: 30000,
            min_throughput_mbps: 250.0,
            max_latency_ms: 2.0,
            availability_percent: 99.0,
        },
    }
}

fn auto_config_storage_for_use_case(
    use_case: &str,
    min_capacity_gb: Option<u64>,
) -> StorageConfiguration {
    match use_case {
        "Development" => development_storage_configuration(),
        "HomeNas" => home_nas_storage_configuration(min_capacity_gb),
        "Database" => database_storage_configuration(min_capacity_gb),
        _ => generic_storage_configuration(min_capacity_gb),
    }
}

fn default_cost_breakdown() -> HashMap<String, f64> {
    HashMap::from([
        ("storage".to_string(), 15.0),
        ("redundancy".to_string(), 5.0),
        ("monitoring".to_string(), 2.0),
    ])
}

fn cost_estimate_for_use_case(use_case: &str) -> CostEstimate {
    match use_case {
        "Development" => CostEstimate {
            setup_cost: 0.0,
            monthly_cost: 0.0,
            cost_per_gb_monthly: 0.0,
            breakdown: default_cost_breakdown(),
            total_cost: 0.0,
        },
        "HomeNas" => CostEstimate {
            setup_cost: 50.0,
            monthly_cost: 5.0,
            cost_per_gb_monthly: 0.005,
            breakdown: default_cost_breakdown(),
            total_cost: 55.0,
        },
        "Database" => CostEstimate {
            setup_cost: 200.0,
            monthly_cost: 25.0,
            cost_per_gb_monthly: 0.05,
            breakdown: default_cost_breakdown(),
            total_cost: 225.0,
        },
        _ => CostEstimate {
            setup_cost: 25.0,
            monthly_cost: 10.0,
            cost_per_gb_monthly: 0.01,
            breakdown: default_cost_breakdown(),
            total_cost: 35.0,
        },
    }
}

/// Auto-configure optimal storage setup.
///
/// POST `/api/v1/storage/auto-config`
///
/// # Errors
///
/// Returns [`Json<DataError>`] when the auto-configurator state cannot be initialized or validation fails.
pub async fn auto_configure(
    State(state): State<ApiState>,
    Json(request): Json<AutoConfigInput>,
) -> Result<Json<DataResponse<AutoConfigResult>>, Json<DataError>> {
    info!(
        "Auto-configuring storage for use case: {:?}",
        request.use_case
    );
    let _auto = state
        .auto_configurator
        .get_or_init(|| AutoConfigurator::new(vec![]));

    let use_case = request.use_case.as_str();
    let recommended_config = auto_config_storage_for_use_case(use_case, request.min_capacity_gb);
    let cost_estimate = cost_estimate_for_use_case(use_case);

    let performance_projection = PerformanceProjection {
        expected_throughput_mbps: 1000.0,
        expected_latency_ms: 5.0,
        expected_iops: 10_000,
        confidence_percent: 85.0,
    };

    let result = AutoConfigResult {
        recommended_config,
        alternatives: vec![],
        cost_estimate,
        performance_projection,
    };

    info!(
        "Auto-configuration completed for {:?} use case",
        request.use_case
    );
    Ok(Json(DataResponse::new(result)))
}

// ==================== SECTION ====================
// HELPER FUNCTIONS
// ==================== SECTION ====================

/// Get available filesystem space (simplified)
#[allow(dead_code)] // Utility function for filesystem monitoring
fn get_filesystem_space(path: &str) -> Option<u64> {
    use std::fs;
    match fs::metadata(path) {
        Ok(_) => {
            // In a real implementation, would use statvfs or similar
            // For demo, return a reasonable estimate based on /tmp
            Some(10 * 1024 * 1024 * 1024) // 10GB
        }
        Err(_) => None,
    }
}

// Helper trait for converting storage backend types to strings
// Removed duplicate ToString implementation - already exists in zfs.rs

#[cfg(test)]
mod round4_storage_helper_tests {
    use super::*;

    #[test]
    fn auto_config_storage_development_is_memory_hot_tier() {
        let c = super::auto_config_storage_for_use_case("Development", None);
        assert_eq!(c.name, "Development Setup");
        assert!(matches!(c.tier, StorageTier::Hot));
        assert_eq!(c.backends.len(), 1);
    }

    #[test]
    fn auto_config_storage_home_nas_and_database_use_expected_names() {
        let h = super::auto_config_storage_for_use_case("HomeNas", Some(2000));
        assert!(h.name.contains("NAS"));
        let d = super::auto_config_storage_for_use_case("Database", None);
        assert!(d.name.contains("Database"));
        let g = super::auto_config_storage_for_use_case("UnknownProfile", None);
        assert_eq!(g.name, "Generic Setup");
    }

    #[test]
    fn cost_estimate_for_use_case_matches_tiered_defaults() {
        let dev = super::cost_estimate_for_use_case("Development");
        assert_eq!(dev.monthly_cost, 0.0);
        let home = super::cost_estimate_for_use_case("HomeNas");
        assert!(home.setup_cost > 0.0);
        let db = super::cost_estimate_for_use_case("Database");
        assert!(db.total_cost > home.total_cost);
    }

    #[test]
    fn get_filesystem_space_some_for_existing_path() {
        if std::path::Path::new("/tmp").exists() {
            assert_eq!(
                super::get_filesystem_space("/tmp"),
                Some(10 * 1024 * 1024 * 1024)
            );
        }
    }

    #[test]
    fn get_filesystem_space_none_for_nonexistent() {
        assert!(super::get_filesystem_space("/no/such/path/nestgate_round4").is_none());
    }
}
