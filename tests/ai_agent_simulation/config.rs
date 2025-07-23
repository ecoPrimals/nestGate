//! AI Agent Simulation Configuration

use std::time::Duration;

/// **AI AGENT SIMULATION CONFIGURATION**
#[derive(Debug, Clone)]
pub struct AIAgentSimulationConfig {
    /// Number of concurrent AI agents to simulate
    pub concurrent_agents: usize,
    /// Duration of simulation test
    pub simulation_duration: Duration,
    /// Operations per agent per minute
    pub operations_per_agent_per_minute: u32,
    /// Agent behavior complexity (1-10)
    pub behavior_complexity: u8,
    /// Error injection rate for resilience testing
    pub error_injection_rate: f64,
    /// Learning simulation enabled
    pub enable_learning_simulation: bool,
    /// Multi-modal interaction complexity
    pub multimodal_complexity: u8,
    /// Agent coordination testing enabled
    pub enable_coordination_testing: bool,
    /// Performance benchmark targets
    pub benchmark_targets: PerformanceBenchmarkTargets,
}

#[derive(Debug, Clone)]
pub struct PerformanceBenchmarkTargets {
    /// Target response time (95th percentile)
    pub response_time_p95_ms: u64,
    /// Target throughput (operations per second)
    pub target_throughput_ops_sec: f64,
    /// Target success rate
    pub target_success_rate: f64,
    /// Target resource utilization efficiency
    pub target_resource_efficiency: f64,
    /// Target concurrent agent capacity
    pub target_concurrent_capacity: usize,
}

impl Default for AIAgentSimulationConfig {
    fn default() -> Self {
        Self {
            concurrent_agents: 50,
            simulation_duration: Duration::from_secs(300), // 5 minutes
            operations_per_agent_per_minute: 20,
            behavior_complexity: 7,     // High complexity
            error_injection_rate: 0.05, // 5% error rate
            enable_learning_simulation: true,
            multimodal_complexity: 8,
            enable_coordination_testing: true,
            benchmark_targets: PerformanceBenchmarkTargets {
                response_time_p95_ms: 500, // 500ms
                target_throughput_ops_sec: 100.0,
                target_success_rate: 0.99,        // 99%
                target_resource_efficiency: 0.85, // 85%
                target_concurrent_capacity: 100,
            },
        }
    }
}
