//! Basic Performance Engine Tests

use nestgate_zfs::performance_engine::PerformanceEngineConfig;

#[cfg(test)]
mod basic_tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = PerformanceEngineConfig::default();
        assert_eq!(config.latency_threshold_ms, 10.0);
    }
} 