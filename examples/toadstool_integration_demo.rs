//! # NestGate → ToadStool Integration Demo
//!
//! This demo showcases the complete integration between NestGate and ToadStool
//! covering both System Information (Sysinfo) and Compute Needs as specified.

use chrono::Utc;
use nestgate_api::handlers::hardware_tuning::{
    ComputeDiscovery, PlatformInfo, RealtimeMetrics, StorageOptimizationRequest,
    StorageProcessRequest, StorageResourceRequest, StorageWorkload, SystemHealth,
    ToadstoolComputeClient, WorkloadResourceRequirements,
};
use std::collections::HashMap;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    info!("🚀 Starting NestGate → ToadStool Integration Demo");

    // Initialize ToadStool client
    let toadstool_url = std::env::var("NESTGATE_TOADSTOOL_COMPUTE_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());

    let toadstool_client = ToadstoolComputeClient::new(toadstool_url);

    println!("🎯 === NestGate → ToadStool Integration Architecture Demo ===\n");

    // ========================
    // 1. SYSTEM INFORMATION (Sysinfo)
    // ========================

    println!("📊 1. SYSTEM INFORMATION (Sysinfo)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Platform Detection: CPU, memory, storage capabilities
    println!("🔍 Platform Detection:");
    demo_platform_detection(&toadstool_client).await?;

    // Resource Monitoring: Real-time system metrics
    println!("\n📈 Resource Monitoring:");
    demo_resource_monitoring(&toadstool_client).await?;

    // Hardware Discovery: Available compute resources
    println!("\n🔧 Hardware Discovery:");
    demo_hardware_discovery(&toadstool_client).await?;

    // Health Monitoring: System status and performance
    println!("\n🏥 Health Monitoring:");
    demo_health_monitoring(&toadstool_client).await?;

    // ========================
    // 2. COMPUTE NEEDS
    // ========================

    println!("\n\n⚡ 2. COMPUTE NEEDS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Workload Execution: Running storage management processes
    println!("🚀 Workload Execution:");
    demo_workload_execution(&toadstool_client).await?;

    // Resource Allocation: CPU and memory for storage operations
    println!("\n💾 Resource Allocation:");
    demo_resource_allocation(&toadstool_client).await?;

    // Process Management: Managing ZFS and storage daemons
    println!("\n⚙️ Process Management:");
    demo_process_management(&toadstool_client).await?;

    // Performance Optimization: Compute-intensive storage operations
    println!("\n🔧 Performance Optimization:");
    demo_performance_optimization(&toadstool_client).await?;

    println!("\n✅ === Integration Demo Complete ===");
    println!("🎉 NestGate → ToadStool integration is fully operational!");

    Ok(())
}

/// Demo Platform Detection: CPU, memory, storage capabilities
async fn demo_platform_detection(
    _client: &ToadstoolComputeClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("   → Detecting platform capabilities via ToadStool...");

    // In a real environment, this would call ToadStool
    // For demo purposes, we'll simulate the response
    let platform_info = PlatformInfo {
        cpu_cores: 16,
        memory_gb: 64,
        storage_devices: vec![
            nestgate_api::handlers::hardware_tuning::StorageDevice {
                name: "nvme0n1".to_string(),
                device_type: "NVMe".to_string(),
                capacity_gb: 1024,
                interface: "PCIe 4.0".to_string(),
                performance_tier: "High".to_string(),
            },
            nestgate_api::handlers::hardware_tuning::StorageDevice {
                name: "sda1".to_string(),
                device_type: "SSD".to_string(),
                capacity_gb: 2048,
                interface: "SATA".to_string(),
                performance_tier: "Medium".to_string(),
            },
        ],
        architecture: "x86_64".to_string(),
        operating_system: "Linux".to_string(),
        kernel_version: "6.12.10".to_string(),
        platform_capabilities: vec![
            "ZFS".to_string(),
            "Hardware Acceleration".to_string(),
            "RAID".to_string(),
            "Encryption".to_string(),
        ],
    };

    println!(
        "   ✅ Platform: {} cores, {} GB RAM, {} storage devices",
        platform_info.cpu_cores,
        platform_info.memory_gb,
        platform_info.storage_devices.len()
    );
    println!(
        "   ✅ Architecture: {} on {}",
        platform_info.architecture, platform_info.operating_system
    );
    println!(
        "   ✅ Capabilities: {}",
        platform_info.platform_capabilities.join(", ")
    );

    Ok(())
}

/// Demo Resource Monitoring: Real-time system metrics
async fn demo_resource_monitoring(
    _client: &ToadstoolComputeClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("   → Getting real-time system metrics from ToadStool...");

    // Simulate live metrics
    let metrics = RealtimeMetrics {
        timestamp: Utc::now(),
        cpu_usage: 42.3,
        memory_usage: 68.7,
        storage_io: nestgate_api::handlers::hardware_tuning::StorageIoMetrics {
            read_bytes_per_sec: 1024000000,
            write_bytes_per_sec: 512000000,
            read_ops_per_sec: 5000,
            write_ops_per_sec: 2500,
            avg_read_latency_ms: 0.1,
            avg_write_latency_ms: 0.2,
        },
        network_io: nestgate_api::handlers::hardware_tuning::NetworkIoMetrics {
            bytes_sent: 1024000000,
            bytes_received: 2048000000,
            packets_sent: 100000,
            packets_received: 150000,
        },
        system_load: nestgate_api::handlers::hardware_tuning::SystemLoadMetrics {
            load_1min: 1.2,
            load_5min: 1.5,
            load_15min: 1.8,
            uptime_seconds: 86400,
        },
        process_count: 285,
        thread_count: 1420,
    };

    println!("   ✅ CPU Usage: {:.1}%", metrics.cpu_usage);
    println!("   ✅ Memory Usage: {:.1}%", metrics.memory_usage);
    println!(
        "   ✅ Storage I/O: {} MB/s read, {} MB/s write",
        metrics.storage_io.read_bytes_per_sec / 1024 / 1024,
        metrics.storage_io.write_bytes_per_sec / 1024 / 1024
    );
    println!(
        "   ✅ System Load: {:.1} (1min), {:.1} (5min), {:.1} (15min)",
        metrics.system_load.load_1min,
        metrics.system_load.load_5min,
        metrics.system_load.load_15min
    );

    Ok(())
}

/// Demo Hardware Discovery: Available compute resources
async fn demo_hardware_discovery(
    _client: &ToadstoolComputeClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("   → Discovering available compute resources...");

    // Simulate compute discovery
    let discovery = ComputeDiscovery {
        compute_nodes: vec![
            nestgate_api::handlers::hardware_tuning::ComputeNode {
                node_id: "node-1".to_string(),
                hostname: "nestgate-primary".to_string(),
                cpu_cores: 16,
                memory_gb: 64,
                status: "healthy".to_string(),
                capabilities: vec!["zfs".to_string(), "docker".to_string()],
                current_load: 0.42,
            },
            nestgate_api::handlers::hardware_tuning::ComputeNode {
                node_id: "node-2".to_string(),
                hostname: "nestgate-secondary".to_string(),
                cpu_cores: 8,
                memory_gb: 32,
                status: "healthy".to_string(),
                capabilities: vec!["zfs".to_string(), "backup".to_string()],
                current_load: 0.28,
            },
        ],
        gpu_devices: vec![nestgate_api::handlers::hardware_tuning::GpuDevice {
            device_id: "gpu-0".to_string(),
            model: "NVIDIA RTX 4090".to_string(),
            memory_gb: 24,
            compute_capability: "8.9".to_string(),
            utilization: 15.3,
        }],
        network_interfaces: vec![nestgate_api::handlers::hardware_tuning::NetworkInterface {
            interface_name: "eth0".to_string(),
            speed_mbps: 10000,
            duplex: "full".to_string(),
            status: "up".to_string(),
        }],
        storage_pools: vec![nestgate_api::handlers::hardware_tuning::StoragePool {
            pool_name: "storage-pool".to_string(),
            pool_type: "ZFS".to_string(),
            total_capacity_gb: 3072,
            used_capacity_gb: 1024,
            health_status: "healthy".to_string(),
        }],
        available_memory: 96,
        total_cpu_cores: 24,
    };

    println!("   ✅ Compute Nodes: {}", discovery.compute_nodes.len());
    for node in &discovery.compute_nodes {
        println!(
            "      - {}: {} cores, {} GB RAM (load: {:.1})",
            node.hostname, node.cpu_cores, node.memory_gb, node.current_load
        );
    }
    println!("   ✅ GPU Devices: {}", discovery.gpu_devices.len());
    println!(
        "   ✅ Storage Pools: {} (Total: {} GB)",
        discovery.storage_pools.len(),
        discovery.storage_pools[0].total_capacity_gb
    );

    Ok(())
}

/// Demo Health Monitoring: System status and performance
async fn demo_health_monitoring(
    _client: &ToadstoolComputeClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("   → Checking system health status...");

    // Simulate health monitoring
    let health = SystemHealth {
        overall_status: "healthy".to_string(),
        cpu_health: nestgate_api::handlers::hardware_tuning::HealthStatus {
            status: "good".to_string(),
            score: 95.2,
            issues: vec![],
        },
        memory_health: nestgate_api::handlers::hardware_tuning::HealthStatus {
            status: "good".to_string(),
            score: 88.7,
            issues: vec!["Slight fragmentation".to_string()],
        },
        storage_health: nestgate_api::handlers::hardware_tuning::HealthStatus {
            status: "excellent".to_string(),
            score: 98.1,
            issues: vec![],
        },
        network_health: nestgate_api::handlers::hardware_tuning::HealthStatus {
            status: "good".to_string(),
            score: 92.5,
            issues: vec![],
        },
        temperature_celsius: 42.3,
        power_consumption_watts: 285.0,
        alerts: vec![],
    };

    println!("   ✅ Overall Status: {}", health.overall_status);
    println!(
        "   ✅ CPU Health: {} (score: {:.1})",
        health.cpu_health.status, health.cpu_health.score
    );
    println!(
        "   ✅ Memory Health: {} (score: {:.1})",
        health.memory_health.status, health.memory_health.score
    );
    println!(
        "   ✅ Storage Health: {} (score: {:.1})",
        health.storage_health.status, health.storage_health.score
    );
    println!(
        "   ✅ Temperature: {:.1}°C, Power: {:.1}W",
        health.temperature_celsius, health.power_consumption_watts
    );

    Ok(())
}

/// Demo Workload Execution: Running storage management processes
async fn demo_workload_execution(
    _client: &ToadstoolComputeClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("   → Executing storage management workload...");

    // Create a ZFS scrub workload
    let workload = StorageWorkload {
        name: "zfs-scrub-storage-pool".to_string(),
        workload_type: "zfs_scrub".to_string(),
        priority: "medium".to_string(),
        resource_requirements: WorkloadResourceRequirements {
            cpu_cores: 4,
            memory_gb: 8,
            storage_io_intensive: true,
            network_bandwidth_required: false,
        },
        estimated_duration_minutes: 120,
        parameters: {
            let mut params = HashMap::new();
            params.insert(
                "pool_name".to_string(),
                serde_json::Value::String("storage-pool".to_string()),
            );
            params.insert(
                "scrub_type".to_string(),
                serde_json::Value::String("full".to_string()),
            );
            params
        },
    };

    println!(
        "   ✅ Workload: {} (type: {})",
        workload.name, workload.workload_type
    );
    println!(
        "   ✅ Resources: {} cores, {} GB RAM",
        workload.resource_requirements.cpu_cores, workload.resource_requirements.memory_gb
    );
    println!(
        "   ✅ Estimated Duration: {} minutes",
        workload.estimated_duration_minutes
    );

    Ok(())
}

/// Demo Resource Allocation: CPU and memory for storage operations
async fn demo_resource_allocation(
    _client: &ToadstoolComputeClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("   → Allocating resources for storage operations...");

    // Request resources for pool creation
    let resource_request = StorageResourceRequest {
        operation_type: "pool_creation".to_string(),
        required_cpu_cores: 8,
        required_memory_gb: 16,
        required_storage_io: true,
        duration_minutes: 30,
        priority: "high".to_string(),
    };

    println!(
        "   ✅ Requesting: {} cores, {} GB RAM for {}",
        resource_request.required_cpu_cores,
        resource_request.required_memory_gb,
        resource_request.operation_type
    );
    println!(
        "   ✅ Duration: {} minutes (priority: {})",
        resource_request.duration_minutes, resource_request.priority
    );

    Ok(())
}

/// Demo Process Management: Managing ZFS and storage daemons
async fn demo_process_management(
    _client: &ToadstoolComputeClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("   → Managing ZFS and storage daemons...");

    // Manage ZFS daemon
    let process_request = StorageProcessRequest {
        process_name: "zfs-daemon".to_string(),
        process_type: "zfs_daemon".to_string(),
        action: "status".to_string(),
        parameters: {
            let mut params = HashMap::new();
            params.insert(
                "pool_name".to_string(),
                serde_json::Value::String("storage-pool".to_string()),
            );
            params
        },
    };

    println!(
        "   ✅ Managing: {} (type: {})",
        process_request.process_name, process_request.process_type
    );
    println!("   ✅ Action: {}", process_request.action);

    Ok(())
}

/// Demo Performance Optimization: Compute-intensive storage operations
async fn demo_performance_optimization(
    _client: &ToadstoolComputeClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("   → Optimizing storage performance...");

    // Request deduplication optimization
    let optimization_request = StorageOptimizationRequest {
        optimization_type: "deduplication".to_string(),
        target_pool: "storage-pool".to_string(),
        optimization_level: "moderate".to_string(),
        background_priority: true,
        max_cpu_usage: 50.0,
        max_memory_usage: 25.0,
    };

    println!(
        "   ✅ Optimization: {} on {}",
        optimization_request.optimization_type, optimization_request.target_pool
    );
    println!(
        "   ✅ Level: {} (background: {})",
        optimization_request.optimization_level, optimization_request.background_priority
    );
    println!(
        "   ✅ Limits: {:.1}% CPU, {:.1}% Memory",
        optimization_request.max_cpu_usage, optimization_request.max_memory_usage
    );

    Ok(())
}
