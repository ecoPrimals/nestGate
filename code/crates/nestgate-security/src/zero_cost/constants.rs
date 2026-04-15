// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Named compile-time values for the zero-cost stack (placeholder metrics and tests).

/// Placeholder request count until real accounting exists.
pub const ZERO_COST_PLACEHOLDER_REQUESTS_PROCESSED: u64 = 1000;

/// Placeholder average latency (nanoseconds) until measured.
pub const ZERO_COST_PLACEHOLDER_AVERAGE_LATENCY_NS: u64 = 50_000;

#[cfg(test)]
/// Test-only compile-time timeout (1s) for zero-cost generic parameters.
pub const ZERO_COST_TEST_TIMEOUT_MS_1000: u64 = 1000;

#[cfg(test)]
/// Test-only compile-time timeout (2s) for zero-cost generic parameters.
pub const ZERO_COST_TEST_TIMEOUT_MS_2000: u64 = 2000;

#[cfg(test)]
/// Test-only compile-time timeout (3s) for zero-cost generic parameters.
pub const ZERO_COST_TEST_TIMEOUT_MS_3000: u64 = 3000;

#[cfg(test)]
/// Test-only compile-time timeout (5s) for zero-cost generic parameters.
pub const ZERO_COST_TEST_TIMEOUT_MS_5000: u64 = 5000;

#[cfg(test)]
/// Test-only P95 latency (nanoseconds) for synthetic performance metrics.
pub const ZERO_COST_TEST_LATENCY_P95_NS: u64 = 50_000;
