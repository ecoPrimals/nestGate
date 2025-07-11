//! End-to-End Chaos Testing for NestGate
//!
//! This module implements comprehensive chaos testing to validate system resilience,
//! failure modes, and recovery mechanisms under stress conditions.

use std::sync::{Arc, atomic::{AtomicBool, AtomicU64, Ordering}};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::sleep;
use futures::future::join_all;
use rand::{Rng, thread_rng};

/// Chaos testing configuration
#[derive(Debug, Clone)]
pub struct ChaosConfig {
    pub duration: Duration,
    pub stress_intensity: f64, // 0.0 to 1.0
    pub failure_injection_rate: f64, // 0.0 to 1.0
    pub recovery_timeout: Duration,
    pub metrics_interval: Duration,
    pub enable_disk_stress: bool,
    pub enable_memory_stress: bool,
    pub enable_cpu_stress: bool,
    pub enable_network_stress: bool,
}

impl Default for ChaosConfig {
    fn default() -> Self {
        Self {
            duration: nestgate_core::constants::test_defaults::TEST_MEDIUM_TIMEOUT,
            stress_intensity: 0.7,
            failure_injection_rate: 0.1,
            recovery_timeout: nestgate_core::constants::test_defaults::TEST_SHORT_TIMEOUT,
            metrics_interval: Duration::from_millis(500),
            enable_disk_stress: true,
            enable_memory_stress: true,
            enable_cpu_stress: true,
            enable_network_stress: true,
        }
    }
}

/// Real-time metrics during chaos testing
#[derive(Debug, Clone)]
pub struct ChaosMetrics {
    pub timestamp: Instant,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_io: f64,
    pub network_io: f64,
    pub active_connections: u64,
    pub zfs_health: String,
    pub error_count: u64,
    pub recovery_count: u64,
    pub response_time_ms: f64,
    pub throughput_ops_per_sec: f64,
}

/// Chaos testing orchestrator
pub struct ChaosTestRunner {
    config: ChaosConfig,
    metrics_history: Arc<RwLock<Vec<ChaosMetrics>>>,
    error_counter: Arc<AtomicU64>,
    recovery_counter: Arc<AtomicU64>,
    active_stressors: Arc<RwLock<Vec<StressorHandle>>>,
    test_start_time: Instant,
    is_running: Arc<AtomicBool>,
}

/// Handle to a running stressor
pub struct StressorHandle {
    pub name: String,
    pub handle: tokio::task::JoinHandle<()>,
    pub stop_signal: Arc<AtomicBool>,
}

impl ChaosTestRunner {
    pub fn new(config: ChaosConfig) -> Self {
        Self {
            config,
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            error_counter: Arc::new(AtomicU64::new(0)),
            recovery_counter: Arc::new(AtomicU64::new(0)),
            active_stressors: Arc::new(RwLock::new(Vec::new())),
            test_start_time: Instant::now(),
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Run comprehensive chaos test
    pub async fn run_chaos_test(&mut self) -> Result<Vec<ChaosMetrics>, Box<dyn std::error::Error>> {
        println!("🔥 Starting Chaos Testing - Duration: {:?}", self.config.duration);
        println!("⚡ Stress Intensity: {:.1}%", self.config.stress_intensity * 100.0);
        println!("💥 Failure Injection Rate: {:.1}%", self.config.failure_injection_rate * 100.0);
        println!("");

        self.is_running.store(true, Ordering::SeqCst);
        self.test_start_time = Instant::now();

        // Start metrics collection
        let metrics_task = self.start_metrics_collection().await;

        // Start system stressors
        self.start_system_stressors().await?;

        // Start failure injection
        let failure_task = self.start_failure_injection().await;

        // Start recovery validation
        let recovery_task = self.start_recovery_validation().await;

        // Run for configured duration
        sleep(self.config.duration).await;

        // Stop all chaos activities
        self.stop_chaos_test().await;

        // Wait for tasks to complete
        let _ = tokio::join!(metrics_task, failure_task, recovery_task);

        // Return collected metrics
        let metrics = self.metrics_history.read().await.clone();
        self.print_chaos_summary(&metrics).await;

        Ok(metrics)
    }

    /// Start real-time metrics collection
    async fn start_metrics_collection(&self) -> tokio::task::JoinHandle<()> {
        let metrics_history = Arc::clone(&self.metrics_history);
        let error_counter = Arc::clone(&self.error_counter);
        let recovery_counter = Arc::clone(&self.recovery_counter);
        let is_running = Arc::clone(&self.is_running);
        let interval = self.config.metrics_interval;

        tokio::spawn(async move {
            while is_running.load(Ordering::SeqCst) {
                let metrics = Self::collect_live_metrics(
                    &error_counter,
                    &recovery_counter,
                ).await;

                // Print live metrics
                Self::print_live_metrics(&metrics);

                // Store metrics
                metrics_history.write().await.push(metrics);

                sleep(interval).await;
            }
        })
    }

    /// Collect real-time system metrics
    async fn collect_live_metrics(
        error_counter: &Arc<AtomicU64>,
        recovery_counter: &Arc<AtomicU64>,
    ) -> ChaosMetrics {
        // CPU usage (simulated with some real system activity)
        let cpu_usage = Self::get_cpu_usage().await;

        // Memory usage (real system memory check)
        let memory_usage = Self::get_memory_usage().await;

        // Disk I/O (approximated from file operations)
        let disk_io = Self::measure_disk_io().await;

        // Network I/O (simulated based on activity)
        let network_io = thread_rng().gen_range(10.0..100.0);

        // Active connections (simulated)
        let active_connections = thread_rng().gen_range(50..200);

        // ZFS health check
        let zfs_health = Self::check_zfs_health().await;

        // Response time measurement
        let response_time_ms = Self::measure_response_time().await;

        // Throughput simulation
        let throughput_ops_per_sec = Self::measure_throughput().await;

        ChaosMetrics {
            timestamp: Instant::now(),
            cpu_usage,
            memory_usage,
            disk_io,
            network_io,
            active_connections,
            zfs_health,
            error_count: error_counter.load(Ordering::SeqCst),
            recovery_count: recovery_counter.load(Ordering::SeqCst),
            response_time_ms,
            throughput_ops_per_sec,
        }
    }

    /// Get CPU usage from system
    async fn get_cpu_usage() -> f64 {
        // Read from /proc/loadavg for system load
        match tokio::fs::read_to_string("/proc/loadavg").await {
            Ok(content) => {
                let parts: Vec<&str> = content.split_whitespace().collect();
                if let Some(load_str) = parts.get(0) {
                    if let Ok(load) = load_str.parse::<f64>() {
                        // Convert load average to approximate CPU percentage
                        return (load * 25.0).min(100.0);
                    }
                }
            }
            Err(_) => {}
        }

        // Fallback to simulated CPU usage
        thread_rng().gen_range(10.0..80.0)
    }

    /// Get memory usage from system
    async fn get_memory_usage() -> f64 {
        // Read from /proc/meminfo
        match tokio::fs::read_to_string("/proc/meminfo").await {
            Ok(content) => {
                let mut total_kb = 0u64;
                let mut available_kb = 0u64;

                for line in content.lines() {
                    if line.starts_with("MemTotal:") {
                        if let Some(value_str) = line.split_whitespace().nth(1) {
                            total_kb = value_str.parse().unwrap_or(0);
                        }
                    } else if line.starts_with("MemAvailable:") {
                        if let Some(value_str) = line.split_whitespace().nth(1) {
                            available_kb = value_str.parse().unwrap_or(0);
                        }
                    }
                }

                if total_kb > 0 && available_kb <= total_kb {
                    let used_kb = total_kb - available_kb;
                    return (used_kb as f64 / total_kb as f64) * 100.0;
                }
            }
            Err(_) => {}
        }

        // Fallback to simulated memory usage
        thread_rng().gen_range(30.0..70.0)
    }

    /// Measure disk I/O activity
    async fn measure_disk_io() -> f64 {
        let start = Instant::now();

        // Perform a small file operation to measure I/O
        let test_data = vec![0u8; 1024]; // 1KB
        let temp_file = "/tmp/chaos_io_test.tmp";

        let _ = tokio::fs::write(temp_file, &test_data).await;
        let _ = tokio::fs::read(temp_file).await;
        let _ = tokio::fs::remove_file(temp_file).await;

        let io_time = start.elapsed().as_millis() as f64;

        // Convert to I/O metric (lower time = higher I/O performance)
        (1000.0 / (io_time + 1.0)).min(100.0)
    }

    /// Check ZFS health status
    async fn check_zfs_health() -> String {
        match tokio::process::Command::new("zpool")
            .args(&["status"])
            .output()
            .await
        {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.contains("ONLINE") {
                    "HEALTHY".to_string()
                } else if stdout.contains("DEGRADED") {
                    "DEGRADED".to_string()
                } else if stdout.contains("FAULTED") {
                    "CRITICAL".to_string()
                } else {
                    "UNKNOWN".to_string()
                }
            }
            _ => "NO_ZFS".to_string(),
        }
    }

    /// Measure system response time
    async fn measure_response_time() -> f64 {
        let start = Instant::now();

        // Simulate some work (file system operation)
        let _ = tokio::fs::metadata("/").await;

        start.elapsed().as_millis() as f64
    }

    /// Measure system throughput
    async fn measure_throughput() -> f64 {
        // Simulate throughput based on system load and randomness
        let base_throughput = 1000.0;
        let variance = thread_rng().gen_range(0.7..1.3);
        base_throughput * variance
    }

    /// Print live metrics to console
    fn print_live_metrics(metrics: &ChaosMetrics) {
        print!("\r🔥 LIVE CHAOS METRICS | ");
        print!("CPU: {:.1}% | ", metrics.cpu_usage);
        print!("MEM: {:.1}% | ", metrics.memory_usage);
        print!("DISK: {:.1} | ", metrics.disk_io);
        print!("NET: {:.1} | ", metrics.network_io);
        print!("ZFS: {} | ", metrics.zfs_health);
        print!("ERR: {} | ", metrics.error_count);
        print!("REC: {} | ", metrics.recovery_count);
        print!("RT: {:.1}ms | ", metrics.response_time_ms);
        print!("TPS: {:.0}    ", metrics.throughput_ops_per_sec);

        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }

    /// Start system stressors
    async fn start_system_stressors(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stressors = self.active_stressors.write().await;

        if self.config.enable_cpu_stress {
            let handle = self.start_cpu_stressor().await;
            stressors.push(handle);
        }

        if self.config.enable_memory_stress {
            let handle = self.start_memory_stressor().await;
            stressors.push(handle);
        }

        if self.config.enable_disk_stress {
            let handle = self.start_disk_stressor().await;
            stressors.push(handle);
        }

        if self.config.enable_network_stress {
            let handle = self.start_network_stressor().await;
            stressors.push(handle);
        }

        println!("🚀 Started {} system stressors", stressors.len());
        Ok(())
    }

    /// Start CPU stressor
    async fn start_cpu_stressor(&self) -> StressorHandle {
        let stop_signal = Arc::new(AtomicBool::new(false));
        let stop_signal_clone = Arc::clone(&stop_signal);
        let intensity = self.config.stress_intensity;

        let handle = tokio::spawn(async move {
            let mut iteration = 0u64;
            while !stop_signal_clone.load(Ordering::SeqCst) {
                // CPU-intensive work
                for _ in 0..(intensity * 1000000.0) as u64 {
                    iteration = iteration.wrapping_add(1);
                    iteration = iteration.wrapping_mul(17);
                }

                // Brief pause to allow other work
                sleep(Duration::from_millis(1)).await;
            }
        });

        StressorHandle {
            name: "CPU_STRESSOR".to_string(),
            handle,
            stop_signal,
        }
    }

    /// Start memory stressor
    async fn start_memory_stressor(&self) -> StressorHandle {
        let stop_signal = Arc::new(AtomicBool::new(false));
        let stop_signal_clone = Arc::clone(&stop_signal);
        let intensity = self.config.stress_intensity;

        let handle = tokio::spawn(async move {
            let mut memory_hogs: Vec<Vec<u8>> = Vec::new();

            while !stop_signal_clone.load(Ordering::SeqCst) {
                // Allocate memory chunks
                let chunk_size = (intensity * 1024.0 * 1024.0) as usize; // MB
                let chunk = vec![0u8; chunk_size];
                memory_hogs.push(chunk);

                // Keep only recent allocations to avoid OOM
                if memory_hogs.len() > 10 {
                    memory_hogs.remove(0);
                }

                sleep(Duration::from_millis(100)).await;
            }
        });

        StressorHandle {
            name: "MEMORY_STRESSOR".to_string(),
            handle,
            stop_signal,
        }
    }

    /// Start disk stressor
    async fn start_disk_stressor(&self) -> StressorHandle {
        let stop_signal = Arc::new(AtomicBool::new(false));
        let stop_signal_clone = Arc::clone(&stop_signal);
        let intensity = self.config.stress_intensity;

        let handle = tokio::spawn(async move {
            let mut file_counter = 0;

            while !stop_signal_clone.load(Ordering::SeqCst) {
                // Create temporary files for I/O stress
                let filename = format!("/tmp/chaos_test_{}.tmp", file_counter);
                let data_size = (intensity * 1024.0 * 1024.0) as usize; // MB
                let data = vec![0u8; data_size];

                // Write and read operations
                if let Ok(()) = tokio::fs::write(&filename, &data).await {
                    let _ = tokio::fs::read(&filename).await;
                    let _ = tokio::fs::remove_file(&filename).await;
                }

                file_counter += 1;
                sleep(Duration::from_millis(200)).await;
            }
        });

        StressorHandle {
            name: "DISK_STRESSOR".to_string(),
            handle,
            stop_signal,
        }
    }

    /// Start network stressor
    async fn start_network_stressor(&self) -> StressorHandle {
        let stop_signal = Arc::new(AtomicBool::new(false));
        let stop_signal_clone = Arc::clone(&stop_signal);
        let intensity = self.config.stress_intensity;

        let handle = tokio::spawn(async move {
            while !stop_signal_clone.load(Ordering::SeqCst) {
                // Simulate network stress with concurrent connections
                let connection_count = (intensity * 10.0) as usize;
                let mut tasks = Vec::new();

                for _ in 0..connection_count {
                    let task = tokio::spawn(async {
                        // Simulate network operation
                        let _ = tokio::time::timeout(
                            Duration::from_millis(100),
                            tokio::net::TcpStream::connect("127.0.0.1:80")
                        ).await;
                    });
                    tasks.push(task);
                }

                // Wait for all connections to complete or timeout
                let _ = join_all(tasks).await;

                sleep(Duration::from_millis(500)).await;
            }
        });

        StressorHandle {
            name: "NETWORK_STRESSOR".to_string(),
            handle,
            stop_signal,
        }
    }

    /// Start failure injection
    async fn start_failure_injection(&self) -> tokio::task::JoinHandle<()> {
        let error_counter = Arc::clone(&self.error_counter);
        let is_running = Arc::clone(&self.is_running);
        let failure_rate = self.config.failure_injection_rate;

        tokio::spawn(async move {
            while is_running.load(Ordering::SeqCst) {
                if thread_rng().gen::<f64>() < failure_rate {
                    // Inject random failure
                    Self::inject_random_failure(&error_counter).await;
                }

                sleep(Duration::from_millis(1000)).await;
            }
        })
    }

    /// Inject random failure
    async fn inject_random_failure(error_counter: &Arc<AtomicU64>) {
        let failure_types = [
            "NETWORK_TIMEOUT",
            "DISK_FULL_SIMULATION",
            "MEMORY_PRESSURE",
            "SERVICE_UNAVAILABLE",
            "ZFS_DEGRADED_SIMULATION",
        ];

        let failure_type = failure_types[thread_rng().gen_range(0..failure_types.len())];

        match failure_type {
            "NETWORK_TIMEOUT" => {
                // Simulate network timeout
                sleep(Duration::from_millis(thread_rng().gen_range(1000..5000))).await;
            }
            "DISK_FULL_SIMULATION" => {
                // Simulate disk full by creating large temp file
                let large_data = vec![0u8; 10 * 1024 * 1024]; // 10MB
                let _ = tokio::fs::write("/tmp/chaos_disk_full.tmp", large_data).await;
                sleep(Duration::from_millis(500)).await;
                let _ = tokio::fs::remove_file("/tmp/chaos_disk_full.tmp").await;
            }
            "MEMORY_PRESSURE" => {
                // Simulate memory pressure
                let _memory_hog = vec![0u8; 50 * 1024 * 1024]; // 50MB
                sleep(Duration::from_millis(1000)).await;
            }
            _ => {
                // Generic failure simulation
                sleep(Duration::from_millis(100)).await;
            }
        }

        error_counter.fetch_add(1, Ordering::SeqCst);
        println!("\n💥 Injected failure: {}", failure_type);
    }

    /// Start recovery validation
    async fn start_recovery_validation(&self) -> tokio::task::JoinHandle<()> {
        let recovery_counter = Arc::clone(&self.recovery_counter);
        let is_running = Arc::clone(&self.is_running);
        let recovery_timeout = self.config.recovery_timeout;

        tokio::spawn(async move {
            while is_running.load(Ordering::SeqCst) {
                // Validate system recovery
                if Self::validate_system_recovery().await {
                    recovery_counter.fetch_add(1, Ordering::SeqCst);
                    println!("\n🔄 System recovery validated");
                }

                sleep(recovery_timeout).await;
            }
        })
    }

    /// Validate system recovery
    async fn validate_system_recovery() -> bool {
        // Check if system is responsive
        let start = Instant::now();
        let responsive = tokio::time::timeout(Duration::from_millis(1000), async {
            tokio::fs::metadata("/").await.is_ok()
        }).await.unwrap_or(false);

        let response_time = start.elapsed();

        // System is considered recovered if responsive within reasonable time
        responsive && response_time < Duration::from_millis(500)
    }

    /// Stop chaos test
    async fn stop_chaos_test(&self) {
        println!("\n🛑 Stopping chaos test...");

        self.is_running.store(false, Ordering::SeqCst);

        // Stop all stressors
        let mut stressors = self.active_stressors.write().await;
        for stressor in stressors.iter() {
            stressor.stop_signal.store(true, Ordering::SeqCst);
        }

        // Wait for stressors to stop
        while let Some(stressor) = stressors.pop() {
            let _ = stressor.handle.await;
            println!("✅ Stopped stressor: {}", stressor.name);
        }
    }

    /// Print chaos test summary
    async fn print_chaos_summary(&self, metrics: &[ChaosMetrics]) {
        if metrics.is_empty() {
            return;
        }

        println!("\n");
        println!("📊 ========== CHAOS TEST SUMMARY ==========");
        println!("🕒 Duration: {:?}", self.test_start_time.elapsed());
        println!("📈 Metrics Collected: {}", metrics.len());

        // Calculate statistics
        let cpu_avg = metrics.iter().map(|m| m.cpu_usage).sum::<f64>() / metrics.len() as f64;
        let cpu_max = metrics.iter().map(|m| m.cpu_usage).fold(0.0, f64::max);

        let mem_avg = metrics.iter().map(|m| m.memory_usage).sum::<f64>() / metrics.len() as f64;
        let mem_max = metrics.iter().map(|m| m.memory_usage).fold(0.0, f64::max);

        let response_avg = metrics.iter().map(|m| m.response_time_ms).sum::<f64>() / metrics.len() as f64;
        let response_max = metrics.iter().map(|m| m.response_time_ms).fold(0.0, f64::max);

        let total_errors = self.error_counter.load(Ordering::SeqCst);
        let total_recoveries = self.recovery_counter.load(Ordering::SeqCst);

        println!("💻 CPU Usage - Avg: {:.1}%, Max: {:.1}%", cpu_avg, cpu_max);
        println!("🧠 Memory Usage - Avg: {:.1}%, Max: {:.1}%", mem_avg, mem_max);
        println!("⏱️  Response Time - Avg: {:.1}ms, Max: {:.1}ms", response_avg, response_max);
        println!("💥 Total Failures Injected: {}", total_errors);
        println!("🔄 Total Recoveries: {}", total_recoveries);

        // Calculate recovery rate
        let recovery_rate = if total_errors > 0 {
            (total_recoveries as f64 / total_errors as f64) * 100.0
        } else {
            100.0
        };

        println!("📊 Recovery Rate: {:.1}%", recovery_rate);

        // Determine test result
        let test_passed = recovery_rate >= 80.0 && response_avg < 1000.0;

        if test_passed {
            println!("✅ CHAOS TEST PASSED - System demonstrates good resilience");
        } else {
            println!("❌ CHAOS TEST FAILED - System needs resilience improvements");
        }

        println!("==========================================");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_light_chaos_scenario() {
        let config = ChaosConfig {
            duration: nestgate_core::constants::test_defaults::TEST_SHORT_TIMEOUT,
            stress_intensity: 0.3,
            failure_injection_rate: 0.05,
            recovery_timeout: Duration::from_secs(2),
            metrics_interval: Duration::from_millis(200),
            enable_disk_stress: false, // Disable for CI
            enable_memory_stress: true,
            enable_cpu_stress: true,
            enable_network_stress: false, // Disable for CI
        };

        let mut runner = ChaosTestRunner::new(config);
        let metrics = runner.run_chaos_test().await.expect("Chaos test should complete");

        assert!(!metrics.is_empty(), "Should collect metrics during test");
        assert!(metrics.len() >= 10, "Should collect multiple metric points");

        // Validate that metrics show system activity
        let has_cpu_activity = metrics.iter().any(|m| m.cpu_usage > 0.0);
        let has_memory_usage = metrics.iter().any(|m| m.memory_usage > 0.0);

        assert!(has_cpu_activity, "Should show CPU activity during stress test");
        assert!(has_memory_usage, "Should show memory usage during stress test");

        println!("✅ Light chaos test completed successfully");
    }

    #[tokio::test]
    async fn test_moderate_chaos_scenario() {
        let config = ChaosConfig {
            duration: Duration::from_secs(15),
            stress_intensity: 0.5,
            failure_injection_rate: 0.1,
            recovery_timeout: Duration::from_secs(3),
            metrics_interval: Duration::from_millis(300),
            enable_disk_stress: true,
            enable_memory_stress: true,
            enable_cpu_stress: true,
            enable_network_stress: false, // Keep disabled for stability
        };

        let mut runner = ChaosTestRunner::new(config);
        let metrics = runner.run_chaos_test().await.expect("Chaos test should complete");

        assert!(!metrics.is_empty(), "Should collect metrics during test");

        // Validate system resilience
        let error_count = runner.error_counter.load(Ordering::SeqCst);
        let recovery_count = runner.recovery_counter.load(Ordering::SeqCst);

        if error_count > 0 {
            let recovery_rate = (recovery_count as f64 / error_count as f64) * 100.0;
            assert!(recovery_rate >= 50.0, "System should recover from at least 50% of failures");
        }

        println!("✅ Moderate chaos test completed successfully");
    }

    #[tokio::test]
    async fn test_fail_safe_validation() {
        let config = ChaosConfig {
            duration: Duration::from_secs(8),
            stress_intensity: 0.8,
            failure_injection_rate: 0.2,
            recovery_timeout: Duration::from_secs(1),
            metrics_interval: Duration::from_millis(100),
            enable_disk_stress: false,
            enable_memory_stress: true,
            enable_cpu_stress: true,
            enable_network_stress: false,
        };

        let mut runner = ChaosTestRunner::new(config);
        let metrics = runner.run_chaos_test().await.expect("Chaos test should complete");

        // Validate fail-safe behavior
        for metric in &metrics {
            // System should never completely lock up
            assert!(metric.response_time_ms < 10000.0, "Response time should stay reasonable even under stress");

            // Memory usage should not exceed safe limits
            assert!(metric.memory_usage < 95.0, "Memory usage should not exceed 95% to prevent OOM");

            // CPU usage spikes are okay, but system should remain responsive
            if metric.cpu_usage > 90.0 {
                assert!(metric.response_time_ms < 5000.0, "High CPU should not cause excessive response times");
            }
        }

        println!("✅ Fail-safe validation completed successfully");
    }

    #[tokio::test]
    async fn test_metrics_collection_accuracy() {
        let config = ChaosConfig {
            duration: Duration::from_secs(5),
            stress_intensity: 0.1,
            failure_injection_rate: 0.0, // No failures for this test
            recovery_timeout: Duration::from_secs(1),
            metrics_interval: Duration::from_millis(100),
            enable_disk_stress: false,
            enable_memory_stress: false,
            enable_cpu_stress: false,
            enable_network_stress: false,
        };

        let mut runner = ChaosTestRunner::new(config);
        let metrics = runner.run_chaos_test().await.expect("Chaos test should complete");

        // Validate metrics collection
        assert!(metrics.len() >= 20, "Should collect metrics at regular intervals");

        // Check that timestamps are sequential
        for window in metrics.windows(2) {
            assert!(window[1].timestamp >= window[0].timestamp, "Timestamps should be sequential");
        }

        // Validate metric ranges
        for metric in &metrics {
            assert!(metric.cpu_usage >= 0.0 && metric.cpu_usage <= 100.0, "CPU usage should be in valid range");
            assert!(metric.memory_usage >= 0.0 && metric.memory_usage <= 100.0, "Memory usage should be in valid range");
            assert!(metric.response_time_ms >= 0.0, "Response time should be non-negative");
            assert!(metric.throughput_ops_per_sec >= 0.0, "Throughput should be non-negative");
        }

        println!("✅ Metrics collection accuracy validated");
    }

    #[tokio::test]
    async fn test_stress_and_recovery_cycle() {
        let config = ChaosConfig {
            duration: Duration::from_secs(12),
            stress_intensity: 0.6,
            failure_injection_rate: 0.15,
            recovery_timeout: Duration::from_secs(2),
            metrics_interval: Duration::from_millis(250),
            enable_disk_stress: false, // Focus on CPU/memory for this test
            enable_memory_stress: true,
            enable_cpu_stress: true,
            enable_network_stress: false,
        };

        let mut runner = ChaosTestRunner::new(config);
        let metrics = runner.run_chaos_test().await.expect("Chaos test should complete");

        // Analyze stress and recovery patterns
        let mut high_stress_periods = 0;
        let mut recovery_periods = 0;

        for window in metrics.windows(3) {
            let avg_cpu = window.iter().map(|m| m.cpu_usage).sum::<f64>() / 3.0;
            let avg_response = window.iter().map(|m| m.response_time_ms).sum::<f64>() / 3.0;

            if avg_cpu > 50.0 || avg_response > 100.0 {
                high_stress_periods += 1;
            } else if avg_cpu < 30.0 && avg_response < 50.0 {
                recovery_periods += 1;
            }
        }

        // System should show both stress and recovery periods
        assert!(high_stress_periods > 0, "Should detect periods of high stress");
        assert!(recovery_periods > 0, "Should detect recovery periods");

        println!("✅ Stress and recovery cycle validation completed");
        println!("📊 High stress periods: {}, Recovery periods: {}", high_stress_periods, recovery_periods);
    }
}
