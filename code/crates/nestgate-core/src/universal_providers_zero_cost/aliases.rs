// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::compute::ZeroCostUniversalComputeWrapper;
use super::orchestration::ZeroCostUniversalOrchestrationWrapper;

/// Type alias for Standardzerocostorchestrationwrapper
pub type StandardZeroCostOrchestrationWrapper<Provider> =
    ZeroCostUniversalOrchestrationWrapper<Provider, 500>;
/// Type alias for Highperformancezerocostorchestrationwrapper
pub type HighPerformanceZeroCostOrchestrationWrapper<Provider> =
    ZeroCostUniversalOrchestrationWrapper<Provider, 5000>;

/// Type alias for Standardzerocostcomputewrapper
pub type StandardZeroCostComputeWrapper<Provider> = ZeroCostUniversalComputeWrapper<Provider, 1000>;
/// Type alias for Highperformancezerocostcomputewrapper
pub type HighPerformanceZeroCostComputeWrapper<Provider> =
    ZeroCostUniversalComputeWrapper<Provider, 10000>;
