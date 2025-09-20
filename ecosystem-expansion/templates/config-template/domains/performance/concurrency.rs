//! **CONCURRENCY CONFIGURATION**

use serde::{Deserialize, Serialize};
// Result type not needed in this module

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyConfig {
    pub max_concurrent: usize,
    pub model: ConcurrencyModel,
    pub work_stealing: bool,
    pub load_balancing: LoadBalancingStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ConcurrencyModel {
    #[default]
    ThreadPool,
    ActorModel,
    ReactiveStreams,
    AsyncAwait,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LoadBalancingStrategy {
    #[default]
    RoundRobin,
    LeastLoaded,
    Random,
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