//! Additional tests for port defaults to boost coverage
//!
//! These tests verify environment variable handling and edge cases

#[cfg(test)]
mod additional_port_tests {
    use crate::constants::port_defaults::*;
    use std::env;

    #[test]
    fn test_all_port_constants_are_valid() {
        // Verify all port constants are set to non-zero values
        assert_ne!(DEFAULT_API_PORT, 0);
        assert_ne!(DEFAULT_ADMIN_PORT, 0);
        assert_ne!(DEFAULT_METRICS_PORT, 0);
        assert_ne!(DEFAULT_HEALTH_PORT, 0);
        assert_ne!(DEFAULT_DEV_PORT, 0);
        assert_ne!(DEFAULT_DEV_ALT_PORT, 0);
        assert_ne!(DEFAULT_POSTGRES_PORT, 0);
        assert_ne!(DEFAULT_MYSQL_PORT, 0);
        assert_ne!(DEFAULT_MONGODB_PORT, 0);
        assert_ne!(DEFAULT_REDIS_PORT, 0);
        assert_ne!(DEFAULT_PROMETHEUS_PORT, 0);
        assert_ne!(DEFAULT_GRAFANA_PORT, 0);
        assert_ne!(DEFAULT_JAEGER_PORT, 0);
        assert_ne!(DEFAULT_RABBITMQ_PORT, 0);
        assert_ne!(DEFAULT_KAFKA_PORT, 0);
    }

    #[test]
    fn test_port_constants_are_distinct() {
        // Most ports should be distinct to avoid conflicts
        // (Some may intentionally share, like metrics and prometheus)
        assert_ne!(DEFAULT_API_PORT, DEFAULT_ADMIN_PORT);
        assert_ne!(DEFAULT_API_PORT, DEFAULT_HEALTH_PORT);
        assert_ne!(DEFAULT_ADMIN_PORT, DEFAULT_HEALTH_PORT);
    }

    #[test]
    #[serial_test::serial] // Ensure env vars don't interfere
    fn test_get_api_port_with_env() {
        env::set_var("NESTGATE_API_PORT", "9999");
        assert_eq!(get_api_port(), 9999);
        env::remove_var("NESTGATE_API_PORT");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_api_port_without_env() {
        env::remove_var("NESTGATE_API_PORT");
        assert_eq!(get_api_port(), DEFAULT_API_PORT);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_api_port_invalid_env() {
        env::set_var("NESTGATE_API_PORT", "invalid");
        assert_eq!(get_api_port(), DEFAULT_API_PORT); // Should fall back to default
        env::remove_var("NESTGATE_API_PORT");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_metrics_port_with_env() {
        env::set_var("NESTGATE_METRICS_PORT", "9191");
        assert_eq!(get_metrics_port(), 9191);
        env::remove_var("NESTGATE_METRICS_PORT");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_health_port_with_env() {
        env::set_var("NESTGATE_HEALTH_PORT", "8888");
        assert_eq!(get_health_port(), 8888);
        env::remove_var("NESTGATE_HEALTH_PORT");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_admin_port_with_env() {
        env::set_var("NESTGATE_ADMIN_PORT", "7777");
        assert_eq!(get_admin_port(), 7777);
        env::remove_var("NESTGATE_ADMIN_PORT");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_dev_port_with_env() {
        env::set_var("NESTGATE_DEV_PORT", "4000");
        assert_eq!(get_dev_port(), 4000);
        env::remove_var("NESTGATE_DEV_PORT");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_postgres_port_with_env() {
        env::set_var("NESTGATE_POSTGRES_PORT", "5433");
        assert_eq!(get_postgres_port(), 5433);
        env::remove_var("NESTGATE_POSTGRES_PORT");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_redis_port_with_env() {
        env::set_var("NESTGATE_REDIS_PORT", "6380");
        assert_eq!(get_redis_port(), 6380);
        env::remove_var("NESTGATE_REDIS_PORT");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_prometheus_port_with_env() {
        env::set_var("NESTGATE_PROMETHEUS_PORT", "9091");
        assert_eq!(get_prometheus_port(), 9091);
        env::remove_var("NESTGATE_PROMETHEUS_PORT");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_grafana_port_with_env() {
        env::set_var("NESTGATE_GRAFANA_PORT", "4000");
        assert_eq!(get_grafana_port(), 4000);
        env::remove_var("NESTGATE_GRAFANA_PORT");
    }

    #[test]
    fn test_parse_port_edge_cases() {
        // Valid ports
        assert_eq!(parse_port("1"), Some(1));
        assert_eq!(parse_port("80"), Some(80));
        assert_eq!(parse_port("443"), Some(443));
        assert_eq!(parse_port("8080"), Some(8080));
        assert_eq!(parse_port("65535"), Some(65535));

        // Invalid ports
        assert_eq!(parse_port("0"), None); // Port 0 is invalid
        assert_eq!(parse_port("65536"), None); // > max port
        assert_eq!(parse_port("99999"), None); // Way too large
        assert_eq!(parse_port("-1"), None); // Negative
        assert_eq!(parse_port("abc"), None); // Not a number
        assert_eq!(parse_port(""), None); // Empty
        assert_eq!(parse_port("  "), None); // Whitespace
        assert_eq!(parse_port("80 80"), None); // Multiple numbers
    }

    #[test]
    fn test_common_service_ports() {
        // Verify we use standard ports for common services
        assert_eq!(DEFAULT_POSTGRES_PORT, 5432);
        assert_eq!(DEFAULT_MYSQL_PORT, 3306);
        assert_eq!(DEFAULT_REDIS_PORT, 6379);
        assert_eq!(DEFAULT_MONGODB_PORT, 27017);
        assert_eq!(DEFAULT_RABBITMQ_PORT, 5672);
        assert_eq!(DEFAULT_KAFKA_PORT, 9092);
    }

    #[test]
    fn test_nestgate_service_ports() {
        // Verify NestGate uses 808x range for its services
        assert_eq!(DEFAULT_API_PORT, 8080);
        assert_eq!(DEFAULT_ADMIN_PORT, 8081);
        assert_eq!(DEFAULT_HEALTH_PORT, 8082);

        // Metrics uses standard Prometheus port
        assert_eq!(DEFAULT_PROMETHEUS_PORT, 9090);
        assert_eq!(DEFAULT_METRICS_PORT, 9090);
    }

    #[test]
    fn test_development_ports() {
        // Verify development ports are in common range
        assert_eq!(DEFAULT_DEV_PORT, 3000);
        assert_eq!(DEFAULT_DEV_ALT_PORT, 5000);
        assert_eq!(DEFAULT_GRAFANA_PORT, 3001);
    }
}
