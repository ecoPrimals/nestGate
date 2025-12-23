//! Comprehensive tests for graceful degradation module
//!
//! Tests all degradation levels, fallback strategies, and state transitions.

#[cfg(test)]
mod tests {
    use super::super::{DegradationLevel, FallbackStrategy, GracefulDegradation};

    #[test]
    fn test_new_degradation_manager() {
        let manager = GracefulDegradation::new();
        assert_eq!(manager.level(), DegradationLevel::Normal);
    }

    #[test]
    fn test_default_degradation_manager() {
        let manager = GracefulDegradation::default();
        assert_eq!(manager.level(), DegradationLevel::Normal);
    }

    #[test]
    fn test_set_degradation_level() {
        let mut manager = GracefulDegradation::new();

        manager.set_level(DegradationLevel::Minor);
        assert_eq!(manager.level(), DegradationLevel::Minor);

        manager.set_level(DegradationLevel::Major);
        assert_eq!(manager.level(), DegradationLevel::Major);

        manager.set_level(DegradationLevel::Critical);
        assert_eq!(manager.level(), DegradationLevel::Critical);

        manager.set_level(DegradationLevel::Emergency);
        assert_eq!(manager.level(), DegradationLevel::Emergency);
    }

    #[test]
    fn test_degradation_level_equality() {
        assert_eq!(DegradationLevel::Normal, DegradationLevel::Normal);
        assert_ne!(DegradationLevel::Normal, DegradationLevel::Minor);
        assert_ne!(DegradationLevel::Minor, DegradationLevel::Major);
        assert_ne!(DegradationLevel::Major, DegradationLevel::Critical);
        assert_ne!(DegradationLevel::Critical, DegradationLevel::Emergency);
    }

    #[test]
    fn test_add_cache_fallback_strategy() {
        let mut manager = GracefulDegradation::new();
        manager.add_strategy("storage".to_string(), FallbackStrategy::Cache);

        // Strategy added successfully
        let result = manager.handle_failure("storage");
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_default_fallback_strategy() {
        let mut manager = GracefulDegradation::new();
        manager.add_strategy("compute".to_string(), FallbackStrategy::Default);

        let result = manager.handle_failure("compute");
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_disable_fallback_strategy() {
        let mut manager = GracefulDegradation::new();
        manager.add_strategy("analytics".to_string(), FallbackStrategy::Disable);

        let result = manager.handle_failure("analytics");
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_alternative_fallback_strategy() {
        let mut manager = GracefulDegradation::new();
        manager.add_strategy(
            "api".to_string(),
            FallbackStrategy::Alternative {
                endpoint: "https://backup.example.com".to_string(),
            },
        );

        let result = manager.handle_failure("api");
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_failure_without_strategy() {
        let mut manager = GracefulDegradation::new();

        // No strategy registered for this capability
        let result = manager.handle_failure("unknown_capability");
        assert!(result.is_ok()); // Should still succeed, just log warning
    }

    #[test]
    fn test_multiple_strategies() {
        let mut manager = GracefulDegradation::new();

        manager.add_strategy("storage".to_string(), FallbackStrategy::Cache);
        manager.add_strategy("compute".to_string(), FallbackStrategy::Default);
        manager.add_strategy("analytics".to_string(), FallbackStrategy::Disable);

        assert!(manager.handle_failure("storage").is_ok());
        assert!(manager.handle_failure("compute").is_ok());
        assert!(manager.handle_failure("analytics").is_ok());
    }

    #[test]
    fn test_strategy_override() {
        let mut manager = GracefulDegradation::new();

        // Add initial strategy
        manager.add_strategy("api".to_string(), FallbackStrategy::Cache);
        assert!(manager.handle_failure("api").is_ok());

        // Override with new strategy
        manager.add_strategy(
            "api".to_string(),
            FallbackStrategy::Alternative {
                endpoint: "https://backup.example.com".to_string(),
            },
        );
        assert!(manager.handle_failure("api").is_ok());
    }

    #[test]
    fn test_degradation_progression() {
        let mut manager = GracefulDegradation::new();

        // Normal operation
        assert_eq!(manager.level(), DegradationLevel::Normal);

        // Minor degradation
        manager.set_level(DegradationLevel::Minor);
        assert_eq!(manager.level(), DegradationLevel::Minor);

        // Major degradation
        manager.set_level(DegradationLevel::Major);
        assert_eq!(manager.level(), DegradationLevel::Major);

        // Critical
        manager.set_level(DegradationLevel::Critical);
        assert_eq!(manager.level(), DegradationLevel::Critical);

        // Emergency
        manager.set_level(DegradationLevel::Emergency);
        assert_eq!(manager.level(), DegradationLevel::Emergency);

        // Recovery back to normal
        manager.set_level(DegradationLevel::Normal);
        assert_eq!(manager.level(), DegradationLevel::Normal);
    }

    #[test]
    fn test_same_level_set_twice() {
        let mut manager = GracefulDegradation::new();

        manager.set_level(DegradationLevel::Minor);
        assert_eq!(manager.level(), DegradationLevel::Minor);

        // Setting same level again (should log but work)
        manager.set_level(DegradationLevel::Minor);
        assert_eq!(manager.level(), DegradationLevel::Minor);
    }

    #[test]
    fn test_fallback_strategy_cloning() {
        let strategy1 = FallbackStrategy::Cache;
        let strategy2 = strategy1.clone();

        let mut manager = GracefulDegradation::new();
        manager.add_strategy("test1".to_string(), strategy1);
        manager.add_strategy("test2".to_string(), strategy2);

        assert!(manager.handle_failure("test1").is_ok());
        assert!(manager.handle_failure("test2").is_ok());
    }

    #[test]
    fn test_degradation_with_multiple_capabilities() {
        let mut manager = GracefulDegradation::new();
        manager.set_level(DegradationLevel::Major);

        // Add strategies for multiple capabilities
        manager.add_strategy("storage".to_string(), FallbackStrategy::Cache);
        manager.add_strategy("compute".to_string(), FallbackStrategy::Disable);
        manager.add_strategy(
            "api".to_string(),
            FallbackStrategy::Alternative {
                endpoint: "https://backup.api.example.com".to_string(),
            },
        );

        // All should handle failures gracefully
        assert!(manager.handle_failure("storage").is_ok());
        assert!(manager.handle_failure("compute").is_ok());
        assert!(manager.handle_failure("api").is_ok());
        assert!(manager.handle_failure("unknown").is_ok());
    }

    #[test]
    fn test_empty_capability_name() {
        let mut manager = GracefulDegradation::new();
        manager.add_strategy("".to_string(), FallbackStrategy::Cache);

        let result = manager.handle_failure("");
        assert!(result.is_ok());
    }

    #[test]
    fn test_long_capability_name() {
        let mut manager = GracefulDegradation::new();
        let long_name = "a".repeat(1000);
        manager.add_strategy(long_name.clone(), FallbackStrategy::Default);

        let result = manager.handle_failure(&long_name);
        assert!(result.is_ok());
    }

    #[test]
    fn test_special_characters_in_capability_name() {
        let mut manager = GracefulDegradation::new();
        let special_name = "capability-with_special.chars@123!";
        manager.add_strategy(special_name.to_string(), FallbackStrategy::Cache);

        let result = manager.handle_failure(special_name);
        assert!(result.is_ok());
    }

    #[test]
    fn test_degradation_level_debug_format() {
        assert!(format!("{:?}", DegradationLevel::Normal).contains("Normal"));
        assert!(format!("{:?}", DegradationLevel::Minor).contains("Minor"));
        assert!(format!("{:?}", DegradationLevel::Major).contains("Major"));
        assert!(format!("{:?}", DegradationLevel::Critical).contains("Critical"));
        assert!(format!("{:?}", DegradationLevel::Emergency).contains("Emergency"));
    }

    #[test]
    fn test_fallback_strategy_debug_format() {
        assert!(format!("{:?}", FallbackStrategy::Cache).contains("Cache"));
        assert!(format!("{:?}", FallbackStrategy::Default).contains("Default"));
        assert!(format!("{:?}", FallbackStrategy::Disable).contains("Disable"));

        let alt = FallbackStrategy::Alternative {
            endpoint: "https://example.com".to_string(),
        };
        assert!(format!("{:?}", alt).contains("Alternative"));
        assert!(format!("{:?}", alt).contains("example.com"));
    }

    #[test]
    fn test_graceful_degradation_state_machine() {
        let mut manager = GracefulDegradation::new();

        // Test state transitions like a state machine
        let transitions = vec![
            DegradationLevel::Normal,
            DegradationLevel::Minor,
            DegradationLevel::Major,
            DegradationLevel::Critical,
            DegradationLevel::Emergency,
            DegradationLevel::Critical, // Recovery
            DegradationLevel::Major,    // Recovery
            DegradationLevel::Minor,    // Recovery
            DegradationLevel::Normal,   // Full recovery
        ];

        for level in transitions {
            manager.set_level(level);
            assert_eq!(manager.level(), level);
        }
    }
}
