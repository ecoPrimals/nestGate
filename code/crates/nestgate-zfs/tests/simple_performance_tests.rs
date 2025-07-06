//! Simple Performance Engine Tests

use nestgate_zfs::performance_engine::PerformanceEngineConfig;

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_performance_config_defaults() {
        let config = PerformanceEngineConfig::default();

        assert_eq!(config.latency_threshold_ms, 10.0);
        assert_eq!(config.cache_hit_threshold, 0.80);
        assert_eq!(config.fragmentation_threshold, 25.0);
        assert_eq!(config.arc_hit_threshold, 0.85);
        assert_eq!(config.optimization_interval_seconds, 60);
        assert_eq!(config.monitoring_interval_seconds, 10);
    }
}
