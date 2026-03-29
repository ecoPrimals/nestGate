// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Edge Case Tests for NestGate Core
//!
//! This module contains critical edge case tests focusing on:
//! - Boundary conditions in math operations
//! - String handling edge cases
//! - Concurrent access patterns
//! - Error handling chains
//! - Resource limits

#[cfg(test)]
mod edge_cases {
    use crate::cache_math::{calculate_total_cache_size, needs_eviction};
    use crate::consensus_math::{
        calculate_consensus_percentage, calculate_required_consensus, is_consensus_achieved,
    };
    use crate::error::{NestGateError, Result};
    use crate::validation_predicates::{
        is_non_empty_string, is_valid_consensus_threshold, is_valid_file_path, is_valid_port_number,
    };
    use std::sync::Arc;
    // RwLock import removed - not used in tests

    /// Edge Case 1: Cache math with zero max size
    #[test]
    fn test_edge_cache_zero_max_size() {
        // When max_size is 0, eviction should never be needed
        assert!(!needs_eviction(1000, 1000, 0));
        assert!(!needs_eviction(u64::MAX, u64::MAX, 0));
    }

    /// Edge Case 2: Cache math with maximum values
    #[test]
    fn test_edge_cache_maximum_values() {
        let max_val = u64::MAX;
        // At boundary - should not overflow
        assert!(!needs_eviction(max_val / 2, max_val / 2, max_val));
        // Over boundary
        assert!(needs_eviction(max_val / 2, max_val / 2 + 2, max_val));
    }

    /// Edge Case 3: Total cache size with empty slice
    #[test]
    fn test_edge_empty_cache_sizes() {
        assert_eq!(calculate_total_cache_size(&[]), 0);
    }

    /// Edge Case 4: Consensus threshold at exact boundaries
    #[test]
    fn test_edge_consensus_exact_boundaries() {
        // Exactly at minimum
        assert!(is_valid_consensus_threshold(0.5));
        // Just below minimum
        assert!(!is_valid_consensus_threshold(0.499));
        // Exactly at maximum
        assert!(is_valid_consensus_threshold(1.0));
        // Just above maximum
        assert!(!is_valid_consensus_threshold(1.001));
    }

    /// Edge Case 5: Consensus calculation with edge vote counts
    #[test]
    fn test_edge_consensus_vote_counts() {
        // Single voter
        assert_eq!(calculate_required_consensus(1, 0.5), 1);
        // Two voters, 50% threshold
        assert_eq!(calculate_required_consensus(2, 0.5), 1);
        // Two voters, >50% threshold
        assert_eq!(calculate_required_consensus(2, 0.51), 2);
        // Zero voters
        assert_eq!(calculate_required_consensus(0, 0.5), 0);
    }

    /// Edge Case 6: Consensus percentage calculation
    #[test]
    fn test_edge_consensus_percentage() {
        assert_eq!(calculate_consensus_percentage(10, 10), 1.0);
        assert_eq!(calculate_consensus_percentage(5, 10), 0.5);
        assert_eq!(calculate_consensus_percentage(0, 10), 0.0);
    }

    /// Edge Case 7: String validation with whitespace
    #[test]
    fn test_edge_string_whitespace_only() {
        // Note: is_non_empty_string checks .is_empty(), not .trim().is_empty()
        // So whitespace-only strings are considered "non-empty"
        assert!(is_non_empty_string("   ")); // Contains chars
        assert!(is_non_empty_string("\t\t\t")); // Contains chars
        assert!(is_non_empty_string("\n\n\n")); // Contains chars
        assert!(!is_non_empty_string("")); // Actually empty
    }

    /// Edge Case 8: File path validation with special characters
    #[test]
    fn test_edge_file_path_special_chars() {
        // Path traversal attempts
        assert!(!is_valid_file_path("../../etc/passwd"));
        assert!(!is_valid_file_path("../../../root"));
        assert!(!is_valid_file_path("./../secret"));

        // Valid paths
        assert!(is_valid_file_path("/home/user/file.txt"));
        assert!(is_valid_file_path("./local/file.txt"));
    }

    /// Edge Case 9: Port validation at boundaries
    #[test]
    fn test_edge_port_boundaries() {
        // Port 0 is typically invalid
        assert!(!is_valid_port_number(0));
        // Port 1 is valid (but privileged)
        assert!(is_valid_port_number(1));
        // Maximum port
        assert!(is_valid_port_number(65535));
        // Common valid ports
        assert!(is_valid_port_number(80));
        assert!(is_valid_port_number(443));
        assert!(is_valid_port_number(8080));
    }

    /// Edge Case 10: Error message handling with special characters
    #[test]
    fn test_edge_error_special_characters() {
        let err =
            NestGateError::internal_error("Error with 'quotes' and \"double quotes\"", "test");
        let err_str = format!("{}", err);
        assert!(err_str.contains("quotes"));

        let err2 = NestGateError::internal_error("Error\nwith\nnewlines", "test");
        let err_str2 = format!("{}", err2);
        assert!(err_str2.contains("Error"));
    }

    /// Edge Case 11: Concurrent error creation
    #[tokio::test]
    async fn test_edge_concurrent_error_creation() {
        let mut handles = vec![];

        for i in 0..100 {
            let handle = tokio::spawn(async move {
                let err = NestGateError::internal_error(format!("Error {}", i), "concurrent_test");
                format!("{}", err)
            });
            handles.push(handle);
        }

        // All should complete successfully
        for (i, handle) in handles.into_iter().enumerate() {
            let result = handle.await.expect("Task should complete");
            assert!(result.contains(&format!("Error {}", i)) || result.contains("concurrent_test"));
        }
    }

    /// Edge Case 12: Cache size calculation with large slice
    #[test]
    fn test_edge_cache_large_slice() {
        let sizes: Vec<u64> = (0..10000).map(|i| i as u64).collect();
        let total = calculate_total_cache_size(&sizes);
        // Sum of 0..9999 = (n * (n-1)) / 2
        let expected: u64 = (10000 * 9999) / 2;
        assert_eq!(total, expected);
    }

    /// Edge Case 13: Consensus achievement check
    #[test]
    fn test_edge_consensus_achievement() {
        assert!(is_consensus_achieved(0.75, 0.5));
        assert!(is_consensus_achieved(0.5, 0.5));
        assert!(!is_consensus_achieved(0.4, 0.5));
    }

    /// Edge Case 14: Very long error messages
    #[test]
    fn test_edge_very_long_error_message() {
        let long_msg = "x".repeat(10000);
        let err = NestGateError::internal_error(long_msg, "test");
        let err_str = format!("{}", err);
        assert!(!err_str.is_empty());
    }

    /// Edge Case 15: Concurrent cache calculations
    #[tokio::test]
    async fn test_edge_concurrent_cache_calculations() {
        let sizes = Arc::new(vec![100u64, 200, 300, 400, 500]);
        let mut handles = vec![];

        for _ in 0..50 {
            let sizes_clone = Arc::clone(&sizes);
            let handle = tokio::spawn(async move { calculate_total_cache_size(&sizes_clone) });
            handles.push(handle);
        }

        // All should return the same result
        for handle in handles {
            let result = handle.await.expect("Task should complete");
            assert_eq!(result, 1500);
        }
    }

    /// Edge Case 16: Consensus required rounding
    #[test]
    fn test_edge_consensus_required_rounding() {
        // Test that rounding works correctly
        let required = calculate_required_consensus(3, 0.667);
        assert!(required >= 2); // Should round up

        let required2 = calculate_required_consensus(10, 0.75);
        assert_eq!(required2, 8); // 10 * 0.75 = 7.5, rounds to 8
    }

    /// Edge Case 17: String validation with unicode
    #[test]
    fn test_edge_string_unicode() {
        assert!(is_non_empty_string("こんにちは")); // Japanese
        assert!(is_non_empty_string("مرحبا")); // Arabic
        assert!(is_non_empty_string("🚀🔥💻")); // Emojis
        assert!(is_non_empty_string("Здравствуйте")); // Russian
    }

    /// Edge Case 18: File path validation with unicode
    #[test]
    fn test_edge_file_path_unicode() {
        // Unicode in paths should be handled correctly
        assert!(is_valid_file_path("/home/用户/文件.txt"));
        assert!(is_valid_file_path("/домой/файл.txt"));
    }

    /// Edge Case 19: Error propagation through Result chains
    #[test]
    fn test_edge_error_result_chains() {
        /// Operation
        fn operation() -> Result<u32> {
            Err(NestGateError::internal_error("base error", "test"))
        }

        let result = operation().map(|x| x + 1).map(|x| x * 2);

        assert!(result.is_err());
    }

    /// Edge Case 20: Concurrent validation operations
    #[tokio::test]
    async fn test_edge_concurrent_validations() {
        let mut handles = vec![];

        for i in 1..=100 {
            let handle = tokio::spawn(async move { is_valid_port_number(i) });
            handles.push(handle);
        }

        // All should complete
        for handle in handles {
            handle.await.expect("Task should complete");
        }
    }
}
