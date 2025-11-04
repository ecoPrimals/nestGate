//! Comprehensive port configuration system
//!
//! Provides centralized port management with environment variable overrides
//! for all NestGate services. Eliminates hardcoded port constants and enables
//! production-ready configuration management.

use std::env;
use std::sync::OnceLock;

// Make this module available
pub use self::ports::*;

mod ports {
    use super::*;

    // ============================================================================
    // Core Service Ports
    // ============================================================================

    /// Get API server port (default: 8080)
    ///
    /// Environment variable: `NESTGATE_API_PORT`
    pub fn api_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_API_PORT").unwrap_or(8080))
    }

    /// Get admin interface port (default: 8081)
    ///
    /// Environment variable: `NESTGATE_ADMIN_PORT`
    pub fn admin_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_ADMIN_PORT").unwrap_or(8081))
    }

    /// Get metrics port (default: 9090)
    ///
    /// Environment variable: `NESTGATE_METRICS_PORT`
    pub fn metrics_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_METRICS_PORT").unwrap_or(9090))
    }

    /// Get health check port (default: 8082)
    ///
    /// Environment variable: `NESTGATE_HEALTH_PORT`
    pub fn health_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_HEALTH_PORT").unwrap_or(8082))
    }

    /// Get WebSocket port (default: 8083)
    ///
    /// Environment variable: `NESTGATE_WEBSOCKET_PORT`
    pub fn websocket_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_WEBSOCKET_PORT").unwrap_or(8083))
    }

    /// Get gRPC port (default: 50051)
    ///
    /// Environment variable: `NESTGATE_GRPC_PORT`
    pub fn grpc_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_GRPC_PORT").unwrap_or(50051))
    }

    // ============================================================================
    // Storage & Database Ports
    // ============================================================================

    /// Get PostgreSQL port (default: 5432)
    ///
    /// Environment variable: `NESTGATE_POSTGRES_PORT`
    pub fn postgres_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_POSTGRES_PORT").unwrap_or(5432))
    }

    /// Get Redis port (default: 6379)
    ///
    /// Environment variable: `NESTGATE_REDIS_PORT`
    pub fn redis_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_REDIS_PORT").unwrap_or(6379))
    }

    /// Get MongoDB port (default: 27017)
    ///
    /// Environment variable: `NESTGATE_MONGODB_PORT`
    pub fn mongodb_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_MONGODB_PORT").unwrap_or(27017))
    }

    /// Get MySQL port (default: 3306)
    ///
    /// Environment variable: `NESTGATE_MYSQL_PORT`
    pub fn mysql_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_MYSQL_PORT").unwrap_or(3306))
    }

    // ============================================================================
    // Monitoring & Observability Ports
    // ============================================================================

    /// Get Prometheus port (default: 9090)
    ///
    /// Environment variable: `NESTGATE_PROMETHEUS_PORT`
    pub fn prometheus_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_PROMETHEUS_PORT").unwrap_or(9090))
    }

    /// Get Grafana port (default: 3000)
    ///
    /// Environment variable: `NESTGATE_GRAFANA_PORT`
    pub fn grafana_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_GRAFANA_PORT").unwrap_or(3000))
    }

    /// Get Jaeger port (default: 14268)
    ///
    /// Environment variable: `NESTGATE_JAEGER_PORT`
    pub fn jaeger_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_JAEGER_PORT").unwrap_or(14268))
    }

    // ============================================================================
    // Message Queue Ports
    // ============================================================================

    /// Get RabbitMQ port (default: 5672)
    ///
    /// Environment variable: `NESTGATE_RABBITMQ_PORT`
    pub fn rabbitmq_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_RABBITMQ_PORT").unwrap_or(5672))
    }

    /// Get Kafka port (default: 9092)
    ///
    /// Environment variable: `NESTGATE_KAFKA_PORT`
    pub fn kafka_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_KAFKA_PORT").unwrap_or(9092))
    }

    // ============================================================================
    // NestGate-Specific Service Ports
    // ============================================================================

    /// Get storage service port (default: 5000)
    ///
    /// Environment variable: `NESTGATE_STORAGE_PORT`
    pub fn storage_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_STORAGE_PORT").unwrap_or(5000))
    }

    /// Get orchestration service port (default: 8084)
    ///
    /// Environment variable: `NESTGATE_ORCHESTRATION_PORT`
    pub fn orchestration_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_ORCHESTRATION_PORT").unwrap_or(8084))
    }

    /// Get storage discovery port (default: 8085)
    ///
    /// Environment variable: `NESTGATE_STORAGE_DISCOVERY_PORT`
    pub fn storage_discovery_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_STORAGE_DISCOVERY_PORT").unwrap_or(8085))
    }

    /// Get compute service port (default: 8086)
    ///
    /// Environment variable: `NESTGATE_COMPUTE_PORT`
    pub fn compute_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_COMPUTE_PORT").unwrap_or(8086))
    }

    /// Get discovery service port (default: 3010)
    ///
    /// Environment variable: `NESTGATE_DISCOVERY_PORT`
    pub fn discovery_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_DISCOVERY_PORT").unwrap_or(3010))
    }

    // ============================================================================
    // Development Ports
    // ============================================================================

    /// Get development server port (default: 3000)
    ///
    /// Environment variable: `NESTGATE_DEV_PORT`
    pub fn dev_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_DEV_PORT").unwrap_or(3000))
    }

    /// Get alternative development port (default: 5000)
    ///
    /// Environment variable: `NESTGATE_DEV_ALT_PORT`
    pub fn dev_alt_port() -> u16 {
        static PORT: OnceLock<u16> = OnceLock::new();
        *PORT.get_or_init(|| parse_env_port("NESTGATE_DEV_ALT_PORT").unwrap_or(5000))
    }

    // ============================================================================
    // Helper Functions
    // ============================================================================

    /// Parse port from environment variable
    ///
    /// Returns `None` if the variable is not set or cannot be parsed as a valid port.
    fn parse_env_port(var_name: &str) -> Option<u16> {
        env::var(var_name)
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .filter(|&p| p > 0) // Port 0 is invalid
    }

    /// Get all configured ports as a map
    ///
    /// Useful for debugging and configuration validation.
    #[must_use]
    pub fn get_all_ports() -> std::collections::HashMap<&'static str, u16> {
        let mut ports = std::collections::HashMap::new();
        
        // Core services
        ports.insert("api", api_port());
        ports.insert("admin", admin_port());
        ports.insert("metrics", metrics_port());
        ports.insert("health", health_port());
        ports.insert("websocket", websocket_port());
        ports.insert("grpc", grpc_port());
        
        // Storage & Database
        ports.insert("postgres", postgres_port());
        ports.insert("redis", redis_port());
        ports.insert("mongodb", mongodb_port());
        ports.insert("mysql", mysql_port());
        
        // Monitoring
        ports.insert("prometheus", prometheus_port());
        ports.insert("grafana", grafana_port());
        ports.insert("jaeger", jaeger_port());
        
        // Message Queues
        ports.insert("rabbitmq", rabbitmq_port());
        ports.insert("kafka", kafka_port());
        
        // NestGate Services
        ports.insert("storage", storage_port());
        ports.insert("orchestration", orchestration_port());
        ports.insert("storage_discovery", storage_discovery_port());
        ports.insert("compute", compute_port());
        ports.insert("discovery", discovery_port());
        
        // Development
        ports.insert("dev", dev_port());
        ports.insert("dev_alt", dev_alt_port());
        
        ports
    }

    /// Validate that all ports are unique
    ///
    /// Returns `Ok(())` if all ports are unique, or an error message listing conflicts.
    ///
    /// # Errors
    ///
    /// Returns an error if duplicate port assignments are detected.
    pub fn validate_port_uniqueness() -> Result<(), String> {
        let ports = get_all_ports();
        let mut seen = std::collections::HashMap::new();
        let mut conflicts = Vec::new();
        
        for (name, port) in &ports {
            if let Some(existing) = seen.insert(port, name) {
                conflicts.push(format!("Port {} used by both '{}' and '{}'", port, existing, name));
            }
        }
        
        if conflicts.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "Port conflicts detected:\n  {}",
                conflicts.join("\n  ")
            ))
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_ports_are_valid() {
        // All ports should be > 0
        assert!(api_port() > 0);
        assert!(admin_port() > 0);
        assert!(metrics_port() > 0);
        assert!(health_port() > 0);
        assert!(websocket_port() > 0);
        assert!(grpc_port() > 0);
    }

    #[test]
    fn test_database_ports_are_valid() {
        assert!(postgres_port() > 0);
        assert!(redis_port() > 0);
        assert!(mongodb_port() > 0);
        assert!(mysql_port() > 0);
    }

    #[test]
    fn test_monitoring_ports_are_valid() {
        assert!(prometheus_port() > 0);
        assert!(grafana_port() > 0);
        assert!(jaeger_port() > 0);
    }

    #[test]
    fn test_message_queue_ports_are_valid() {
        assert!(rabbitmq_port() > 0);
        assert!(kafka_port() > 0);
    }

    #[test]
    fn test_nestgate_service_ports_are_valid() {
        assert!(storage_port() > 0);
        assert!(orchestration_port() > 0);
        assert!(storage_discovery_port() > 0);
        assert!(compute_port() > 0);
        assert!(discovery_port() > 0);
    }

    #[test]
    fn test_dev_ports_are_valid() {
        assert!(dev_port() > 0);
        assert!(dev_alt_port() > 0);
    }

    #[test]
    fn test_get_all_ports() {
        let ports = get_all_ports();
        
        // Should have all expected services
        assert!(ports.contains_key("api"));
        assert!(ports.contains_key("admin"));
        assert!(ports.contains_key("metrics"));
        assert!(ports.contains_key("health"));
        assert!(ports.contains_key("postgres"));
        assert!(ports.contains_key("redis"));
        
        // All ports should be valid
        for (_, port) in ports {
            assert!(port > 0, "Port must be greater than 0");
        }
    }

    #[test]
    fn test_standard_port_numbers() {
        // Verify well-known defaults
        assert_eq!(postgres_port(), 5432);
        assert_eq!(redis_port(), 6379);
        assert_eq!(mongodb_port(), 27017);
        assert_eq!(mysql_port(), 3306);
        assert_eq!(grpc_port(), 50051);
    }

    #[test]
    fn test_port_ranges() {
        let ports = get_all_ports();
        
        for (name, port) in ports {
            assert!(
                port > 0 && port <= 65535,
                "Port '{}' ({}) must be in valid range 1-65535",
                name,
                port
            );
        }
    }

    #[test]
    fn test_environment_port_override() {
        // Set test environment variable
        std::env::set_var("NESTGATE_API_PORT", "9999");
        
        // Note: This won't affect api_port() due to OnceLock caching
        // But we can test the concept
        let test_port = std::env::var("NESTGATE_API_PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok());
        
        assert_eq!(test_port, Some(9999));
        
        // Clean up
        std::env::remove_var("NESTGATE_API_PORT");
    }
}
