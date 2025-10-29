// **LOAD BALANCER SYSTEM**
//! Module definitions and exports.
// This module provides comprehensive load balancing capabilities for NestGate services.
// Split from a single large file to maintain the 1000-line limit while preserving
//! all functionality and maintaining backward compatibility.

// Sub-module declarations
pub mod algorithms;
pub mod implementations;
pub mod stats;
pub mod traits;

// Re-export all public types and traits for backward compatibility
pub use algorithms::*;
pub use implementations::*;
pub use stats::*;
pub use traits::*;

// Convenience re-exports for common usage patterns
pub use algorithms::{
    LeastConnectionsLoadBalancer, RandomLoadBalancer, RoundRobinLoadBalancer,
    WeightedRandomLoadBalancer, WeightedRoundRobinLoadBalancer,
};
pub use implementations::HealthAwareLoadBalancer;
pub use stats::{LoadBalancerStats, ServiceStats};
pub use traits::LoadBalancerImpl;
