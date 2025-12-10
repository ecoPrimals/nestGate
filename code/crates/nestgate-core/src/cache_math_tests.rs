//! Cache Math Tests
//!
//! Comprehensive tests for cache calculation utilities to achieve 100% coverage.

#[cfg(test)]
mod cache_math_tests {
    use super::*;

    #[test]
    fn test_optimal_cache_size_calculation() {
        // Test with typical values
        let available_memory = 1024 * 1024 * 1024; // 1GB
        let cache_size = calculate_optimal_cache_size(available_memory);
        
        assert!(cache_size > 0);
        assert!(cache_size <= available_memory);
        assert!(cache_size > available_memory / 10); // At least 10% used
    }

    #[test]
    fn test_cache_size_with_zero_memory() {
        let cache_size = calculate_optimal_cache_size(0);
        assert_eq!(cache_size, 0);
    }

    #[test]
    fn test_cache_size_with_small_memory() {
        let small_memory = 1024; // 1KB
        let cache_size = calculate_optimal_cache_size(small_memory);
        assert!(cache_size <= small_memory);
    }

    #[test]
    fn test_cache_hit_ratio_calculation() {
        let hits = 80;
        let total = 100;
        let ratio = calculate_cache_hit_ratio(hits, total);
        
        assert!((ratio - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_cache_hit_ratio_zero_total() {
        let ratio = calculate_cache_hit_ratio(0, 0);
        assert_eq!(ratio, 0.0);
    }

    #[test]
    fn test_cache_hit_ratio_perfect() {
        let ratio = calculate_cache_hit_ratio(100, 100);
        assert_eq!(ratio, 1.0);
    }

    #[test]
    fn test_cache_hit_ratio_zero_hits() {
        let ratio = calculate_cache_hit_ratio(0, 100);
        assert_eq!(ratio, 0.0);
    }

    #[test]
    fn test_cache_eviction_threshold() {
        let cache_size = 1024;
        let threshold = calculate_eviction_threshold(cache_size);
        
        assert!(threshold > 0);
        assert!(threshold <= cache_size);
    }

    #[test]
    fn test_cache_entry_size_estimation() {
        let key_len = 32;
        let value_len = 128;
        let entry_size = estimate_cache_entry_size(key_len, value_len);
        
        // Entry size should include overhead + key + value
        assert!(entry_size >= key_len + value_len);
    }

    #[test]
    fn test_max_cache_entries_calculation() {
        let total_cache_size = 1024 * 1024; // 1MB
        let avg_entry_size = 1024; // 1KB
        let max_entries = calculate_max_cache_entries(total_cache_size, avg_entry_size);
        
        assert!(max_entries > 0);
        assert!(max_entries <= total_cache_size / avg_entry_size);
    }
}

