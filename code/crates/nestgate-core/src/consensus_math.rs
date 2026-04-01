// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

/// 🧮 **CONSENSUS MATH MODULE** 🧮
/// Pure arithmetic functions for consensus calculations.
/// Extracted from complex business logic to enable precise testing
/// and catch arithmetic mutations (+ vs -, * vs /, >= vs >).
/// **MUTATION TESTING TARGET**: This module specifically addresses:
/// - `(((active_nodes.len() as f64)) * self.config.min_consensus).ceil() as usize` mutations
/// - `consensus_percentage = ((successful_verifications.len() as f64)) / ((active_nodes.len() as f64))` mutations
/// - Comparison mutations in consensus evaluation
///
/// Calculate required consensus count from node count and minimum threshold
///
/// **PURE FUNCTION**: No side effects, deterministic output
///
/// **TESTABLE**: Can verify exact arithmetic with edge cases
#[must_use]
pub fn calculate_required_consensus(node_count: usize, min_consensus: f64) -> usize {
    if node_count == 0 {
        return 0;
    }
    ((node_count as f64) * min_consensus).ceil() as usize
}

/// Calculate consensus percentage from successful and total counts
/// **PURE FUNCTION**: No side effects, handles division by zero
/// **TESTABLE**: Can verify exact division with floating point precision
#[must_use]
pub fn calculate_consensus_percentage(successful: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        successful as f64 / total as f64
    }
}
/// Check if consensus threshold is achieved
/// **PURE FUNCTION**: Simple comparison logic
/// **TESTABLE**: Can verify boundary conditions precisely
#[must_use]
pub fn is_consensus_achieved(percentage: f64, minimum: f64) -> bool {
    percentage >= minimum
}
/// Calculate consensus expiry from verification results
/// **PURE FUNCTION**: Minimum value calculation
/// **TESTABLE**: Can verify `min()` logic with exact inputs
#[must_use]
pub fn calculate_consensus_expiry(valid_until_times: &[i64], default_duration: i64) -> i64 {
    if valid_until_times.is_empty() {
        // Default expiry: current time + default duration
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64
            + default_duration
    } else {
        // Use minimum expiry from all verifications
        let fallback = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64
            + default_duration;
        *valid_until_times.iter().min().unwrap_or(&fallback)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    /// 🎯 **ARITHMETIC MUTATION DETECTION TESTS**
    /// These tests are specifically designed to catch mutations in arithmetic operations

    #[test]
    fn test_consensus_calculation_exactvalues() {
        // ✅ CATCHES MULTIPLICATION MUTATIONS (* vs +, * vs -)
        assert_eq!(calculate_required_consensus(10, 0.6), 6); // 10 * 0.6 = 6.0 -> ceil -> 6
        assert_eq!(calculate_required_consensus(10, 0.7), 7); // 10 * 0.7 = 7.0 -> ceil -> 7
        assert_eq!(calculate_required_consensus(7, 0.5), 4); // 7 * 0.5 = 3.5 -> ceil -> 4

        // ✅ CATCHES CEILING FUNCTION MUTATIONS (ceil vs floor, ceil vs round)
        assert_eq!(calculate_required_consensus(3, 0.34), 2); // 3 * 0.34 = 1.02 -> ceil -> 2
        assert_eq!(calculate_required_consensus(3, 0.33), 1); // 3 * 0.33 = 0.99 -> ceil -> 1

        // ✅ CATCHES EDGE CASES
        assert_eq!(calculate_required_consensus(1, 0.9), 1); // 1 * 0.9 = 0.9 -> ceil -> 1
        assert_eq!(calculate_required_consensus(0, 0.5), 0); // Edge case: no nodes
    }

    #[test]
    fn test_consensus_percentage_division_precision() {
        // ✅ CATCHES DIVISION MUTATIONS (/ vs *, / vs +, / vs -)
        assert_eq!(calculate_consensus_percentage(3, 10), 0.3);
        assert_eq!(calculate_consensus_percentage(7, 10), 0.7);
        assert_eq!(calculate_consensus_percentage(1, 4), 0.25);

        // ✅ CATCHES DIVISION BY ZERO HANDLING
        assert_eq!(calculate_consensus_percentage(5, 0), 0.0); // Should not panic

        // ✅ CATCHES PRECISION EDGE CASES
        assert_eq!(calculate_consensus_percentage(1, 3), 1.0 / 3.0); // Repeating decimal
        assert_eq!(calculate_consensus_percentage(0, 10), 0.0); // Zero successful
        assert_eq!(calculate_consensus_percentage(10, 10), 1.0); // All successful
    }

    #[test]
    fn test_consensus_threshold_comparison_boundary() {
        // ✅ CATCHES COMPARISON MUTATIONS (>= vs >, >= vs <, >= vs <=)
        assert!(is_consensus_achieved(0.6, 0.6)); // Exactly at threshold
        assert!(is_consensus_achieved(0.61, 0.6)); // Just above threshold
        assert!(!is_consensus_achieved(0.59, 0.6)); // Just below threshold

        // ✅ CATCHES EXTREME VALUES
        assert!(is_consensus_achieved(1.0, 0.9)); // 100% vs 90% requirement
        assert!(!is_consensus_achieved(0.0, 0.1)); // 0% vs 10% requirement
        assert!(is_consensus_achieved(0.5, 0.5)); // Exactly equal

        // ✅ CATCHES FLOATING POINT PRECISION ISSUES
        let calculated = calculate_consensus_percentage(6, 10); // Should be exactly 0.6
        assert!(is_consensus_achieved(calculated, 0.6));
    }

    #[test]
    fn test_consensus_expiry_minimum_calculation() {
        // ✅ CATCHES MIN/MAX MUTATIONS (min vs max)
        let times = vec![1000, 2000, 1500, 3000];
        assert_eq!(calculate_consensus_expiry(&times, 3600), 1000); // Should be minimum

        // ✅ CATCHES EMPTY SLICE HANDLING
        let empty: Vec<i64> = vec![];
        let result = calculate_consensus_expiry(&empty, 3600);
        assert!(result > 0); // Should return reasonable default

        // ✅ CATCHES SINGLE VALUE CASE
        let single = vec![5000];
        assert_eq!(calculate_consensus_expiry(&single, 3600), 5000);
    }

    #[test]
    fn test_integration_consensus_workflow() {
        // ✅ FULL WORKFLOW TEST - catches mutations in the complete calculation chain
        let node_count = 10;
        let min_consensus = 0.6;
        let successful_verifications = 7;

        // Step 1: Calculate required consensus
        let required = calculate_required_consensus(node_count, min_consensus);
        assert_eq!(required, 6); // 10 * 0.6 = 6

        // Step 2: Calculate actual percentage
        let percentage = calculate_consensus_percentage(successful_verifications, node_count);
        assert_eq!(percentage, 0.7); // 7/10 = 0.7

        // Step 3: Check if consensus achieved
        assert!(is_consensus_achieved(percentage, min_consensus)); // 0.7 >= 0.6

        // ✅ VERIFY FAILURE CASE
        let insufficient_verifications = 5;
        let low_percentage = calculate_consensus_percentage(insufficient_verifications, node_count);
        assert_eq!(low_percentage, 0.5); // 5/10 = 0.5
        assert!(!is_consensus_achieved(low_percentage, min_consensus)); // 0.5 < 0.6
    }
}
