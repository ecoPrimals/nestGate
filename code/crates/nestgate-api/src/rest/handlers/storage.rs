//
// Pure data layer handlers for storage backend operations.
// These handlers provide clean data access for storage management
// without any authentication or user management overhead.

use axum::{
    extract::{Query, State},
    response::Json,
};
use std::collections::HashMap;
use tracing::{debug, info};

use crate::rest::models::*;
use crate::rest::{ApiState, DataError, DataResponse, ListQuery};
use nestgate_core::universal_storage::AutoConfigurator;

// ============================================================================
// STORAGE BACKEND DATA HANDLERS
// ============================================================================

/// List available storage backends
/// GET /api/v1/storage/backends
pub async fn list_backends(
    State(state): State<ApiState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<DataResponse<Vec<StorageBackend>>>, Json<DataError>> {
    debug!("Listing available storage backends");

    let _detector = state.storage_detector.lock().await;

    // Get detected storage backends
    let mut backends = Vec::new();

    // Add memory backend (always available)
    backends.push(StorageBackend {
        backend_type: StorageBackendType::Memory,
        name: "Memory Storage".to_string(),
        description: "High-speed in-memory storage for temporary data".to_string(),
        available_bytes: 1024 * 1024 * 1024, // 1GB
        total_bytes: 1024 * 1024 * 1024,
        capabilities: vec![
            StorageCapability::BasicOperations,
            StorageCapability::Volatile,
            StorageCapability::AtomicWrites,
        ],
        performance: StoragePerformance {
            read_throughput_mbps: 10000.0,
            write_throughput_mbps: 8000.0,
            avg_latency_ms: 0.001,
            iops: 1000000,
            tier: PerformanceTier::High,
        },
        status: StorageBackendStatus::Online,
        config_schema: None,
    });

    // Add filesystem backend (check if available)
    if std::path::Path::new("/tmp").exists() {
        backends.push(StorageBackend {
            backend_type: StorageBackendType::Filesystem,
            name: "Local Filesystem".to_string(),
            description: "Local filesystem storage with persistence".to_string(),
            available_bytes: get_filesystem_space("/tmp").unwrap_or(10 * 1024 * 1024 * 1024), // 10GB default
            total_bytes: get_filesystem_space("/tmp").unwrap_or(10 * 1024 * 1024 * 1024),
            capabilities: vec![
                StorageCapability::BasicOperations,
                StorageCapability::Durable,
                StorageCapability::Snapshots,
                StorageCapability::Checksumming,
            ],
            performance: StoragePerformance {
                read_throughput_mbps: 500.0,
                write_throughput_mbps: 400.0,
                avg_latency_ms: 1.0,
                iops: 50000,
                tier: PerformanceTier::Medium,
            },
            status: StorageBackendStatus::Online,
            config_schema: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Base path for filesystem storage"
                    }
                },
                "required": ["path"]
            })),
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
                backends.sort_by(|a, b| a.backend_type.to_string().cmp(&b.backend_type.to_string()))
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

    let _detector = state.storage_detector.lock().await;

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
        description: "Discovered in-memory storage".to_string(),
        available_bytes: 2 * 1024 * 1024 * 1024, // 2GB
        total_bytes: 2 * 1024 * 1024 * 1024,
        capabilities: vec![
            StorageCapability::BasicOperations,
            StorageCapability::Volatile,
        ],
        performance: StoragePerformance {
            read_throughput_mbps: 12000.0,
            write_throughput_mbps: 9000.0,
            avg_latency_ms: 0.001,
            iops: 1200000,
            tier: PerformanceTier::High,
        },
        status: StorageBackendStatus::Online,
        config_schema: None,
    });

    if let Some(path) = &request.path {
        if std::path::Path::new(path).exists() {
            discovered_backends.push(StorageBackend {
                backend_type: StorageBackendType::Filesystem,
                name: format!("Filesystem at {}", path),
                description: format!("Discovered filesystem storage at {}", path),
                available_bytes: get_filesystem_space(path).unwrap_or(5 * 1024 * 1024 * 1024),
                total_bytes: get_filesystem_space(path).unwrap_or(5 * 1024 * 1024 * 1024),
                capabilities: vec![
                    StorageCapability::BasicOperations,
                    StorageCapability::Durable,
                    StorageCapability::Snapshots,
                ],
                performance: StoragePerformance {
                    read_throughput_mbps: 450.0,
                    write_throughput_mbps: 350.0,
                    avg_latency_ms: 1.2,
                    iops: 45000,
                    tier: PerformanceTier::Medium,
                },
                status: StorageBackendStatus::Online,
                config_schema: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "default": path
                        }
                    }
                })),
            });
        }
    }

    // Mock cloud storage if requested (using request parameters directly)
    if request.include_cloud.unwrap_or(false) {
        discovered_backends.push(StorageBackend {
            backend_type: StorageBackendType::Cloud,
            name: "Cloud Storage (Mock)".to_string(),
            description: "Simulated cloud storage backend".to_string(),
            available_bytes: 100 * 1024 * 1024 * 1024, // 100GB
            total_bytes: 100 * 1024 * 1024 * 1024,
            capabilities: vec![
                StorageCapability::BasicOperations,
                StorageCapability::Scalable,
                StorageCapability::Durable,
                StorageCapability::Backup,
            ],
            performance: StoragePerformance {
                read_throughput_mbps: 100.0,
                write_throughput_mbps: 80.0,
                avg_latency_ms: 50.0,
                iops: 5000,
                tier: PerformanceTier::Low,
            },
            status: StorageBackendStatus::Online,
            config_schema: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "access_key": {"type": "string"},
                    "secret_key": {"type": "string"},
                    "region": {"type": "string"},
                    "bucket": {"type": "string"}
                },
                "required": ["access_key", "secret_key", "bucket"]
            })),
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
    let test_size_mb = request.test_size_mb.unwrap_or(100);

    // Simulate benchmark (in real implementation, would perform actual I/O tests)
    tokio::time::sleep(std::time::Duration::from_millis(500)).await; // Simulate test time

    let results = match request.backend {
        StorageBackendType::Memory => BenchmarkResults {
            backend: StorageBackendType::Memory,
            duration_seconds: duration,
            test_size_mb,
            read_performance: PerformanceMetrics {
                throughput_mbps: 8500.0,
                avg_latency_ms: 0.001,
                p95_latency_ms: 0.002,
                p99_latency_ms: 0.005,
                iops: 850000,
            },
            write_performance: PerformanceMetrics {
                throughput_mbps: 7200.0,
                avg_latency_ms: 0.002,
                p95_latency_ms: 0.003,
                p99_latency_ms: 0.008,
                iops: 720000,
            },
            mixed_performance: Some(PerformanceMetrics {
                throughput_mbps: 6800.0,
                avg_latency_ms: 0.003,
                p95_latency_ms: 0.005,
                p99_latency_ms: 0.012,
                iops: 680000,
            }),
            timestamp: chrono::Utc::now(),
        },
        StorageBackendType::Filesystem => BenchmarkResults {
            backend: StorageBackendType::Filesystem,
            duration_seconds: duration,
            test_size_mb,
            read_performance: PerformanceMetrics {
                throughput_mbps: 425.3,
                avg_latency_ms: 0.9,
                p95_latency_ms: 1.2,
                p99_latency_ms: 2.1,
                iops: 42530,
            },
            write_performance: PerformanceMetrics {
                throughput_mbps: 398.7,
                avg_latency_ms: 1.1,
                p95_latency_ms: 1.5,
                p99_latency_ms: 2.8,
                iops: 39870,
            },
            mixed_performance: Some(PerformanceMetrics {
                throughput_mbps: 380.2,
                avg_latency_ms: 1.3,
                p95_latency_ms: 1.8,
                p99_latency_ms: 3.2,
                iops: 38020,
            }),
            timestamp: chrono::Utc::now(),
        },
        StorageBackendType::Cloud => BenchmarkResults {
            backend: StorageBackendType::Cloud,
            duration_seconds: duration,
            test_size_mb,
            read_performance: PerformanceMetrics {
                throughput_mbps: 95.2,
                avg_latency_ms: 45.0,
                p95_latency_ms: 120.0,
                p99_latency_ms: 250.0,
                iops: 9520,
            },
            write_performance: PerformanceMetrics {
                throughput_mbps: 78.5,
                avg_latency_ms: 55.0,
                p95_latency_ms: 150.0,
                p99_latency_ms: 300.0,
                iops: 7850,
            },
            mixed_performance: Some(PerformanceMetrics {
                throughput_mbps: 82.1,
                avg_latency_ms: 50.0,
                p95_latency_ms: 135.0,
                p99_latency_ms: 275.0,
                iops: 8210,
            }),
            timestamp: chrono::Utc::now(),
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

/// Auto-configure optimal storage setup
/// POST /api/v1/storage/auto-config
pub async fn auto_configure(
    State(state): State<ApiState>,
    Json(request): Json<AutoConfigRequest>,
) -> Result<Json<DataResponse<AutoConfigResult>>, Json<DataError>> {
    info!(
        "Auto-configuring storage for use case: {:?}",
        request.use_case
    );

    // Create auto-configurator if not exists
    let mut auto_config_opt = state.auto_configurator.lock().await;
    if auto_config_opt.is_none() {
        *auto_config_opt = Some(AutoConfigurator::new(vec![])); // Empty storage list for now
    }

    // Auto configurator would be used here in full implementation
    // let _auto_configurator = auto_config_opt.as_ref();

    let _config = match request.use_case {
        UseCase::Development => StorageConfiguration {
            name: "Development Setup".to_string(),
            tiers: vec![StorageTier {
                name: "Dev Storage".to_string(),
                backend: StorageBackendType::Memory,
                capacity_gb: 2,
                purpose: "Fast development storage".to_string(),
                performance: StoragePerformance {
                    read_throughput_mbps: 8000.0,
                    write_throughput_mbps: 6000.0,
                    avg_latency_ms: 0.001,
                    iops: 800000,
                    tier: PerformanceTier::High,
                },
            }],
            total_capacity_gb: 2,
            redundancy: RedundancyLevel::None,
            features: vec![
                StorageCapability::BasicOperations,
                StorageCapability::Volatile,
            ],
            implementation_steps: vec![
                "Create memory-based storage backend".to_string(),
                "Configure development datasets".to_string(),
                "Set up basic monitoring".to_string(),
            ],
        },
        UseCase::HomeNas => StorageConfiguration {
            name: "Home NAS Setup".to_string(),
            tiers: vec![StorageTier {
                name: "Primary Storage".to_string(),
                backend: StorageBackendType::Filesystem,
                capacity_gb: request.min_capacity_gb.unwrap_or(1000),
                purpose: "Main file storage with redundancy".to_string(),
                performance: StoragePerformance {
                    read_throughput_mbps: 400.0,
                    write_throughput_mbps: 300.0,
                    avg_latency_ms: 1.0,
                    iops: 40000,
                    tier: PerformanceTier::Medium,
                },
            }],
            total_capacity_gb: request.min_capacity_gb.unwrap_or(1000),
            redundancy: request.redundancy_level.unwrap_or(RedundancyLevel::Mirror),
            features: vec![
                StorageCapability::BasicOperations,
                StorageCapability::Durable,
                StorageCapability::Snapshots,
                StorageCapability::Compression,
                StorageCapability::Checksumming,
            ],
            implementation_steps: vec![
                "Set up filesystem backend".to_string(),
                "Configure RAID mirror for redundancy".to_string(),
                "Enable compression and checksumming".to_string(),
                "Set up automated snapshots".to_string(),
                "Configure monitoring and alerts".to_string(),
            ],
        },
        UseCase::Database => StorageConfiguration {
            name: "Database Storage Setup".to_string(),
            tiers: vec![StorageTier {
                name: "High-Performance Tier".to_string(),
                backend: StorageBackendType::Filesystem,
                capacity_gb: request.min_capacity_gb.unwrap_or(500),
                purpose: "High-IOPS database storage".to_string(),
                performance: StoragePerformance {
                    read_throughput_mbps: 600.0,
                    write_throughput_mbps: 500.0,
                    avg_latency_ms: 0.5,
                    iops: 80000,
                    tier: PerformanceTier::High,
                },
            }],
            total_capacity_gb: request.min_capacity_gb.unwrap_or(500),
            redundancy: RedundancyLevel::RaidZ2,
            features: vec![
                StorageCapability::BasicOperations,
                StorageCapability::Durable,
                StorageCapability::AtomicWrites,
                StorageCapability::Checksumming,
            ],
            implementation_steps: vec![
                "Configure high-performance storage backend".to_string(),
                "Set up RAID-Z2 for fault tolerance".to_string(),
                "Enable atomic writes for consistency".to_string(),
                "Configure low-latency monitoring".to_string(),
            ],
        },
        _ => StorageConfiguration {
            name: "Generic Setup".to_string(),
            tiers: vec![StorageTier {
                name: "General Purpose".to_string(),
                backend: StorageBackendType::Filesystem,
                capacity_gb: request.min_capacity_gb.unwrap_or(100),
                purpose: "General purpose storage".to_string(),
                performance: StoragePerformance {
                    read_throughput_mbps: 300.0,
                    write_throughput_mbps: 250.0,
                    avg_latency_ms: 2.0,
                    iops: 30000,
                    tier: PerformanceTier::Medium,
                },
            }],
            total_capacity_gb: request.min_capacity_gb.unwrap_or(100),
            redundancy: RedundancyLevel::None,
            features: vec![
                StorageCapability::BasicOperations,
                StorageCapability::Durable,
            ],
            implementation_steps: vec![
                "Set up basic storage backend".to_string(),
                "Configure monitoring".to_string(),
            ],
        },
    };

    // Generate cost estimate
    let cost_estimate = CostEstimate {
        setup_cost: match request.use_case {
            UseCase::Development => 0.0,
            UseCase::HomeNas => 50.0,
            UseCase::Database => 200.0,
            _ => 25.0,
        },
        monthly_cost: match request.use_case {
            UseCase::Development => 0.0,
            UseCase::HomeNas => 5.0,
            UseCase::Database => 25.0,
            _ => 10.0,
        },
        cost_per_gb_monthly: match request.use_case {
            UseCase::Development => 0.0,
            UseCase::HomeNas => 0.005,
            UseCase::Database => 0.05,
            _ => 0.01,
        },
        breakdown: {
            let mut breakdown = HashMap::new();
            breakdown.insert("storage".to_string(), 15.0);
            breakdown.insert("redundancy".to_string(), 5.0);
            breakdown.insert("monitoring".to_string(), 2.0);
            breakdown
        },
    };

    // Generate performance projection (using placeholder values)
    let performance_projection = PerformanceProjection {
        expected_throughput_mbps: 1000.0, // Placeholder
        expected_latency_ms: 5.0,         // Placeholder
        expected_iops: 10000,             // Placeholder
        scalability: match request.use_case {
            UseCase::Development => "Limited scalability, optimized for speed".to_string(),
            UseCase::HomeNas => "Moderate scalability, can add drives".to_string(),
            UseCase::Database => "High scalability with performance focus".to_string(),
            _ => "Basic scalability".to_string(),
        },
    };

    let result = AutoConfigResult {
        recommended_config: StorageConfiguration {
            name: format!("{:?} Configuration", request.use_case),
            tiers: vec![], // Placeholder - would be populated with actual tiers
            total_capacity_gb: 1000, // Placeholder
            redundancy: RedundancyLevel::RaidZ1, // Placeholder
            features: vec![StorageCapability::BasicOperations], // Placeholder
            implementation_steps: vec![
                "Configure storage backend".to_string(),
                "Set up monitoring".to_string(),
                "Test configuration".to_string(),
            ],
        },
        alternatives: vec![], // Would generate alternatives in full implementation
        rationale: format!(
            "Optimized configuration for {} use case with default settings",
            format!("{:?}", request.use_case).to_lowercase()
        ),
        cost_estimate,
        performance_projection,
    };

    info!(
        "Auto-configuration completed for {:?} use case",
        request.use_case
    );
    Ok(Json(DataResponse::new(result)))
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Get available filesystem space (simplified)
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
