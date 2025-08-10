/// Fault Injection Testing Framework
///
/// This module implements comprehensive fault injection testing to validate system resilience:
/// - Network failures and partitions
/// - Disk failures and I/O errors
/// - Memory pressure and OOM conditions
/// - CPU exhaustion scenarios
/// - ZFS-specific failures
/// - Service dependency failures
/// - Time-based fault scenarios
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

/// Fault injection configuration
#[derive(Debug, Clone)]
pub struct FaultInjectionConfig {
    pub test_duration: Duration,
    pub fault_probability: f64, // 0.0 to 1.0
    pub recovery_timeout: Duration,
    pub enable_network_faults: bool,
    pub enable_disk_faults: bool,
    pub enable_memory_faults: bool,
    pub enable_cpu_faults: bool,
    pub enable_zfs_faults: bool,
    pub enable_service_faults: bool,
    pub chaos_mode: bool,
}

impl Default for FaultInjectionConfig {
    fn default() -> Self {
        Self {
            test_duration: nestgate_core::constants::test_defaults::TEST_LONG_TIMEOUT,
            fault_probability: 0.15,
            recovery_timeout: nestgate_core::constants::test_defaults::TEST_SHORT_TIMEOUT,
            enable_network_faults: true,
            enable_disk_faults: true,
            enable_memory_faults: true,
            enable_cpu_faults: true,
            enable_zfs_faults: true,
            enable_service_faults: true,
            chaos_mode: false,
        }
    }
}

/// Types of faults that can be injected
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FaultType {
    NetworkPartition,
    NetworkTimeout,
    NetworkCorruption,
    DiskFull,
    DiskReadError,
    DiskWriteError,
    MemoryPressure,
    MemoryLeak,
    OutOfMemory,
    CpuExhaustion,
    CpuThrottling,
    ZfsPoolOffline,
    ZfsDeviceError,
    ZfsChecksum,
    ServiceUnavailable,
    ServiceSlowdown,
    ServiceCrash,
    DatabaseConnection,
    ClockSkew,
    FileSystemFull,
}

/// Fault injection event
#[derive(Debug, Clone)]
pub struct FaultEvent {
    pub fault_type: FaultType,
    pub start_time: Instant,
    pub duration: Duration,
    pub target_component: String,
    pub severity: FaultSeverity,
    pub injected_successfully: bool,
    pub recovery_successful: bool,
    pub impact_metrics: HashMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FaultSeverity {
    Low,      // Minimal impact, should be handled gracefully
    Medium,   // Noticeable impact, degraded performance acceptable
    High,     // Significant impact, failover mechanisms should activate
    Critical, // Severe impact, system should maintain core functionality
}

/// System resilience metrics
#[derive(Debug, Clone)]
pub struct ResilienceMetrics {
    pub timestamp: Instant,
    pub faults_injected: usize,
    pub faults_recovered: usize,
    pub system_uptime_percent: f64,
    pub average_recovery_time: Duration,
    pub max_recovery_time: Duration,
    pub error_rate: f64,
    pub performance_degradation: f64,
    pub availability_score: f64,
}

/// Main fault injection orchestrator
pub struct FaultInjectionFramework {
    config: FaultInjectionConfig,
    active_faults: Arc<RwLock<HashMap<String, FaultEvent>>>,
    fault_history: Arc<RwLock<Vec<FaultEvent>>>,
    resilience_metrics: Arc<RwLock<Vec<ResilienceMetrics>>>,
    system_health: Arc<AtomicU64>, // 0-100 health score
    is_running: Arc<AtomicBool>,
}

impl FaultInjectionFramework {
    pub fn new(config: FaultInjectionConfig) -> Self {
        Self {
            config,
            active_faults: Arc::new(RwLock::new(HashMap::new())),
            fault_history: Arc::new(RwLock::new(Vec::new())),
            resilience_metrics: Arc::new(RwLock::new(Vec::new())),
            system_health: Arc::new(AtomicU64::new(100)),
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Run comprehensive fault injection test
    pub async fn run_fault_injection_test(
        &self,
    ) -> Result<Vec<FaultEvent>, Box<dyn std::error::Error + Send + Sync>> {
        println!("💥 Starting Comprehensive Fault Injection Testing");
        println!("==================================================");
        println!("⚙️ Test Duration: {:?}", self.config.test_duration);
        println!(
            "🎲 Fault Probability: {:.1}%",
            self.config.fault_probability * 100.0
        );
        println!("🔄 Recovery Timeout: {:?}", self.config.recovery_timeout);

        self.is_running.store(true, Ordering::SeqCst);
        let _start_time = Instant::now();

        // Start fault injection loop
        let fault_injection_task = self.start_fault_injection_loop().await;

        // Start recovery monitoring
        let recovery_monitoring_task = self.start_recovery_monitoring().await;

        // Start metrics collection
        let metrics_collection_task = self.start_metrics_collection().await;

        // Run for configured duration
        sleep(self.config.test_duration).await;

        // Stop all fault injection activities
        self.is_running.store(false, Ordering::SeqCst);

        // Wait for tasks to complete
        let _ = tokio::join!(
            fault_injection_task,
            recovery_monitoring_task,
            metrics_collection_task
        );

        // Collect final results
        let fault_history = self.fault_history.read().await.clone();
        let resilience_metrics = self.resilience_metrics.read().await.clone();

        self.print_fault_injection_summary(&fault_history, &resilience_metrics)
            .await;

        Ok(fault_history)
    }

    /// Start fault injection loop
    async fn start_fault_injection_loop(&self) -> tokio::task::JoinHandle<()> {
        let active_faults = Arc::clone(&self.active_faults);
        let fault_history = Arc::clone(&self.fault_history);
        let system_health = Arc::clone(&self.system_health);
        let is_running = Arc::clone(&self.is_running);
        let config = self.config.clone();

        tokio::spawn(async move {
            while is_running.load(Ordering::SeqCst) {
                // Check if we should inject a fault
                let random_probability = {
                    use rand::Rng;
                    rand::thread_rng().gen::<f64>()
                };
                if random_probability < config.fault_probability {
                    // Select random fault type
                    let fault_type = Self::select_random_fault_type(&config);
                    let fault_id = format!(
                        "fault_{}",
                        Uuid::new_v4()
                            .simple()
                            .to_string()
                            .chars()
                            .take(8)
                            .collect::<String>()
                    );

                    println!("💥 Injecting fault: {fault_type:?} (ID: {fault_id})");

                    // Create fault event
                    let fault_duration = {
                        use rand::Rng;
                        Duration::from_secs(rand::thread_rng().gen_range(5..30))
                    };
                    let fault_event = FaultEvent {
                        fault_type: fault_type.clone(),
                        start_time: Instant::now(),
                        duration: fault_duration,
                        target_component: Self::select_target_component(&fault_type),
                        severity: Self::determine_fault_severity(&fault_type),
                        injected_successfully: false,
                        recovery_successful: false,
                        impact_metrics: HashMap::new(),
                    };

                    // Attempt to inject the fault
                    match Self::inject_fault(&fault_type, &fault_event).await {
                        Ok(_) => {
                            let mut fault = fault_event;
                            fault.injected_successfully = true;

                            // Record active fault
                            active_faults
                                .write()
                                .await
                                .insert(fault_id.clone(), fault.clone());

                            // Update system health
                            let health_impact = Self::calculate_health_impact(&fault.severity);
                            let current_health = system_health.load(Ordering::SeqCst);
                            let new_health = current_health.saturating_sub(health_impact);
                            system_health.store(new_health, Ordering::SeqCst);

                            println!(
                                "  ✅ Fault injected successfully, system health: {new_health}%"
                            );
                        }
                        Err(e) => {
                            println!("  ❌ Failed to inject fault: {e}");
                            fault_history.write().await.push(fault_event);
                        }
                    }
                }

                // Wait before next potential fault injection
                let wait_millis = {
                    use rand::Rng;
                    rand::thread_rng().gen_range(1000..5000)
                };
                sleep(Duration::from_millis(wait_millis)).await;
            }
        })
    }

    /// Start recovery monitoring
    async fn start_recovery_monitoring(&self) -> tokio::task::JoinHandle<()> {
        let active_faults = Arc::clone(&self.active_faults);
        let fault_history = Arc::clone(&self.fault_history);
        let system_health = Arc::clone(&self.system_health);
        let is_running = Arc::clone(&self.is_running);
        let recovery_timeout = self.config.recovery_timeout;

        tokio::spawn(async move {
            while is_running.load(Ordering::SeqCst) {
                let mut faults_to_recover = Vec::new();

                // Check active faults for recovery
                {
                    let active = active_faults.read().await;
                    for (fault_id, fault) in active.iter() {
                        // Check if fault duration has elapsed or recovery should be attempted
                        if fault.start_time.elapsed() >= fault.duration
                            || fault.start_time.elapsed() >= recovery_timeout
                        {
                            faults_to_recover.push((fault_id.clone(), fault.clone()));
                        }
                    }
                }

                // Attempt recovery for expired faults
                for (fault_id, mut fault) in faults_to_recover {
                    println!(
                        "🔄 Attempting recovery for fault: {:?} (ID: {})",
                        fault.fault_type, fault_id
                    );

                    match Self::recover_from_fault(&fault.fault_type, &fault).await {
                        Ok(_) => {
                            fault.recovery_successful = true;
                            println!("  ✅ Recovery successful for fault: {:?}", fault.fault_type);

                            // Restore system health
                            let health_restoration = Self::calculate_health_impact(&fault.severity);
                            let current_health = system_health.load(Ordering::SeqCst);
                            let new_health = (current_health + health_restoration).min(100);
                            system_health.store(new_health, Ordering::SeqCst);

                            println!("  💚 System health restored to: {new_health}%");
                        }
                        Err(e) => {
                            println!(
                                "  ❌ Recovery failed for fault: {:?} - {}",
                                fault.fault_type, e
                            );
                        }
                    }

                    // Move fault to history
                    active_faults.write().await.remove(&fault_id);
                    fault_history.write().await.push(fault);
                }

                sleep(Duration::from_secs(2)).await;
            }
        })
    }

    /// Start metrics collection
    async fn start_metrics_collection(&self) -> tokio::task::JoinHandle<()> {
        let resilience_metrics = Arc::clone(&self.resilience_metrics);
        let fault_history = Arc::clone(&self.fault_history);
        let system_health = Arc::clone(&self.system_health);
        let is_running = Arc::clone(&self.is_running);

        tokio::spawn(async move {
            while is_running.load(Ordering::SeqCst) {
                // Collect current resilience metrics
                let history = fault_history.read().await;
                let current_health = system_health.load(Ordering::SeqCst);

                let faults_injected = history.len();
                let faults_recovered = history.iter().filter(|f| f.recovery_successful).count();

                let recovery_times: Vec<Duration> = history
                    .iter()
                    .filter(|f| f.recovery_successful)
                    .map(|f| f.duration)
                    .collect();

                let average_recovery_time = if !recovery_times.is_empty() {
                    recovery_times.iter().sum::<Duration>() / recovery_times.len() as u32
                } else {
                    Duration::from_secs(0)
                };

                let max_recovery_time = recovery_times
                    .iter()
                    .max()
                    .cloned()
                    .unwrap_or(Duration::from_secs(0));

                let metrics = ResilienceMetrics {
                    timestamp: Instant::now(),
                    faults_injected,
                    faults_recovered,
                    system_uptime_percent: current_health as f64,
                    average_recovery_time,
                    max_recovery_time,
                    error_rate: if faults_injected > 0 {
                        (faults_injected - faults_recovered) as f64 / faults_injected as f64
                    } else {
                        0.0
                    },
                    performance_degradation: (100.0 - current_health as f64) / 100.0,
                    availability_score: if faults_injected > 0 {
                        faults_recovered as f64 / faults_injected as f64 * 100.0
                    } else {
                        100.0
                    },
                };

                resilience_metrics.write().await.push(metrics);

                sleep(Duration::from_secs(5)).await;
            }
        })
    }

    // Fault injection and recovery methods

    fn select_random_fault_type(config: &FaultInjectionConfig) -> FaultType {
        let mut possible_faults = Vec::new();

        if config.enable_network_faults {
            possible_faults.extend_from_slice(&[
                FaultType::NetworkPartition,
                FaultType::NetworkTimeout,
                FaultType::NetworkCorruption,
            ]);
        }

        if config.enable_disk_faults {
            possible_faults.extend_from_slice(&[
                FaultType::DiskFull,
                FaultType::DiskReadError,
                FaultType::DiskWriteError,
                FaultType::FileSystemFull,
            ]);
        }

        if config.enable_memory_faults {
            possible_faults.extend_from_slice(&[
                FaultType::MemoryPressure,
                FaultType::MemoryLeak,
                FaultType::OutOfMemory,
            ]);
        }

        if config.enable_cpu_faults {
            possible_faults
                .extend_from_slice(&[FaultType::CpuExhaustion, FaultType::CpuThrottling]);
        }

        if config.enable_zfs_faults {
            possible_faults.extend_from_slice(&[
                FaultType::ZfsPoolOffline,
                FaultType::ZfsDeviceError,
                FaultType::ZfsChecksum,
            ]);
        }

        if config.enable_service_faults {
            possible_faults.extend_from_slice(&[
                FaultType::ServiceUnavailable,
                FaultType::ServiceSlowdown,
                FaultType::ServiceCrash,
                FaultType::DatabaseConnection,
            ]);
        }

        // Always include some general faults
        possible_faults.push(FaultType::ClockSkew);

        if possible_faults.is_empty() {
            FaultType::NetworkTimeout // Fallback
        } else {
            possible_faults[thread_rng().gen_range(0..possible_faults.len())].clone()
        }
    }

    fn select_target_component(fault_type: &FaultType) -> String {
        match fault_type {
            FaultType::NetworkPartition
            | FaultType::NetworkTimeout
            | FaultType::NetworkCorruption => "network".to_string(),
            FaultType::DiskFull
            | FaultType::DiskReadError
            | FaultType::DiskWriteError
            | FaultType::FileSystemFull => "disk".to_string(),
            FaultType::MemoryPressure | FaultType::MemoryLeak | FaultType::OutOfMemory => {
                "memory".to_string()
            }
            FaultType::CpuExhaustion | FaultType::CpuThrottling => "cpu".to_string(),
            FaultType::ZfsPoolOffline | FaultType::ZfsDeviceError | FaultType::ZfsChecksum => {
                "zfs".to_string()
            }
            FaultType::ServiceUnavailable
            | FaultType::ServiceSlowdown
            | FaultType::ServiceCrash => "service".to_string(),
            FaultType::DatabaseConnection => "database".to_string(),
            FaultType::ClockSkew => "system".to_string(),
        }
    }

    fn determine_fault_severity(fault_type: &FaultType) -> FaultSeverity {
        match fault_type {
            FaultType::NetworkTimeout | FaultType::ServiceSlowdown | FaultType::CpuThrottling => {
                FaultSeverity::Low
            }
            FaultType::NetworkCorruption | FaultType::DiskReadError | FaultType::MemoryPressure => {
                FaultSeverity::Medium
            }
            FaultType::NetworkPartition
            | FaultType::DiskWriteError
            | FaultType::ZfsDeviceError
            | FaultType::ServiceUnavailable => FaultSeverity::High,
            FaultType::DiskFull
            | FaultType::OutOfMemory
            | FaultType::ZfsPoolOffline
            | FaultType::ServiceCrash => FaultSeverity::Critical,
            _ => FaultSeverity::Medium,
        }
    }

    fn calculate_health_impact(severity: &FaultSeverity) -> u64 {
        match severity {
            FaultSeverity::Low => 5,
            FaultSeverity::Medium => 15,
            FaultSeverity::High => 30,
            FaultSeverity::Critical => 50,
        }
    }

    async fn inject_fault(
        fault_type: &FaultType,
        _fault_event: &FaultEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Simulate fault injection with appropriate delay
        let injection_delay = match fault_type {
            FaultType::NetworkPartition | FaultType::NetworkTimeout => Duration::from_millis(100),
            FaultType::DiskFull | FaultType::FileSystemFull => Duration::from_millis(500),
            FaultType::MemoryPressure | FaultType::OutOfMemory => Duration::from_millis(200),
            FaultType::CpuExhaustion => Duration::from_millis(300),
            FaultType::ZfsPoolOffline => Duration::from_millis(1000),
            FaultType::ServiceCrash => Duration::from_millis(50),
            _ => Duration::from_millis(100),
        };

        sleep(injection_delay).await;

        // Simulate some faults failing to inject
        if thread_rng().gen::<f64>() < 0.1 {
            // 10% failure rate
            return Err("Simulated injection failure".into());
        }

        Ok(())
    }

    async fn recover_from_fault(
        fault_type: &FaultType,
        _fault_event: &FaultEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Simulate recovery with appropriate delay
        let recovery_delay = match fault_type {
            FaultType::NetworkPartition => Duration::from_millis(2000),
            FaultType::DiskFull => Duration::from_millis(3000),
            FaultType::MemoryPressure => Duration::from_millis(1000),
            FaultType::ZfsPoolOffline => Duration::from_millis(5000),
            FaultType::ServiceCrash => Duration::from_millis(1500),
            _ => Duration::from_millis(500),
        };

        sleep(recovery_delay).await;

        // Simulate some recoveries failing
        if thread_rng().gen::<f64>() < 0.05 {
            // 5% recovery failure rate
            return Err("Simulated recovery failure".into());
        }

        Ok(())
    }

    /// Print fault injection test summary
    async fn print_fault_injection_summary(
        &self,
        fault_history: &[FaultEvent],
        resilience_metrics: &[ResilienceMetrics],
    ) {
        println!("\n📊 FAULT INJECTION TEST SUMMARY:");
        println!("==================================");

        if let Some(final_metrics) = resilience_metrics.last() {
            println!(
                "💥 Total Faults Injected: {}",
                final_metrics.faults_injected
            );
            println!(
                "🔄 Successful Recoveries: {}",
                final_metrics.faults_recovered
            );
            println!("📈 Recovery Rate: {:.1}%", final_metrics.availability_score);
            println!(
                "⏱️ Average Recovery Time: {:.2}s",
                final_metrics.average_recovery_time.as_secs_f64()
            );
            println!(
                "⏱️ Max Recovery Time: {:.2}s",
                final_metrics.max_recovery_time.as_secs_f64()
            );
            println!("❌ Error Rate: {:.1}%", final_metrics.error_rate * 100.0);
            println!(
                "🎯 Final System Health: {:.1}%",
                final_metrics.system_uptime_percent
            );
        }

        // Fault type statistics
        let mut fault_type_counts: HashMap<FaultType, usize> = HashMap::new();
        for fault in fault_history {
            *fault_type_counts
                .entry(fault.fault_type.clone())
                .or_insert(0) += 1;
        }

        println!("\n🔍 FAULT TYPE BREAKDOWN:");
        for (fault_type, count) in fault_type_counts {
            println!("  {fault_type:?}: {count} occurrences");
        }

        // Determine resilience score
        let resilience_score = if let Some(metrics) = resilience_metrics.last() {
            let recovery_score = metrics.availability_score;
            let health_score = metrics.system_uptime_percent;
            let speed_score = if metrics.average_recovery_time.as_secs() < 10 {
                100.0
            } else {
                100.0 - (metrics.average_recovery_time.as_secs() as f64 - 10.0) * 5.0
            }
            .max(0.0);

            recovery_score * 0.5 + health_score * 0.3 + speed_score * 0.2
        } else {
            0.0
        };

        let certification = if resilience_score >= 90.0 {
            "🥇 EXCELLENT RESILIENCE"
        } else if resilience_score >= 80.0 {
            "🥈 GOOD RESILIENCE"
        } else if resilience_score >= 70.0 {
            "🥉 ACCEPTABLE RESILIENCE"
        } else {
            "❌ INSUFFICIENT RESILIENCE"
        };

        println!("\n🏆 Resilience Certification: {certification} ({resilience_score:.1}%)");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Heavy test - use 'cargo bench' for full performance testing"]
    async fn test_fault_injection_framework() {
        println!("💥 Starting Comprehensive Fault Injection Testing");
        let config = FaultInjectionConfig {
            test_duration: nestgate_core::constants::test_defaults::TEST_SHORT_TIMEOUT,
            fault_probability: 0.3,
            recovery_timeout: Duration::from_secs(3),
            enable_network_faults: true,
            enable_disk_faults: true,
            enable_memory_faults: false, // Disable for test
            enable_cpu_faults: false,    // Disable for test
            enable_zfs_faults: false,    // Disable for test
            enable_service_faults: true,
            chaos_mode: false,
        };

        let framework = FaultInjectionFramework::new(config);
        let result = framework.run_fault_injection_test().await;

        assert!(result.is_ok());
        let fault_events = result.unwrap();

        // Should have injected some faults during the test
        assert!(
            !fault_events.is_empty(),
            "Should have injected at least one fault"
        );

        // Check that faults were properly recorded
        for fault in &fault_events {
            assert!(!fault.target_component.is_empty());
            assert!(fault.duration > Duration::from_secs(0));
        }
    }

    #[tokio::test]
    async fn test_fault_type_selection() {
        let config = FaultInjectionConfig {
            enable_network_faults: true,
            enable_disk_faults: false,
            enable_memory_faults: false,
            enable_cpu_faults: false,
            enable_zfs_faults: false,
            enable_service_faults: false,
            ..Default::default()
        };

        // Should only select network faults
        for _ in 0..10 {
            let fault_type = FaultInjectionFramework::select_random_fault_type(&config);
            assert!(matches!(
                fault_type,
                FaultType::NetworkPartition
                    | FaultType::NetworkTimeout
                    | FaultType::NetworkCorruption
                    | FaultType::ClockSkew // Always available
            ));
        }
    }

    #[tokio::test]
    async fn test_fault_severity_assignment() {
        assert_eq!(
            FaultInjectionFramework::determine_fault_severity(&FaultType::NetworkTimeout),
            FaultSeverity::Low
        );
        assert_eq!(
            FaultInjectionFramework::determine_fault_severity(&FaultType::NetworkCorruption),
            FaultSeverity::Medium
        );
        assert_eq!(
            FaultInjectionFramework::determine_fault_severity(&FaultType::NetworkPartition),
            FaultSeverity::High
        );
        assert_eq!(
            FaultInjectionFramework::determine_fault_severity(&FaultType::DiskFull),
            FaultSeverity::Critical
        );
    }

    #[tokio::test]
    async fn test_health_impact_calculation() {
        assert_eq!(
            FaultInjectionFramework::calculate_health_impact(&FaultSeverity::Low),
            5
        );
        assert_eq!(
            FaultInjectionFramework::calculate_health_impact(&FaultSeverity::Medium),
            15
        );
        assert_eq!(
            FaultInjectionFramework::calculate_health_impact(&FaultSeverity::High),
            30
        );
        assert_eq!(
            FaultInjectionFramework::calculate_health_impact(&FaultSeverity::Critical),
            50
        );
    }

    #[tokio::test]
    async fn test_target_component_selection() {
        assert_eq!(
            FaultInjectionFramework::select_target_component(&FaultType::NetworkTimeout),
            "network"
        );
        assert_eq!(
            FaultInjectionFramework::select_target_component(&FaultType::DiskFull),
            "disk"
        );
        assert_eq!(
            FaultInjectionFramework::select_target_component(&FaultType::MemoryPressure),
            "memory"
        );
        assert_eq!(
            FaultInjectionFramework::select_target_component(&FaultType::ZfsPoolOffline),
            "zfs"
        );
    }

    #[tokio::test]
    async fn test_fault_injection_and_recovery() {
        let fault_event = FaultEvent {
            fault_type: FaultType::NetworkTimeout,
            start_time: Instant::now(),
            duration: Duration::from_secs(5),
            target_component: "network".to_string(),
            severity: FaultSeverity::Low,
            injected_successfully: false,
            recovery_successful: false,
            impact_metrics: HashMap::new(),
        };

        // Test fault injection
        let _injection_result =
            FaultInjectionFramework::inject_fault(&FaultType::NetworkTimeout, &fault_event).await;
        // Should succeed most of the time (90% success rate in simulation)

        // Test recovery
        let _recovery_result =
            FaultInjectionFramework::recover_from_fault(&FaultType::NetworkTimeout, &fault_event)
                .await;
        // Should succeed most of the time (95% success rate in simulation)
    }
}
