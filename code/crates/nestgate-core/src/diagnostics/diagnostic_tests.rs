//! Comprehensive tests for diagnostic entry module
//! Added: November 14, 2025 - Real Production Tests

#[cfg(test)]
mod diagnostic_tests {
    use crate::diagnostics::diagnostic::*;
    use crate::diagnostics::types::{ComponentType, DiagnosticLevel};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_diagnostic_new() {
        let diag = Diagnostic::new(
            DiagnosticLevel::Warning,
            ComponentType::Memory,
            "Memory usage high".to_string(),
        );

        assert!(!diag.id.is_empty());
        assert_eq!(diag.level, DiagnosticLevel::Warning);
        assert_eq!(diag.component, ComponentType::Memory);
        assert_eq!(diag.message, "Memory usage high");
        assert!(!diag.resolved);
        assert!(diag.resolved_at.is_none());
        assert!(diag.details.is_none());
        assert!(diag.path.is_none());
    }

    #[test]
    fn test_diagnostic_info() {
        let diag = Diagnostic::info(ComponentType::System, "System started".to_string());

        assert_eq!(diag.level, DiagnosticLevel::Info);
        assert_eq!(diag.component, ComponentType::System);
        assert_eq!(diag.message, "System started");
    }

    #[test]
    fn test_diagnostic_warning() {
        let diag = Diagnostic::warning(ComponentType::Network, "High latency".to_string());

        assert_eq!(diag.level, DiagnosticLevel::Warning);
        assert_eq!(diag.component, ComponentType::Network);
        assert_eq!(diag.message, "High latency");
    }

    #[test]
    fn test_diagnostic_error() {
        let diag = Diagnostic::error(ComponentType::Database, "Connection failed".to_string());

        assert_eq!(diag.level, DiagnosticLevel::Error);
        assert_eq!(diag.component, ComponentType::Database);
        assert_eq!(diag.message, "Connection failed");
    }

    #[test]
    fn test_diagnostic_critical() {
        let diag = Diagnostic::critical(ComponentType::Storage, "Disk full".to_string());

        assert_eq!(diag.level, DiagnosticLevel::Critical);
        assert_eq!(diag.component, ComponentType::Storage);
        assert_eq!(diag.message, "Disk full");
    }

    #[test]
    fn test_diagnostic_with_details() {
        let diag = Diagnostic::warning(ComponentType::Cpu, "CPU usage high".to_string())
            .with_details("CPU at 95% for 5 minutes".to_string());

        assert_eq!(diag.details, Some("CPU at 95% for 5 minutes".to_string()));
    }

    #[test]
    fn test_diagnostic_with_resource() {
        let diag = Diagnostic::error(ComponentType::Storage, "Read error".to_string())
            .with_resource("/dev/sda1");

        assert_eq!(diag.path, Some("/dev/sda1".to_string()));
    }

    #[test]
    fn test_diagnostic_chaining() {
        let diag = Diagnostic::critical(ComponentType::Memory, "Out of memory".to_string())
            .with_details("Only 100MB remaining".to_string())
            .with_resource("/proc/meminfo");

        assert_eq!(diag.level, DiagnosticLevel::Critical);
        assert_eq!(diag.details, Some("Only 100MB remaining".to_string()));
        assert_eq!(diag.path, Some("/proc/meminfo".to_string()));
    }

    #[test]
    fn test_diagnostic_resolve() {
        let mut diag = Diagnostic::error(ComponentType::Network, "Connection lost".to_string());

        assert!(!diag.resolved);
        assert!(diag.resolved_at.is_none());

        diag.resolve();

        assert!(diag.resolved);
        assert!(diag.resolved_at.is_some());
    }

    #[test]
    fn test_diagnostic_is_severe() {
        let info = Diagnostic::info(ComponentType::System, "Info message".to_string());
        let warning = Diagnostic::warning(ComponentType::System, "Warning message".to_string());
        let error = Diagnostic::error(ComponentType::System, "Error message".to_string());
        let critical = Diagnostic::critical(ComponentType::System, "Critical message".to_string());

        assert!(!info.is_severe());
        assert!(!warning.is_severe());
        assert!(error.is_severe());
        assert!(critical.is_severe());
    }

    #[test]
    fn test_diagnostic_is_unresolved() {
        let mut diag =
            Diagnostic::warning(ComponentType::Cache, "Cache miss rate high".to_string());

        assert!(diag.is_unresolved());

        diag.resolve();

        assert!(!diag.is_unresolved());
    }

    #[test]
    fn test_diagnostic_age_seconds() {
        let diag = Diagnostic::info(ComponentType::Application, "App started".to_string());

        // Should be very recent
        let age = diag.age_seconds();
        assert!(age < 2, "Diagnostic should be less than 2 seconds old");

        // Wait a bit and check age increases
        thread::sleep(Duration::from_millis(100));
        let age_after = diag.age_seconds();
        assert!(age_after >= age, "Age should increase with time");
    }

    #[test]
    fn test_diagnostic_unique_ids() {
        let diag1 = Diagnostic::info(ComponentType::System, "Message 1".to_string());
        let diag2 = Diagnostic::info(ComponentType::System, "Message 1".to_string());

        // Even with same message, IDs should be unique
        assert_ne!(diag1.id, diag2.id);
    }

    #[test]
    fn test_diagnostic_serialization() {
        let diag = Diagnostic::warning(ComponentType::Network, "Network slow".to_string())
            .with_details("Latency 500ms".to_string())
            .with_resource("eth0");

        let json = serde_json::to_string(&diag).unwrap();
        assert!(json.contains("Network slow"));
        assert!(json.contains("Latency 500ms"));
        assert!(json.contains("eth0"));

        let deserialized: Diagnostic = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.message, diag.message);
        assert_eq!(deserialized.level, diag.level);
        assert_eq!(deserialized.component, diag.component);
    }

    #[test]
    fn test_diagnostic_clone() {
        let original = Diagnostic::error(ComponentType::Database, "DB error".to_string())
            .with_details("Connection timeout".to_string());

        let cloned = original.clone();

        assert_eq!(original.id, cloned.id);
        assert_eq!(original.message, cloned.message);
        assert_eq!(original.level, cloned.level);
        assert_eq!(original.details, cloned.details);
    }

    #[test]
    fn test_diagnostic_all_components() {
        let components = vec![
            ComponentType::Cpu,
            ComponentType::Memory,
            ComponentType::Storage,
            ComponentType::Network,
            ComponentType::System,
            ComponentType::Application,
            ComponentType::Database,
            ComponentType::Cache,
        ];

        for component in components {
            let diag = Diagnostic::info(component, format!("{} test", component));
            assert_eq!(diag.component, component);
        }
    }

    #[test]
    fn test_diagnostic_all_levels() {
        // Test info level
        let info = Diagnostic::info(ComponentType::System, "Test message".to_string());
        assert_eq!(info.level, DiagnosticLevel::Info);

        // Test warning level
        let warning = Diagnostic::warning(ComponentType::System, "Test message".to_string());
        assert_eq!(warning.level, DiagnosticLevel::Warning);

        // Test error level
        let error = Diagnostic::error(ComponentType::System, "Test message".to_string());
        assert_eq!(error.level, DiagnosticLevel::Error);

        // Test critical level
        let critical = Diagnostic::critical(ComponentType::System, "Test message".to_string());
        assert_eq!(critical.level, DiagnosticLevel::Critical);
    }

    #[test]
    fn test_diagnostic_timestamp_is_recent() {
        use std::time::SystemTime;

        let before = SystemTime::now();
        let diag = Diagnostic::info(ComponentType::System, "Timestamp test".to_string());
        let after = SystemTime::now();

        // Timestamp should be between before and after
        assert!(diag.timestamp >= before);
        assert!(diag.timestamp <= after);
    }

    #[test]
    fn test_diagnostic_resolve_timestamp() {
        let mut diag = Diagnostic::warning(ComponentType::Network, "Test".to_string());

        thread::sleep(Duration::from_millis(10));
        let resolve_time_before = std::time::SystemTime::now();

        diag.resolve();

        let resolve_time_after = std::time::SystemTime::now();

        let resolved_at = diag.resolved_at.unwrap();
        assert!(resolved_at >= resolve_time_before);
        assert!(resolved_at <= resolve_time_after);
    }

    #[test]
    fn test_diagnostic_debug_format() {
        let diag = Diagnostic::critical(ComponentType::Storage, "Critical issue".to_string());
        let debug_str = format!("{:?}", diag);

        assert!(debug_str.contains("Diagnostic"));
        assert!(debug_str.contains("Critical issue"));
    }
}
