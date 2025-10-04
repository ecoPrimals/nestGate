/// System Introspection Module
/// Handles system introspection and auto-detection including:
/// - System capability detection
/// - Hardware profiling and resource limits
/// - Runtime environment analysis
/// - Optimal configuration recommendations
use crate::{NestGateError, Result};
use std::collections::HashMap;
/// System capabilities profile
#[derive(Debug, Clone)]
pub struct SystemCapabilities {
    pub cpu_cores: usize,
    pub logical_cores: usize,
    pub memory_gb: f64,
    pub network_interfaces: Vec<String>,
    pub storage_available: bool,
    pub container_runtime: Option<String>,
    pub os_type: String,
}
/// Hardware performance profile
#[derive(Debug, Clone)]
pub struct HardwareProfile {
    pub cpu_score: f64,
    pub memory_score: f64,
    pub storage_score: f64,
    pub network_score: f64,
    pub overall_score: f64,
    pub recommended_limits: HashMap<String, usize>,
}
/// System introspection subsystem
#[derive(Debug)]
#[allow(dead_code)]
pub struct SystemIntrospection {
    capabilities: Option<SystemCapabilities>,
    hardware_profile: Option<HardwareProfile>,
}
impl Default for SystemIntrospection {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemIntrospection {
    /// Create new system introspection subsystem
    #[must_use]
    pub fn new() -> Self {
        Self {
            capabilities: None,
            hardware_profile: None,
        }
    }

    /// **SYSTEM ANALYSIS**: Discover resource limits through system analysis
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn discover_resource_limits(&mut self, resource_type: &str) -> Result<usize> {
        // Ensure we have system capabilities
        if self.capabilities.is_none() {
            self.capabilities = Some(self.detect_system_capabilities().await?);
        }

        let capabilities = self.capabilities.as_ref().ok_or_else(|| {
            NestGateError::internal_error(
                "Capabilities not initialized after detection attempt",
                "introspection",
            )
        })?;

        match resource_type {
            "connections" => {
                // Base on memory and CPU cores
                let base_connections = capabilities.logical_cores * 100;
                let memory_factor = (capabilities.memory_gb * 50.0) as usize;
                Ok(base_connections.max(memory_factor).min(10000))
            }
            "threads" => {
                // Optimal thread count based on workload type
                Ok((capabilities.logical_cores * 2).clamp(4, 64))
            }
            "memory_buffer" => {
                // Buffer size based on available memory
                let buffer_mb = (capabilities.memory_gb * 0.1) as usize; // 10% of memory
                Ok((buffer_mb * 1024 * 1024).clamp(4096, 64 * 1024 * 1024)) // 4KB to 64MB
            }
            "file_handles" => {
                // File handles based on system limits
                Ok(self.get_system_file_limit().await.unwrap_or(1024))
            }
            "queue_size" => {
                // Queue size based on memory and expected load
                let base_queue = (capabilities.memory_gb * 1000.0) as usize;
                Ok(base_queue.clamp(1000, 100_000))
            }
            _ => {
                // Generic limit based on system capacity
                Ok((capabilities.logical_cores * 1000).clamp(1000, 50000))
            }
        }
    }

    /// **CAPABILITY DETECTION**: Detect system capabilities
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn detect_system_capabilities(&self) -> Result<SystemCapabilities> {
        Ok(SystemCapabilities {
            cpu_cores: num_cpus::get_physical(),
            logical_cores: num_cpus::get(),
            memory_gb: self.estimate_memory_gb().await?,
            network_interfaces: self.detect_network_interfaces().await?,
            storage_available: self.check_storage_availability().await?,
            container_runtime: self.detect_container_runtime().await,
            os_type: self.detect_os_type(),
        })
    }

    /// **HARDWARE PROFILING**: Create hardware performance profile
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn create_hardware_profile(&mut self) -> Result<HardwareProfile> {
        // Ensure we have capabilities
        if self.capabilities.is_none() {
            self.capabilities = Some(self.detect_system_capabilities().await?);
        }

        let capabilities = self.capabilities.as_ref().ok_or_else(|| {
            NestGateError::internal_error(
                "Capabilities not initialized after detection attempt",
                "introspection",
            )
        })?;

        // Score components (0.0 to 1.0)
        let cpu_score = self.calculate_cpu_score(capabilities).await?;
        let memory_score = self.calculate_memory_score(capabilities).await?;
        let storage_score = self.calculate_storage_score().await?;
        let network_score = self.calculate_network_score(capabilities).await?;

        let overall_score = (cpu_score + memory_score + storage_score + network_score) / 4.0;

        // Generate recommended limits based on scores
        let mut recommended_limits = HashMap::new();
        recommended_limits.insert(
            "max_connections".to_string(),
            ((overall_score * 5000.0) as usize).clamp(100, 10000),
        );
        recommended_limits.insert(
            "worker_threads".to_string(),
            ((cpu_score * capabilities.logical_cores as f64 * 2.0) as usize).clamp(2, 32),
        );
        recommended_limits.insert(
            "buffer_size".to_string(),
            ((memory_score * 32768.0) as usize).clamp(4096, 65536),
        );

        Ok(HardwareProfile {
            cpu_score,
            memory_score,
            storage_score,
            network_score,
            overall_score,
            recommended_limits,
        })
    }

    /// **MEMORY ESTIMATION**: Estimate available memory
    async fn estimate_memory_gb(&self) -> Result<f64> {
        // Simplified estimation - in a real system would use proper system APIs
        if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
            // Parse /proc/meminfo on Linux systems
            for line in meminfo.lines() {
                if line.starts_with("MemTotal:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = kb_str.parse::<u64>() {
                            return Ok(kb as f64 / 1024.0 / 1024.0); // Convert KB to GB
                        }
                    }
                }
            }
        }

        // Fallback estimation based on environment
        if std::env::var("KUBERNETES_NAMESPACE").is_ok() {
            Ok(2.0) // Assume 2GB in containerized environment
        } else {
            Ok(8.0) // Assume 8GB for desktop/server environment
        }
    }

    /// **NETWORK INTERFACE DETECTION**: Detect available network interfaces
    async fn detect_network_interfaces(&self) -> Result<Vec<String>> {
        // Simplified implementation - in a real system would use proper network APIs
        let mut interfaces = vec!["lo".to_string()]; // Always have loopback

        // Common interface names
        let common_interfaces = vec!["eth0", "wlan0", "en0", "ens33"];
        for interface in common_interfaces {
            // In a real implementation, would check if interface actually exists
            interfaces.push(interface.to_string());
        }

        Ok(interfaces)
    }

    /// **STORAGE AVAILABILITY**: Check storage availability
    async fn check_storage_availability(&self) -> Result<bool> {
        // Check if we can create temporary files (basic storage test)
        match std::env::temp_dir().try_exists() {
            Ok(exists) => Ok(exists),
            Err(_) => Ok(false),
        }
    }

    /// **CONTAINER RUNTIME DETECTION**: Detect container runtime using capability-based approach
    async fn detect_container_runtime(&self) -> Option<String> {
        // Modern capability-based detection (preferred)
        if let Ok(compute_type) = std::env::var("COMPUTE_CAPABILITY_TYPE") {
            tracing::info!(
                "Using modern capability-based compute detection: {}",
                compute_type
            );
            return Some(format!("capability-based-{compute_type}"));
        }

        // DEPRECATED: Legacy vendor-specific detection patterns
        // These will be removed in version 4.0.0

        // DEPRECATED: Kubernetes namespace detection
        if std::env::var("KUBERNETES_NAMESPACE").is_ok() {
            tracing::warn!(
                "DEPRECATED: KUBERNETES_NAMESPACE detected for container runtime detection. \
                Please migrate to COMPUTE_CAPABILITY_TYPE=orchestrated. \
                This Kubernetes-specific detection will be removed in version 4.0.0."
            );
            return Some("legacy-orchestrated".to_string());
        }

        // DEPRECATED: Docker Compose detection
        if std::env::var("DOCKER_COMPOSE_PROJECT").is_ok() {
            tracing::warn!(
                "DEPRECATED: DOCKER_COMPOSE_PROJECT detected for container runtime detection. \
                Please migrate to COMPUTE_CAPABILITY_TYPE=containerized. \
                This Docker-specific detection will be removed in version 4.0.0."
            );
            return Some("legacy-containerized".to_string());
        }

        // DEPRECATED: Docker environment file detection
        if std::path::Path::new("/.dockerenv").exists() {
            tracing::warn!(
                "DEPRECATED: /.dockerenv file detected for container runtime detection. \
                Please migrate to COMPUTE_CAPABILITY_TYPE=containerized. \
                This Docker-specific detection will be removed in version 4.0.0."
            );
            return Some("legacy-containerized".to_string());
        }

        // No containerization detected
        None
    }

    /// **OS TYPE DETECTION**: Detect operating system type
    fn detect_os_type(&self) -> String {
        std::env::consts::OS.to_string()
    }

    /// **FILE LIMIT DETECTION**: Get system file descriptor limit
    async fn get_system_file_limit(&self) -> Result<usize> {
        // Try to read from system files or environment
        if let Ok(limit_str) = std::env::var("NESTGATE_MAX_FILE_HANDLES") {
            if let Ok(limit) = limit_str.parse::<usize>() {
                return Ok(limit);
            }
        }

        // Default reasonable limit
        Ok(1024)
    }

    /// **CPU SCORING**: Calculate CPU performance score
    async fn calculate_cpu_score(&self, capabilities: &SystemCapabilities) -> Result<f64> {
        // Score based on logical cores (0.1 to 1.0)
        let core_score = (capabilities.logical_cores as f64 / 16.0).clamp(0.1, 1.0);
        Ok(core_score)
    }

    /// **MEMORY SCORING**: Calculate memory performance score
    async fn calculate_memory_score(&self, capabilities: &SystemCapabilities) -> Result<f64> {
        // Score based on memory GB (0.1 to 1.0)
        let memory_score = (capabilities.memory_gb / 32.0).clamp(0.1, 1.0);
        Ok(memory_score)
    }

    /// **STORAGE SCORING**: Calculate storage performance score
    async fn calculate_storage_score(&self) -> Result<f64> {
        // Simplified storage scoring - in a real system would benchmark I/O
        if std::path::Path::new("/sys/block").exists() {
            // Assume SSD if on modern Linux system
            Ok(0.8)
        } else {
            // Conservative estimate
            Ok(0.5)
        }
    }

    /// **NETWORK SCORING**: Calculate network performance score
    async fn calculate_network_score(&self, capabilities: &SystemCapabilities) -> Result<f64> {
        // Score based on number of interfaces and environment
        let interface_score = (capabilities.network_interfaces.len() as f64 / 4.0).clamp(0.1, 1.0);

        // Bonus for containerized environments (usually have better networking)
        let environment_bonus = if capabilities.container_runtime.is_some() {
            0.2
        } else {
            0.0
        };

        Ok((interface_score + environment_bonus).min(1.0))
    }

    /// **INTROSPECTION SUMMARY**: Get system introspection summary
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    #[must_use]
    pub async fn get_introspection_summary(&mut self) -> Result<HashMap<String, String>> {
        let capabilities = if let Some(caps) = &self.capabilities {
            caps.clone()
        } else {
            let caps = self.detect_system_capabilities().await?;
            self.capabilities = Some(caps.clone());
            caps
        };

        let mut summary = HashMap::new();

        summary.insert("cpu_cores".to_string(), capabilities.cpu_cores.to_string());
        summary.insert(
            "logical_cores".to_string(),
            capabilities.logical_cores.to_string(),
        );
        summary.insert(
            "memory_gb".to_string(),
            format!("{:.1}", capabilities.memory_gb),
        );
        summary.insert(
            "network_interfaces".to_string(),
            capabilities.network_interfaces.len().to_string(),
        );
        summary.insert(
            "storage_available".to_string(),
            capabilities.storage_available.to_string(),
        );
        summary.insert(
            "container_runtime".to_string(),
            capabilities
                .container_runtime
                .unwrap_or_else(|| "native".to_string()),
        );
        summary.insert("os_type".to_string(), capabilities.os_type);

        Ok(summary)
    }
}
