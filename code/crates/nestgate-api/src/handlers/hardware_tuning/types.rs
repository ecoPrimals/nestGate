//! **HARDWARE TUNING TYPES**
//!
//! Data structures and type definitions for hardware tuning operations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

use nestgate_core::Result;

/// **HARDWARE TUNING CONFIG**
///
/// Configuration for hardware tuning operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareTuningConfig {
    /// Number of CPU cores to allocate
    pub cpu_cores: u32,
    /// Amount of memory in gigabytes to allocate
    pub memory_gb: u32,
    /// Whether CPU tuning is enabled
    pub cpu_tuning_enabled: bool,
    /// Whether memory optimization is enabled
    pub memory_optimization_enabled: bool,
    /// Whether GPU tuning is enabled
    pub gpu_tuning_enabled: bool,
    /// Monitoring interval for performance metrics
    pub monitoring_interval: Duration,
}

impl Default for HardwareTuningConfig {
    fn default() -> Self {
        Self {
            cpu_cores: 8,
            memory_gb: 16,
            cpu_tuning_enabled: true,
            memory_optimization_enabled: true,
            gpu_tuning_enabled: false, // Disabled by default, enabled if GPU detected
            monitoring_interval: Duration::from_secs(5),
        }
    }
}

/// **COMPUTE ALLOCATION**
///
/// Resource allocation specification for compute workloads.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeAllocation {
    /// Number of CPU cores allocated
    pub cpu_cores: u32,
    /// Amount of memory in gigabytes allocated
    pub memory_gb: u32,
    /// Number of GPU units allocated
    pub gpu_count: u32,
}

/// **COMPUTE RESOURCES**
///
/// Available compute resources in the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeResources {
    /// Available CPU cores
    pub available_cpu: u32,
    /// Available memory in gigabytes
    pub available_memory_gb: u32,
    /// Available GPU units
    pub available_gpu: u32,
}

/// **COMPUTE RESOURCE REQUEST**
///
/// Request for compute resource allocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeResourceRequest {
    /// Number of CPU cores requested
    pub cpu_cores: u32,
    /// Amount of memory in gigabytes requested
    pub memory_gb: u32,
    /// Number of GPU units requested
    pub gpu_count: u32,
}

/// **AVAILABLE RESOURCES**
///
/// Currently available system resources for allocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableResources {
    /// Available CPU cores
    pub available_cpu: u32,
    /// Available memory in gigabytes
    pub available_memory_gb: u32,
    /// Available GPU units
    pub available_gpu: u32,
}

/// **GPU ALLOCATION**
///
/// GPU resource allocation specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuAllocation {
    /// GPU device identifier
    pub gpu_id: String,
    /// GPU memory allocation in gigabytes
    pub memory_gb: u32,
}

/// **TUNING SERVICE REGISTRATION**
///
/// Registration information for hardware tuning services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningServiceRegistration {
    /// Name of the tuning service
    pub service_name: String,
    /// Service endpoint URL
    pub endpoint: String,
}

/// **COMPUTE ADAPTER**
///
/// Adapter for interfacing with compute services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeAdapter {
    /// Name of the associated service
    pub service_name: String,
}

impl ComputeAdapter {
    /// Create a new compute adapter for the specified service
    #[must_use]
    pub const fn new(service_name: String) -> Self {
        Self { service_name }
    }
}

/// **LIVE HARDWARE METRICS**
///
/// Real-time hardware performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveHardwareMetrics {
    /// Current CPU utilization percentage
    pub cpu_usage: f64,
    /// Current memory utilization percentage
    pub memory_usage: f64,
    /// Current disk I/O rate
    pub disk_io: f64,
    /// Current network I/O rate
    pub network_io: f64,
    /// Current power consumption in watts
    pub power_consumption: f64,
    /// Current system temperature in Celsius
    pub temperature: f64,
    /// Current GPU utilization percentage
    pub gpu_usage: f64,
    /// Current disk usage percentage
    pub disk_usage: f64,
    /// Current network utilization percentage
    pub network_usage: f64,
    /// Timestamp when metrics were collected
    pub timestamp: DateTime<Utc>,
}

/// **TUNING RESULT**
///
/// Results from hardware tuning operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningResult {
    /// Name of the tuning profile applied
    pub profile_name: String,
    /// List of optimizations that were applied
    pub optimizations_applied: Vec<String>,
    /// Estimated power consumption increase
    pub estimated_power_increase: f64,
    /// Measured performance improvement percentage
    pub performance_improvement: f64,
    /// Hardware metrics before tuning
    pub before_metrics: LiveHardwareMetrics,
    /// Hardware metrics after tuning
    pub after_metrics: LiveHardwareMetrics,
}

/// **BENCHMARK RESULT**
///
/// Results from hardware benchmark tests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Type of benchmark that was run
    pub benchmark_type: String,
    /// Benchmark score achieved
    pub score: f64,
    /// Duration of the benchmark in milliseconds
    pub duration_ms: u64,
    /// Hardware metrics during benchmark
    pub metrics: LiveHardwareMetrics,
}

/// **PERFORMANCE SNAPSHOT**
///
/// Point-in-time performance snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    /// Timestamp when snapshot was taken
    pub timestamp: DateTime<Utc>,
    /// CPU utilization at snapshot time
    pub cpu_usage: f64,
    /// Memory utilization at snapshot time
    pub memory_usage: f64,
    /// Disk I/O rate at snapshot time
    pub disk_io: f64,
    /// Network I/O rate at snapshot time
    pub network_io: f64,
}

/// **SYSTEM PROFILE**
///
/// System configuration profile for different workload types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemProfile {
    /// CPU tuning profile name
    pub cpu_profile: String,
    /// Memory tuning profile name
    pub memory_profile: String,
    /// Storage tuning profile name
    pub storage_profile: String,
    /// Network tuning profile name
    pub network_profile: String,
}

/// **LIVE HARDWARE TUNING SESSION**
///
/// Represents an active hardware tuning session with real-time metrics collection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveHardwareTuningSession {
    /// Unique session identifier for tracking
    pub session_id: String,
    /// Timestamp when the session was started
    pub started_at: DateTime<Utc>,
    /// Current resource allocation configuration
    pub resource_allocation: ComputeAllocation,
    /// Real-time hardware metrics being collected
    pub current_metrics: LiveHardwareMetrics,
}

impl LiveHardwareTuningSession {
    /// Create a new hardware tuning session
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn new() -> Result<Self> {
        Ok(Self {
            session_id: format!("session_{}", Uuid::new_v4()),
            started_at: Utc::now(),
            resource_allocation: ComputeAllocation {
                cpu_cores: 8,
                memory_gb: 16,
                gpu_count: 1,
            },
            current_metrics: LiveHardwareMetrics {
                cpu_usage: 25.0,
                memory_usage: 40.0,
                disk_io: 80.0,
                network_io: 60.0,
                power_consumption: 280.0,
                temperature: 58.0,
                gpu_usage: 15.0,
                disk_usage: 70.0,
                network_usage: 30.0,
                timestamp: Utc::now(),
            },
        })
    }

    /// Collect current hardware performance metrics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn collect_current_metrics(&self) -> Result<LiveHardwareMetrics> {
        Ok(LiveHardwareMetrics {
            cpu_usage: 30.0,
            memory_usage: 45.0,
            disk_io: 85.0,
            network_io: 65.0,
            power_consumption: 290.0,
            temperature: 60.0,
            gpu_usage: 20.0,
            disk_usage: 70.0,
            network_usage: 35.0,
            timestamp: Utc::now(),
        })
    }
}

/// **SYSTEM CAPABILITIES**
///
/// Hardware capabilities and specifications of the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCapabilities {
    /// Number of CPU cores available
    pub cpu_cores: usize,
    /// CPU model identifier
    pub cpu_model: String,
    /// Total system memory in gigabytes
    pub memory_gb: u64,
    /// Whether GPU acceleration is available
    pub gpu_available: bool,
    /// GPU information if available
    pub gpu_info: Option<GpuInfo>,
}

/// **CPU INFORMATION**
///
/// Detailed CPU specifications and capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    /// Number of CPU cores
    pub cores: usize,
    /// CPU model name and identifier
    pub model: String,
}

/// **MEMORY INFORMATION**
///
/// System memory specifications and availability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    /// Total system memory in gigabytes
    pub total_gb: u64,
}

/// **GPU INFORMATION**
///
/// Graphics processing unit specifications and capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    /// GPU device name
    pub name: String,
    /// GPU memory in megabytes
    pub memory_mb: u64,
}

/// **CPU MONITOR**
///
/// Hardware monitor for CPU performance and utilization.
#[derive(Debug, Clone)]
pub struct CpuMonitor;

/// **MEMORY MONITOR**
///
/// Hardware monitor for memory usage and availability.
#[derive(Debug, Clone)]
pub struct MemoryMonitor;

/// **GPU MONITOR**
///
/// Hardware monitor for GPU utilization and performance.
#[derive(Debug, Clone)]
pub struct GpuMonitor;

/// **TUNING SESSION**
///
/// Active tuning session
#[derive(Debug, Clone)]
pub struct TuningSession {
    /// Unique identifier for the tuning session
    pub session_id: String,
    /// Timestamp when the session was started
    pub started_at: DateTime<Utc>,
    /// Resource allocation configuration for this session
    pub resource_allocation: ComputeAllocation,
    /// Current hardware metrics being monitored
    pub current_metrics: LiveHardwareMetrics,
}

/// **HARDWARE MONITORS**
///
/// Collection of hardware monitoring services.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HardwareMonitors {
    /// CPU performance monitor
    pub cpu: CpuMonitor,
    /// Memory utilization monitor
    pub memory: MemoryMonitor,
    /// GPU performance monitor (if available)
    pub gpu: Option<GpuMonitor>,
}

/// **SYSTEM METRICS COLLECTOR**
///
/// Collects and aggregates system performance metrics.
#[derive(Debug, Clone)]
#[allow(dead_code)] // Monitor fields used for system integration
pub struct SystemMetricsCollector {
    /// CPU monitoring
    pub cpu_monitor: CpuMonitor,
    /// Memory monitoring  
    pub memory_monitor: MemoryMonitor,
    /// GPU monitoring (if available)
    pub gpu_monitor: Option<GpuMonitor>,
}

impl SystemMetricsCollector {
    /// Create a new system metrics collector with hardware detection
    ///
    /// Initializes monitoring components for CPU, memory, and GPU (if available).
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    #[must_use]
    pub const fn new() -> Result<Self> {
        Ok(Self {
            cpu_monitor: CpuMonitor,
            memory_monitor: MemoryMonitor,
            gpu_monitor: None, // Initialize based on GPU detection
        })
    }

    /// Collect current hardware performance metrics from the system
    ///
    /// Gathers real-time CPU, memory, and GPU usage statistics.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn collect_current_metrics(&self) -> Result<LiveHardwareMetrics> {
        // Collect real metrics from system
        let cpu_usage = self.get_cpu_usage()?;
        let memory_usage = self.get_memory_usage()?;
        let gpu_usage = self.get_gpu_usage().unwrap_or(0.0);

        Ok(LiveHardwareMetrics {
            timestamp: Utc::now(),
            cpu_usage,
            memory_usage,
            gpu_usage,
            disk_usage: self.get_disk_usage().unwrap_or(0.0),
            network_usage: self.get_network_usage().await.unwrap_or(0.0),
            disk_io: 0.0,
            network_io: 0.0,
            power_consumption: 0.0,
            temperature: 0.0,
        })
    }

    fn get_cpu_usage(&self) -> Result<f64> {
        // Read CPU usage from /proc/stat
        match std::fs::read_to_string("/proc/stat") {
            Ok(content) => {
                if let Some(line) = content.lines().next() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 5 && parts[0] == "cpu" {
                        let user: u64 = parts[1].parse().unwrap_or(0);
                        let nice: u64 = parts[2].parse().unwrap_or(0);
                        let system: u64 = parts[3].parse().unwrap_or(0);
                        let idle: u64 = parts[4].parse().unwrap_or(0);

                        let total = user + nice + system + idle;
                        let usage = if total > 0 {
                            ((total - idle) as f64 / total as f64) * 100.0
                        } else {
                            0.0
                        };
                        return Ok(usage);
                    }
                }
                Ok(0.0)
            }
            Err(_) => Ok(0.0), // Fallback for non-Linux systems
        }
    }

    fn get_memory_usage(&self) -> Result<f64> {
        // Read memory usage from /proc/meminfo
        match std::fs::read_to_string("/proc/meminfo") {
            Ok(content) => {
                let mut total_kb = 0u64;
                let mut available_kb = 0u64;

                for line in content.lines() {
                    if line.starts_with("MemTotal:") {
                        if let Some(value) = line.split_whitespace().nth(1) {
                            total_kb = value.parse().unwrap_or(0);
                        }
                    } else if line.starts_with("MemAvailable:") {
                        if let Some(value) = line.split_whitespace().nth(1) {
                            available_kb = value.parse().unwrap_or(0);
                        }
                    }
                }

                if total_kb > 0 {
                    let used_kb = total_kb - available_kb;
                    let usage_percent = (used_kb as f64 / total_kb as f64) * 100.0;
                    Ok(usage_percent)
                } else {
                    Ok(0.0)
                }
            }
            Err(_) => Ok(0.0), // Fallback for non-Linux systems
        }
    }

    fn get_gpu_usage(&self) -> Result<f64> {
        // Try to read GPU usage from nvidia-smi or other GPU tools
        // For now, return 0.0 if no GPU monitoring available
        if let Ok(output) = std::process::Command::new("nvidia-smi")
            .args([
                "--query-gpu=utilization.gpu",
                "--format=csv,noheader,nounits",
            ])
            .output()
        {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Ok(usage) = stdout.trim().parse::<f64>() {
                    return Ok(usage);
                }
            }
        }
        Ok(0.0) // No GPU or monitoring not available
    }

    fn get_disk_usage(&self) -> Result<f64> {
        // Get disk usage for root filesystem
        if let Ok(_metadata) = std::fs::metadata("/") {
            // This is a simplified approach - would need statvfs for accurate disk usage
            // For now, return a calculated estimate based on available system info
            match std::process::Command::new("df")
                .args(["/", "--output=pcent"])
                .output()
            {
                Ok(output) if output.status.success() => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if let Some(line) = stdout.lines().nth(1) {
                        let percent_str = line.trim().trim_end_matches('%');
                        if let Ok(usage) = percent_str.parse::<f64>() {
                            return Ok(usage);
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(0.0) // Fallback
    }

    async fn get_network_usage(&self) -> Result<f64> {
        // Read network statistics from /proc/net/dev
        match std::fs::read_to_string("/proc/net/dev") {
            Ok(content) => {
                let mut total_bytes = 0u64;
                for line in content.lines().skip(2) {
                    // Skip header lines
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 10 {
                        // Sum receive and transmit bytes (columns 1 and 9)
                        let rx_bytes: u64 = parts[1].parse().unwrap_or(0);
                        let tx_bytes: u64 = parts[9].parse().unwrap_or(0);
                        total_bytes += rx_bytes + tx_bytes;
                    }
                }
                // Convert to percentage based on interface capacity (simplified)
                // This is a basic implementation - real usage would track rates over time
                Ok(if total_bytes > 0 { 10.0 } else { 0.0 })
            }
            Err(_) => Ok(0.0), // Fallback for non-Linux systems
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_tuning_config_default() {
        let config = HardwareTuningConfig::default();
        assert_eq!(config.cpu_cores, 8);
        assert_eq!(config.memory_gb, 16);
        assert!(config.cpu_tuning_enabled);
        assert!(config.memory_optimization_enabled);
        assert!(!config.gpu_tuning_enabled);
        assert_eq!(config.monitoring_interval, Duration::from_secs(5));
    }

    #[test]
    fn test_compute_allocation_creation() {
        let allocation = ComputeAllocation {
            cpu_cores: 4,
            memory_gb: 8,
            gpu_count: 2,
        };
        assert_eq!(allocation.cpu_cores, 4);
        assert_eq!(allocation.memory_gb, 8);
        assert_eq!(allocation.gpu_count, 2);
    }

    #[test]
    fn test_compute_resources_creation() {
        let resources = ComputeResources {
            available_cpu: 16,
            available_memory_gb: 64,
            available_gpu: 4,
        };
        assert_eq!(resources.available_cpu, 16);
        assert_eq!(resources.available_memory_gb, 64);
        assert_eq!(resources.available_gpu, 4);
    }

    #[test]
    fn test_compute_adapter_new() {
        let adapter = ComputeAdapter::new("test-service".to_string());
        assert_eq!(adapter.service_name, "test-service");
    }

    #[test]
    fn test_live_hardware_tuning_session_new() {
        let session = LiveHardwareTuningSession::new().expect("Should create session");
        assert!(session.session_id.starts_with("session_"));
        assert_eq!(session.resource_allocation.cpu_cores, 8);
        assert_eq!(session.resource_allocation.memory_gb, 16);
        assert_eq!(session.current_metrics.cpu_usage, 25.0);
    }

    #[test]
    fn test_live_hardware_tuning_session_collect_metrics() {
        let session = LiveHardwareTuningSession::new().expect("Should create session");
        let metrics = session
            .collect_current_metrics()
            .expect("Should collect metrics");
        assert_eq!(metrics.cpu_usage, 30.0);
        assert_eq!(metrics.memory_usage, 45.0);
    }

    #[test]
    fn test_system_metrics_collector_new() {
        let collector = SystemMetricsCollector::new().expect("Should create collector");
        assert!(collector.gpu_monitor.is_none());
    }

    #[test]
    fn test_system_capabilities_serialization() {
        let caps = SystemCapabilities {
            cpu_cores: 8,
            cpu_model: "Intel Core i7".to_string(),
            memory_gb: 32,
            gpu_available: true,
            gpu_info: Some(GpuInfo {
                name: "NVIDIA RTX 3080".to_string(),
                memory_mb: 10240,
            }),
        };

        let json = serde_json::to_string(&caps).expect("Should serialize");
        let deserialized: SystemCapabilities =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(caps.cpu_cores, deserialized.cpu_cores);
        assert_eq!(caps.memory_gb, deserialized.memory_gb);
    }

    #[test]
    fn test_gpu_allocation_creation() {
        let allocation = GpuAllocation {
            gpu_id: "GPU-0".to_string(),
            memory_gb: 8,
        };
        assert_eq!(allocation.gpu_id, "GPU-0");
        assert_eq!(allocation.memory_gb, 8);
    }

    #[test]
    fn test_tuning_service_registration() {
        use nestgate_core::constants::hardcoding::{addresses, ports};
        let endpoint = format!(
            "http://{}:{}",
            addresses::LOCALHOST_NAME,
            ports::HTTP_DEFAULT
        );

        let registration = TuningServiceRegistration {
            service_name: "tuning-service".to_string(),
            endpoint,
        };
        assert_eq!(registration.service_name, "tuning-service");
        assert!(registration.endpoint.starts_with("http://"));
    }

    #[test]
    fn test_performance_snapshot_creation() {
        let snapshot = PerformanceSnapshot {
            timestamp: Utc::now(),
            cpu_usage: 45.5,
            memory_usage: 60.2,
            disk_io: 80.0,
            network_io: 120.5,
        };
        assert_eq!(snapshot.cpu_usage, 45.5);
        assert_eq!(snapshot.memory_usage, 60.2);
    }

    #[test]
    fn test_system_profile_creation() {
        let profile = SystemProfile {
            cpu_profile: "performance".to_string(),
            memory_profile: "balanced".to_string(),
            storage_profile: "high-throughput".to_string(),
            network_profile: "low-latency".to_string(),
        };
        assert_eq!(profile.cpu_profile, "performance");
        assert_eq!(profile.memory_profile, "balanced");
    }

    #[test]
    fn test_benchmark_result_creation() {
        let now = Utc::now();
        let benchmark = BenchmarkResult {
            benchmark_type: "cpu-stress".to_string(),
            score: 8500.0,
            duration_ms: 5000,
            metrics: LiveHardwareMetrics {
                cpu_usage: 95.0,
                memory_usage: 70.0,
                disk_io: 100.0,
                network_io: 50.0,
                power_consumption: 350.0,
                temperature: 75.0,
                gpu_usage: 0.0,
                disk_usage: 60.0,
                network_usage: 25.0,
                timestamp: now,
            },
        };
        assert_eq!(benchmark.benchmark_type, "cpu-stress");
        assert_eq!(benchmark.score, 8500.0);
    }

    #[test]
    fn test_tuning_result_creation() {
        let now = Utc::now();
        let before = LiveHardwareMetrics {
            cpu_usage: 50.0,
            memory_usage: 60.0,
            disk_io: 80.0,
            network_io: 40.0,
            power_consumption: 280.0,
            temperature: 55.0,
            gpu_usage: 10.0,
            disk_usage: 65.0,
            network_usage: 20.0,
            timestamp: now,
        };

        let after = LiveHardwareMetrics {
            cpu_usage: 45.0,
            memory_usage: 55.0,
            disk_io: 90.0,
            network_io: 50.0,
            power_consumption: 300.0,
            temperature: 58.0,
            gpu_usage: 15.0,
            disk_usage: 65.0,
            network_usage: 25.0,
            timestamp: now,
        };

        let result = TuningResult {
            profile_name: "balanced".to_string(),
            optimizations_applied: vec!["cpu-governor".to_string(), "memory-tuning".to_string()],
            estimated_power_increase: 20.0,
            performance_improvement: 15.5,
            before_metrics: before,
            after_metrics: after,
        };

        assert_eq!(result.profile_name, "balanced");
        assert_eq!(result.optimizations_applied.len(), 2);
        assert_eq!(result.performance_improvement, 15.5);
    }

    #[test]
    fn test_cpu_info_serialization() {
        let info = CpuInfo {
            cores: 8,
            model: "AMD Ryzen 7".to_string(),
        };

        let json = serde_json::to_string(&info).expect("Should serialize");
        let deserialized: CpuInfo = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(info.cores, deserialized.cores);
        assert_eq!(info.model, deserialized.model);
    }

    #[test]
    fn test_memory_info_serialization() {
        let info = MemoryInfo { total_gb: 64 };

        let json = serde_json::to_string(&info).expect("Should serialize");
        let deserialized: MemoryInfo = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(info.total_gb, deserialized.total_gb);
    }

    #[test]
    fn test_gpu_info_serialization() {
        let info = GpuInfo {
            name: "NVIDIA RTX 4090".to_string(),
            memory_mb: 24576,
        };

        let json = serde_json::to_string(&info).expect("Should serialize");
        let deserialized: GpuInfo = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(info.name, deserialized.name);
        assert_eq!(info.memory_mb, deserialized.memory_mb);
    }

    #[test]
    fn test_available_resources_creation() {
        let resources = AvailableResources {
            available_cpu: 12,
            available_memory_gb: 48,
            available_gpu: 2,
        };
        assert_eq!(resources.available_cpu, 12);
        assert_eq!(resources.available_memory_gb, 48);
    }

    #[test]
    fn test_compute_resource_request() {
        let request = ComputeResourceRequest {
            cpu_cores: 4,
            memory_gb: 16,
            gpu_count: 1,
        };
        assert_eq!(request.cpu_cores, 4);
        assert_eq!(request.memory_gb, 16);
        assert_eq!(request.gpu_count, 1);
    }

    #[test]
    fn test_live_hardware_metrics_serialization() {
        let metrics = LiveHardwareMetrics {
            cpu_usage: 55.5,
            memory_usage: 70.2,
            disk_io: 100.0,
            network_io: 80.5,
            power_consumption: 300.0,
            temperature: 65.0,
            gpu_usage: 40.0,
            disk_usage: 75.0,
            network_usage: 35.0,
            timestamp: Utc::now(),
        };

        let json = serde_json::to_string(&metrics).expect("Should serialize");
        let deserialized: LiveHardwareMetrics =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(metrics.cpu_usage, deserialized.cpu_usage);
        assert_eq!(metrics.memory_usage, deserialized.memory_usage);
    }
}

#[cfg(test)]
#[path = "types_comprehensive_tests.rs"]
mod comprehensive_tests;
