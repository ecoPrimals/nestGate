// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

/// Pure arithmetic for cache sizing, eviction, and hit ratios.
///
/// Extracted for unit tests and mutation testing (e.g. `+=` vs `-=`, comparison operators).
/// Targets include `CacheManager` size updates, eviction thresholds, and hit-ratio division.
///
/// Calculate if cache needs eviction based on current and new sizes
///
/// **PURE FUNCTION**: No side effects, deterministic output with overflow protection
///
/// **TESTABLE**: Can verify exact arithmetic with boundary conditions
#[must_use]
pub const fn needs_eviction(current_size: u64, new_item_size: u64, max_size: u64) -> bool {
    if max_size == 0 {
        return false; // No size limit
    }
    // Prevent overflow by checking if addition would exceed max 🛡️
    if current_size > max_size {
        return true; // Already over limit
    }

    new_item_size > max_size - current_size
}

/// Calculate total cache size from item sizes
/// **PURE FUNCTION**: Simple addition with overflow protection
/// **TESTABLE**: Can verify sum calculation with edge cases and overflow protection
#[must_use]
pub fn calculate_total_cache_size(item_sizes: &[u64]) -> u64 {
    // Use saturating fold to prevent overflow 🛡️
    item_sizes
        .iter()
        .fold(0u64, |acc, &size| acc.saturating_add(size))
}
/// Calculate how much space needs to be evicted
/// **PURE FUNCTION**: Safe subtraction with minimum eviction amount
/// **TESTABLE**: Can verify exact eviction calculations
#[must_use]
pub const fn calculate_eviction_size(current_size: u64, new_item_size: u64, max_size: u64) -> u64 {
    if current_size + new_item_size <= max_size {
        return 0; // No eviction needed
    }
    (current_size + new_item_size) - max_size
}

/// Calculate cache hit ratio from hit and miss counts
/// **PURE FUNCTION**: Safe division with zero handling and extreme value logic
/// **TESTABLE**: Can verify exact floating point precision
#[must_use]
#[expect(
    clippy::cast_precision_loss,
    reason = "Hit ratio metric from u64 hit/miss counters"
)]
pub fn calculate_hit_ratio(hits: u64, misses: u64) -> f64 {
    // Handle extreme cases where both values are near max 🛡️
    if hits == u64::MAX && misses == u64::MAX {
        return 0.5; // Logically, equal hits and misses = 50% hit rate
    }
    // Use saturating addition to prevent overflow
    let total_requests = hits.saturating_add(misses);
    if total_requests == 0 {
        0.0
    } else {
        hits as f64 / total_requests as f64
    }
}

/// Update cache size after adding an item
/// **PURE FUNCTION**: Simple addition with overflow protection
/// **TESTABLE**: Can verify size updates with boundary conditions
#[must_use]
pub const fn add_to_cache_size(current_size: u64, item_size: u64) -> u64 {
    current_size.saturating_add(item_size)
}
/// Update cache size after removing an item
/// **PURE FUNCTION**: Safe subtraction preventing underflow
/// **TESTABLE**: Can verify size updates with underflow protection
#[must_use]
pub const fn subtract_from_cache_size(current_size: u64, item_size: u64) -> u64 {
    current_size.saturating_sub(item_size)
}
/// Check if cache has reached maximum size threshold
/// **PURE FUNCTION**: Simple comparison logic
/// **TESTABLE**: Can verify boundary conditions precisely
#[must_use]
pub const fn is_at_max_size(current_size: u64, max_size: u64) -> bool {
    if max_size == 0 {
        return false; // No size limit
    }
    current_size >= max_size
}

/// Calculate memory pool threshold for expansion
/// **PURE FUNCTION**: Percentage-based threshold calculation
/// **TESTABLE**: Can verify threshold arithmetic with precision
#[must_use]
#[expect(
    clippy::cast_precision_loss,
    reason = "Compares current size to a fractional threshold of max_size"
)]
pub fn calculate_pool_expansion_threshold(
    current_size: usize,
    max_size: usize,
    threshold_percent: f64,
) -> bool {
    if max_size == 0 {
        return false; // No expansion needed if no limit
    }
    let threshold_size = max_size as f64 * threshold_percent / 100.0;
    (current_size as f64) >= threshold_size
}

/// Calculate optimal eviction count based on access patterns
/// **PURE FUNCTION**: Strategy-based eviction calculation
/// **TESTABLE**: Can verify eviction count with different strategies
#[must_use]
#[expect(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "Percentage of item count; result clamped with min(total_items)"
)]
pub fn calculate_optimal_eviction_count(total_items: usize, target_free_percent: f64) -> usize {
    if total_items == 0 {
        return 0;
    }
    let target_free_items = (total_items as f64 * target_free_percent / 100.0) as usize;
    target_free_items.min(total_items) // Ensure we don't evict more than we have
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::*;

    /// 🎯 **CACHE ARITHMETIC MUTATION DETECTION TESTS**
    /// These tests are specifically designed to catch mutations in cache operations

    #[test]
    fn test_eviction_decision_boundary_conditions() {
        // ✅ CATCHES COMPARISON MUTATIONS (> vs >=, > vs <)
        assert!(!needs_eviction(100, 50, 200)); // 100 + 50 = 150 < 200, no eviction
        assert!(needs_eviction(100, 101, 200)); // 100 + 101 = 201 > 200, needs eviction
        assert!(!needs_eviction(100, 100, 200)); // 100 + 100 = 200 = 200, boundary case

        // ✅ CATCHES ZERO MAX SIZE EDGE CASE
        assert!(!needs_eviction(1000, 1000, 0)); // No limit when max_size is 0

        // ✅ CATCHES ADDITION MUTATIONS (+ vs -, + vs *)
        assert!(needs_eviction(150, 51, 200)); // 150 + 51 = 201 > 200
        assert!(!needs_eviction(150, 50, 200)); // 150 + 50 = 200 = 200
    }

    #[test]
    fn test_total_size_calculation_precision() {
        // ✅ CATCHES SUM MUTATIONS AND OVERFLOW HANDLING
        let sizes = vec![100, 200, 300, 400];
        assert_eq!(calculate_total_cache_size(&sizes), 1000);

        // ✅ CATCHES EMPTY SLICE HANDLING
        assert_eq!(calculate_total_cache_size(&[]), 0);

        // ✅ CATCHES SINGLE ITEM CASE
        assert_eq!(calculate_total_cache_size(&[42]), 42);

        // ✅ CATCHES LARGE NUMBER HANDLING
        let large_sizes = vec![u64::MAX / 4, u64::MAX / 4, u64::MAX / 4, u64::MAX / 4];
        let total = calculate_total_cache_size(&large_sizes);
        assert!(total > 0); // Should not overflow to 0
    }

    #[test]
    fn test_eviction_size_calculation_exact() {
        // ✅ CATCHES ARITHMETIC MUTATIONS (+ vs -, - vs +)
        assert_eq!(calculate_eviction_size(150, 100, 200), 50); // (150 + 100) - 200 = 50
        assert_eq!(calculate_eviction_size(100, 50, 200), 0); // 150 <= 200, no eviction
        assert_eq!(calculate_eviction_size(200, 1, 200), 1); // (200 + 1) - 200 = 1

        // ✅ CATCHES EDGE CASES
        assert_eq!(calculate_eviction_size(0, 0, 100), 0); // Nothing to evict
        assert_eq!(calculate_eviction_size(300, 0, 200), 100); // Current already over limit
    }

    #[test]
    fn test_hit_ratio_division_precision() {
        // ✅ CATCHES DIVISION MUTATIONS (/ vs *, / vs +, / vs -)
        assert_eq!(calculate_hit_ratio(8, 2), 0.8); // 8 / (8 + 2) = 8/10 = 0.8
        assert_eq!(calculate_hit_ratio(1, 3), 0.25); // 1 / (1 + 3) = 1/4 = 0.25
        assert_eq!(calculate_hit_ratio(7, 3), 0.7); // 7 / (7 + 3) = 7/10 = 0.7

        // ✅ CATCHES DIVISION BY ZERO HANDLING
        assert_eq!(calculate_hit_ratio(0, 0), 0.0); // 0 requests total

        // ✅ CATCHES PERFECT HIT RATIO
        assert_eq!(calculate_hit_ratio(10, 0), 1.0); // 10/10 = 100% hit rate

        // ✅ CATCHES ZERO HIT RATIO
        assert_eq!(calculate_hit_ratio(0, 10), 0.0); // 0/10 = 0% hit rate
    }

    #[test]
    fn test_cache_size_update_mutations() {
        // ✅ CATCHES ADDITION MUTATIONS (+= vs -=, + vs -)
        assert_eq!(add_to_cache_size(100, 50), 150);
        assert_eq!(add_to_cache_size(0, 100), 100);

        // ✅ CATCHES OVERFLOW PROTECTION (saturating_add vs wrapping_add)
        assert_eq!(add_to_cache_size(u64::MAX, 1), u64::MAX); // Should saturate, not wrap

        // ✅ CATCHES SUBTRACTION MUTATIONS (-= vs +=, - vs +)
        assert_eq!(subtract_from_cache_size(150, 50), 100);
        assert_eq!(subtract_from_cache_size(100, 100), 0);

        // ✅ CATCHES UNDERFLOW PROTECTION (saturating_sub vs wrapping_sub)
        assert_eq!(subtract_from_cache_size(50, 100), 0); // Should saturate at 0, not wrap
    }

    #[test]
    fn test_max_size_threshold_comparison() {
        // ✅ CATCHES COMPARISON MUTATIONS (>= vs >, >= vs <)
        assert!(is_at_max_size(200, 200)); // Exactly at max
        assert!(is_at_max_size(201, 200)); // Over max
        assert!(!is_at_max_size(199, 200)); // Under max

        // ✅ CATCHES ZERO MAX SIZE HANDLING
        assert!(!is_at_max_size(1000, 0)); // No limit when max is 0
        assert!(!is_at_max_size(0, 0)); // Both zero
    }

    #[test]
    fn test_pool_expansion_threshold_calculation() {
        // ✅ CATCHES PERCENTAGE CALCULATION MUTATIONS (* vs /, / vs *)
        assert!(calculate_pool_expansion_threshold(75, 100, 75.0)); // 75 >= 75% of 100
        assert!(!calculate_pool_expansion_threshold(74, 100, 75.0)); // 74 < 75% of 100
        assert!(calculate_pool_expansion_threshold(80, 100, 75.0)); // 80 > 75% of 100

        // ✅ CATCHES DIVISION BY 100 MUTATIONS
        assert!(calculate_pool_expansion_threshold(8, 10, 75.0)); // 8 >= 7.5 (75% of 10)
        assert!(!calculate_pool_expansion_threshold(7, 10, 75.0)); // 7 < 7.5 (75% of 10)

        // ✅ CATCHES ZERO MAX SIZE HANDLING
        assert!(!calculate_pool_expansion_threshold(100, 0, 50.0)); // No expansion if no limit
    }

    #[test]
    fn test_eviction_count_calculation() {
        // ✅ CATCHES PERCENTAGE CALCULATION MUTATIONS
        assert_eq!(calculate_optimal_eviction_count(100, 25.0), 25); // 25% of 100 = 25
        assert_eq!(calculate_optimal_eviction_count(80, 50.0), 40); // 50% of 80 = 40
        assert_eq!(calculate_optimal_eviction_count(10, 30.0), 3); // 30% of 10 = 3

        // ✅ CATCHES EDGE CASES
        assert_eq!(calculate_optimal_eviction_count(0, 50.0), 0); // No items to evict
        assert_eq!(calculate_optimal_eviction_count(10, 0.0), 0); // 0% eviction

        // ✅ CATCHES MIN FUNCTION MUTATIONS (min vs max)
        assert_eq!(calculate_optimal_eviction_count(5, 200.0), 5); // Can't evict more than total
    }

    #[test]
    fn test_integration_cache_workflow() {
        // ✅ FULL CACHE WORKFLOW TEST - catches mutations in the complete calculation chain
        let current_size = 150;
        let new_item_size = 100;
        let max_size = 200;

        // Step 1: Check if eviction is needed
        assert!(needs_eviction(current_size, new_item_size, max_size));

        // Step 2: Calculate eviction size
        let eviction_needed = calculate_eviction_size(current_size, new_item_size, max_size);
        assert_eq!(eviction_needed, 50); // (150 + 100) - 200 = 50

        // Step 3: Calculate new size after adding item
        let new_total_size = add_to_cache_size(current_size, new_item_size);
        assert_eq!(new_total_size, 250); // 150 + 100 = 250

        // Step 4: Calculate size after eviction
        let final_size = subtract_from_cache_size(new_total_size, eviction_needed);
        assert_eq!(final_size, 200); // 250 - 50 = 200 (exactly at max)

        // Step 5: Verify final state
        assert!(is_at_max_size(final_size, max_size)); // Should be at max size
    }

    #[test]
    fn test_statistics_calculation_integration() {
        // ✅ HIT RATIO CALCULATION WORKFLOW
        let hits = 85;
        let misses = 15;
        let hit_ratio = calculate_hit_ratio(hits, misses);
        assert_eq!(hit_ratio, 0.85); // 85 / (85 + 15) = 0.85

        // ✅ CACHE SIZE MANAGEMENT WORKFLOW
        let initial_size = 0;
        let size_after_add1 = add_to_cache_size(initial_size, 100);
        let size_after_add2 = add_to_cache_size(size_after_add1, 150);
        let size_after_remove = subtract_from_cache_size(size_after_add2, 50);
        assert_eq!(size_after_remove, 200); // 0 + 100 + 150 - 50 = 200
    }
}
