// **CONCURRENCY CONFIGURATION**

use serde::{Deserialize, Serialize};
// Result type not needed in this module

/// Concurrency configuration for controlling parallel execution.
///
/// Defines how tasks are executed concurrently and how work is distributed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyConfig {
    /// Maximum number of concurrent operations allowed.
    pub max_concurrent: usize,
    /// Concurrency model to use for parallel execution.
    pub model: ConcurrencyModel,
    /// Whether to enable work-stealing for better load distribution.
    pub work_stealing: bool,
    /// Load balancing strategy for distributing work.
    pub load_balancing: LoadBalancingStrategy,
}

/// Concurrency execution model.
///
/// Determines how concurrent operations are executed and managed.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ConcurrencyModel {
    /// Traditional thread pool model (default).
    #[default]
    ThreadPool,
    /// Actor-based message passing model.
    ActorModel,
    /// Reactive streams for backpressure-aware processing.
    ReactiveStreams,
    /// Async/await coroutine model.
    AsyncAwait,
}

/// Load balancing strategy for work distribution.
///
/// Determines how tasks are assigned to workers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LoadBalancingStrategy {
    /// Round-robin distribution (default).
    #[default]
    RoundRobin,
    /// Assign to least loaded worker.
    LeastLoaded,
    /// Random assignment.
    Random,
    /// Weighted distribution based on worker capacity.
    Weighted,
}

impl Default for ConcurrencyConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 100,
            model: ConcurrencyModel::default(),
            work_stealing: true,
            load_balancing: LoadBalancingStrategy::default(),
        }
    }
}
