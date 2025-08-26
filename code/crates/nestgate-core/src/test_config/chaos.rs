///
/// This module provides configuration for chaos engineering tests including
/// failure injection, network chaos, resource chaos, and recovery testing.
use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== CHAOS ENGINEERING CONFIGURATION ====================

/// **Unified chaos engineering configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestChaosConfig {
    /// Failure injection configuration
    pub failure_injection: FailureInjectionConfig,
    /// Network chaos configuration
    pub network_chaos: NetworkChaosConfig,
    /// Resource chaos configuration
    pub resource_chaos: ResourceChaosConfig,
    /// Recovery testing configuration
    pub recovery: RecoveryTestConfig,
}

/// **Failure injection configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FailureInjectionConfig {
    /// Enable failure injection
    pub enabled: bool,
    /// Failure probability (0.0-1.0)
    pub failure_probability: f64,
    /// Failure types to inject
    pub failure_types: Vec<String>,
    /// Injection duration
    pub injection_duration: Duration,
}

/// **Network chaos configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkChaosConfig {
    /// Enable network chaos
    pub enabled: bool,
    /// Latency injection (ms)
    pub latency_ms: u64,
    /// Packet loss percentage
    pub packet_loss_percent: f64,
    /// Bandwidth limitation (Mbps)
    pub bandwidth_limit_mbps: f64,
}

/// **Resource chaos configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceChaosConfig {
    /// Enable resource chaos
    pub enabled: bool,
    /// CPU stress percentage
    pub cpu_stress_percent: f64,
    /// Memory stress MB
    pub memory_stress_mb: u64,
    /// Disk stress MB/s
    pub disk_stress_mbps: f64,
}

/// **Recovery test configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecoveryTestConfig {
    /// Enable recovery tests
    pub enabled: bool,
    /// Recovery timeout
    pub recovery_timeout: Duration,
    /// Recovery verification steps
    pub verification_steps: Vec<String>,
    /// Expected recovery time
    pub expected_recovery_time: Duration,
}
