// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// **CONCURRENCY CONFIGURATION**

use serde::{Deserialize, Serialize};
// Result type not needed in this module

/// Concurrency configuration for controlling parallel execution.
///
/// Defines how tasks are executed concurrently and how work is distributed.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Concurrency
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
/// Concurrencymodel
pub enum ConcurrencyModel {
    /// Traditional thread pool model (default).
    #[default]
    /// Threadpool
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
/// Loadbalancingstrategy
pub enum LoadBalancingStrategy {
    /// Round-robin distribution (default).
    #[default]
    /// Roundrobin
    RoundRobin,
    /// Assign to least loaded worker.
    LeastLoaded,
    /// Random assignment.
    Random,
    /// Weighted distribution based on worker capacity.
    Weighted,
}

impl Default for ConcurrencyConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            max_concurrent: 100,
            model: ConcurrencyModel::default(),
            work_stealing: true,
            load_balancing: LoadBalancingStrategy::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn serde_roundtrip<T>(v: &T)
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let s = serde_json::to_string(v).expect("to_string");
        let _: T = serde_json::from_str(&s).expect("from_str");
    }

    #[test]
    fn concurrency_config_default_serde() {
        serde_roundtrip(&ConcurrencyConfig::default());
    }

    #[test]
    fn concurrency_model_variants() {
        for m in [
            ConcurrencyModel::ThreadPool,
            ConcurrencyModel::ActorModel,
            ConcurrencyModel::ReactiveStreams,
            ConcurrencyModel::AsyncAwait,
        ] {
            serde_roundtrip(&m);
        }
    }

    #[test]
    fn load_balancing_strategy_variants() {
        for s in [
            LoadBalancingStrategy::RoundRobin,
            LoadBalancingStrategy::LeastLoaded,
            LoadBalancingStrategy::Random,
            LoadBalancingStrategy::Weighted,
        ] {
            serde_roundtrip(&s);
        }
    }
}
