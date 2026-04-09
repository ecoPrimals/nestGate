// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![cfg(any(test, feature = "dev-stubs"))]

//! **DEVELOPMENT STUBS MODULE**
//!
//! ⚠️ **WARNING: DEVELOPMENT AND TESTING ONLY** ⚠️
//!
//! This module consolidates all development stub implementations for the NestGate API.
//! All data returned is HARDCODED and does not reflect actual system state.
//!
//! **DO NOT USE IN PRODUCTION** - Use real implementations from respective crates instead.

// ═══════════════════════════════════════════════════════════════
// 🔒 FEATURE GATE: Development/Testing Only
// ═══════════════════════════════════════════════════════════════
// This module is NOT compiled in production builds.
// Only available with the 'dev-stubs' feature flag.
//!
//! # Purpose
//!
//! This module provides stub implementations to enable:
//! - Local development without specialized hardware
//! - Unit testing of API endpoints with predictable data
//! - Integration testing in CI/CD environments
//! - Rapid prototyping and debugging
//!
//! # Feature Gates
//!
//! All modules in this directory are only available with the `dev-stubs` feature flag.
//! Production builds will NOT include this code.
//!
//! # Organization
//!
//! - [`zfs`](crate::dev_stubs::zfs) - ZFS operation stubs (development pools, datasets, etc.)
//! - [`hardware`](crate::dev_stubs::hardware) - Hardware tuning stubs (CPU, GPU, memory metrics)
//! - [`testing`](crate::dev_stubs::testing) - Test utilities (mock builders, test doubles)
//!
//! # Production Alternatives
//!
//! For production use, see:
//! - `nestgate_zfs` crate - Real ZFS implementations
//! - System monitoring crates - Real hardware metrics
//! - Proper integration tests - Real system integration
//!
//! # Migration Note
//!
//! **Consolidated**: November 10, 2025
//! - Replaced scattered stub files with organized module structure
//! - All stubs now in centralized location for easier maintenance
//! - Deprecated: Individual stub files (zfs_stub.rs, stub_helpers.rs)

pub mod hardware;
pub mod testing;
pub mod zfs;

// Re-export commonly used types for convenience
pub use hardware::{
    create_stub_benchmark_result, create_stub_compute_allocation, create_stub_compute_resources,
    create_stub_system_profile, create_stub_tuning_result, create_zero_hardware_metrics,
};
pub use testing::{
    MockingConfig, ResourceAllocation, WorkloadResult, build_mock_resource_allocation,
    build_mock_workload_result,
};
pub use zfs::{ProductionZfsManager, ZfsConfig};
