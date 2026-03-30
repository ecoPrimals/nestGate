// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Pre-configured type aliases for common ZFS manager deployments.

use super::ZeroCostZfsManager;

/// Development ZFS manager: Small limits, fast timeout
pub type DevelopmentZfsManager = ZeroCostZfsManager<10, 100, 1000, 10_000>; // 10 pools, 100 datasets, 1k snapshots, 10s timeout
/// Production ZFS manager: Large limits, standard timeout
pub type ProductionZfsManager = ZeroCostZfsManager<100, 10_000, 100_000, 30000>; // 100 pools, 10k datasets, 100k snapshots, 30s timeout
/// High-performance ZFS manager: Optimized limits, balanced timeout
pub type HighPerformanceZfsManager = ZeroCostZfsManager<200, 20000, 200000, 45000>; // 200 pools, 20k datasets, 200k snapshots, 45s timeout
/// Testing ZFS manager: Tiny limits, very fast timeout
pub type TestingZfsManager = ZeroCostZfsManager<2, 10, 100, 5000>; // 2 pools, 10 datasets, 100 snapshots, 5s timeout
/// Enterprise ZFS manager: Very large limits, long timeout
pub type EnterpriseZfsManager = ZeroCostZfsManager<1000, 100_000, 1_000_000, 60000>; // 1k pools, 100k datasets, 1M snapshots, 60s timeout
