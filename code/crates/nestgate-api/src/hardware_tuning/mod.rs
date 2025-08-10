//! Universal Hardware Tuning and Compute Integration
//!
//! ✅ **MODERNIZED**: Capability-based compute service integration with universal adapter pattern

pub mod adapter;
pub mod client;
pub mod types;

// ✅ MODERN: Universal compute integration (capability-based)
pub use adapter::HardwareTuningAdapter;
pub use client::UniversalComputeClient;
pub use types::*;
