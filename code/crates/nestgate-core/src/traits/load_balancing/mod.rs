//! Load Balancing Module for Service Distribution
//!
//! **MIGRATED FROM**: `traits::load_balancing` (November 7, 2025)
//! **CANONICAL**: This is the single source of truth for load balancing
//! **STATUS**: Production-ready, native async
//!
//! This module provides comprehensive load balancing functionality with multiple
//! algorithms, health awareness, and weighted routing support.

pub mod algorithms;
pub mod core;
pub mod health_aware;
pub mod weighted;

// Re-export all public items for convenient access
pub use algorithms::{LeastConnectionsLoadBalancer, RandomLoadBalancer, RoundRobinLoadBalancer};
pub use core::{LoadBalancer, LoadBalancerStats, LoadBalancingAlgorithm, ServiceStats};
pub use health_aware::HealthAwareLoadBalancer;
pub use weighted::{WeightedRandomLoadBalancer, WeightedRoundRobinLoadBalancer};

