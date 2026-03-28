// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use super::compute::ZeroCostUniversalComputeWrapper;
use super::orchestration::ZeroCostUniversalOrchestrationWrapper;
use super::security::ZeroCostUniversalSecurityWrapper;

/// Common zero-cost provider configurations
pub type StandardZeroCostSecurityWrapper<Provider> =
    ZeroCostUniversalSecurityWrapper<Provider, 1000>;
/// Type alias for Highperformancezerocostsecuritywrapper
pub type HighPerformanceZeroCostSecurityWrapper<Provider> =
    ZeroCostUniversalSecurityWrapper<Provider, 10000>;
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
