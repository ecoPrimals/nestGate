// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use super::ZeroCostService;

/// Minimal struct to test ZeroCostService marker trait
#[derive(Debug)]
struct TestZeroCostService;

impl ZeroCostService<Self> for TestZeroCostService {}

#[test]
fn test_zero_cost_service_marker() {
    // ZeroCostService is a marker trait - just verify we can use it
    let _service = TestZeroCostService;
}

#[test]
fn test_zero_cost_service_type_implements_trait() {
    // Verify the trait can be used as a bound
    fn requires_zero_cost<T: ZeroCostService<T>>(_t: &T) {}
    let service = TestZeroCostService;
    requires_zero_cost(&service);
}
