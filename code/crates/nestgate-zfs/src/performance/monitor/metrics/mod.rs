// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! ZFS performance metrics collection split by concern (pool I/O, host resources, tiers).

mod collect;
mod io_stats;
mod parsing;
mod pool_metrics;
mod system_metrics;
mod tier_metrics;

#[cfg(test)]
mod iostat_and_queue_tests;

#[cfg(test)]
impl crate::performance::types::ZfsPerformanceMonitor {
    pub(crate) fn test_parse_iostat_bandwidth(
        value: &str,
    ) -> std::result::Result<u64, std::num::ParseFloatError> {
        Self::parse_iostat_bandwidth(value)
    }

    pub(crate) fn test_get_real_queue_depth(
        tier: &crate::types::StorageTier,
    ) -> nestgate_core::Result<f64> {
        Self::get_real_queue_depth(tier)
    }

    pub(crate) fn test_parse_zpool_iostat(
        output: &str,
    ) -> nestgate_core::Result<crate::performance::types::IoStatsSummary> {
        Self::parse_zpool_iostat(output)
    }

    pub(crate) fn test_parse_zpool_get_pool_properties(
        output: &str,
    ) -> crate::performance::types::PoolProperties {
        self::parsing::parse_zpool_get_pool_properties(output)
    }
}
