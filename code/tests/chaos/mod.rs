// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **CHAOS TEST SUITE - NESTGATE**
//!
//! Chaos engineering tests to verify system resilience under adverse conditions
//!
//! **Test Categories**:
//! - Network failures and partitions
//! - Timeout cascades
//! - Service unavailability
//! - Resource exhaustion
//! - Partial failures
//! - Recovery scenarios

#[cfg(test)]
mod network_chaos;

#[cfg(test)]
mod timeout_chaos;

#[cfg(test)]
mod service_failure_chaos;

#[cfg(test)]
mod resource_chaos;

