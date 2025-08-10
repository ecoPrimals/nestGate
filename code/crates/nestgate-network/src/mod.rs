//! NestGate Network Module
//!
//! This module provides network connectivity and orchestration capabilities through:
//! - OrchestrationAdapter (universal adapter pattern)

pub mod api;
pub mod connection_manager;
pub mod orchestration_adapter;  // ✅ RENAMED: songbird → orchestration_adapter (capability-based)
pub mod universal_orchestration;
pub mod types;

// Universal adapter-based exports (recommended)
pub use orchestration_adapter::{
    OrchestrationAdapter, OrchestrationServiceRegistration,
    ServiceDiscoveryRequest, ServiceDiscoveryResponse,
    DiscoveredService,
};

// Common type exports
pub use types::*;

// Use canonical Result type from nestgate-core::error
pub use nestgate_core::error::Result; 