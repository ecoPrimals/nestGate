//! Hardware Tuning Module
//!
//! This module provides hardware tuning capabilities for NestGate,
//! including client integration with ToadStool compute services.

pub mod client;
pub mod handler;
pub mod types;

// Re-export main types and clients for convenience
pub use client::ToadstoolComputeClient;
pub use handler::HardwareTuningHandler;
pub use types::*;
