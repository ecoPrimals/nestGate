//! **PERFORMANCE HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceHandlerConfig {
    pub caching: CachingHandlerConfig,
    pub optimization: OptimizationHandlerConfig,
    pub profiler: ProfilerHandlerConfig,
    pub load_balancing: LoadBalancingHandlerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingHandlerConfig { pub enabled: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationHandlerConfig { pub enabled: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilerHandlerConfig { pub enabled: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingHandlerConfig { pub enabled: bool }

impl Default for PerformanceHandlerConfig {
    fn default() -> Self {
        Self {
            caching: CachingHandlerConfig { enabled: true },
            optimization: OptimizationHandlerConfig { enabled: true },
            profiler: ProfilerHandlerConfig { enabled: false },
            load_balancing: LoadBalancingHandlerConfig { enabled: false },
        }
    }
}

impl PerformanceHandlerConfig {
    pub fn production_optimized() -> Self { Self::default() }
    pub fn development_optimized() -> Self { Self::default() }
    pub fn high_performance() -> Self { Self::default() }
    pub fn merge(self, _other: Self) -> Self { self }
    pub fn validate(&self) -> crate::Result<()> { Ok(()) }
} 