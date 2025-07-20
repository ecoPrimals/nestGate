//! # Hardware Tuning API Handler
//!
//! **Agnostic hardware tuning for any setup**
//!
//! This handler provides REST API endpoints for automatic hardware detection
//! and tuning, with external extraction protection via crypto locks.
//!
//! This module has been refactored into a modular structure.
//! The implementation is now in the `hardware_tuning` submodule.

// Re-export everything from the hardware_tuning module
pub use crate::hardware_tuning::*;
