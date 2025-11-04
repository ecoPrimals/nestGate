//! **COMPREHENSIVE HEALTH HANDLER TESTS**
//!
//! Test coverage for health check handlers - critical for monitoring and operations.

#[cfg(test)]
mod tests {
    use super::super::health::*;

    #[test]
    fn test_health_check_basic() {
        let health = HealthCheck {
            healthy: true,
            checks: vec![],
        };
        
        assert!(health.healthy);
        assert_eq!(health.checks.len(), 0);
    }

    #[test]
    fn test_health_check_with_services() {
        let health = HealthCheck {
            healthy: true,
            checks: vec![
                ServiceHealth {
                    name: "database".to_string(),
                    healthy: true,
                    message: None,
                },
                ServiceHealth {
                    name: "cache".to_string(),
                    healthy: true,
                    message: None,
                },
            ],
        };
        
        assert!(health.healthy);
        assert_eq!(health.checks.len(), 2);
        assert!(health.checks.iter().all(|c| c.healthy));
    }

    #[test]
    fn test_service_health_unhealthy() {
        let service = ServiceHealth {
            name: "database".to_string(),
            healthy: false,
            message: Some("Connection timeout".to_string()),
        };
        
        assert!(!service.healthy);
        assert_eq!(service.name, "database");
        assert_eq!(service.message, Some("Connection timeout".to_string()));
    }

    #[test]
    fn test_health_check_overall_health() {
        let health = HealthCheck {
            healthy: false,
            checks: vec![
                ServiceHealth {
                    name: "api".to_string(),
                    healthy: true,
                    message: None,
                },
                ServiceHealth {
                    name: "database".to_string(),
                    healthy: false,
                    message: Some("Down".to_string()),
                },
            ],
        };
        
        // Overall health should be false if any service is unhealthy
        assert!(!health.healthy);
        assert_eq!(health.checks.len(), 2);
        assert!(health.checks.iter().any(|c| !c.healthy));
    }

    #[test]
    fn test_health_check_serialization() {
        let health = HealthCheck {
            healthy: true,
            checks: vec![
                ServiceHealth {
                    name: "test".to_string(),
                    healthy: true,
                    message: None,
                },
            ],
        };
        
        let json = serde_json::to_string(&health);
        assert!(json.is_ok(), "HealthCheck should serialize");
        
        let serialized = json.expect("Test setup failed");
        assert!(serialized.contains("\"healthy\":true"));
        assert!(serialized.contains("\"test\""));
    }

    #[test]
    fn test_health_check_deserialization() {
        let json = r#"{
            "healthy": true,
            "checks": [{
                "name": "service1",
                "healthy": true,
                "message": null
            }]
        }"#;
        
        let health: std::result::Result<HealthCheck, _> = serde_json::from_str(json);
        assert!(health.is_ok(), "HealthCheck should deserialize");
        
        let health = health.expect("Test setup failed");
        assert!(health.healthy);
        assert_eq!(health.checks.len(), 1);
        assert_eq!(health.checks[0].name, "service1");
    }

    #[test]
    fn test_service_health_with_message() {
        let service = ServiceHealth {
            name: "cache".to_string(),
            healthy: true,
            message: Some("Connected to Redis".to_string()),
        };
        
        assert!(service.healthy);
        assert!(service.message.is_some());
        assert_eq!(service.message.as_ref().expect("Test setup failed"), "Connected to Redis");
    }

    #[test]
    fn test_health_check_empty_checks() {
        let health = HealthCheck {
            healthy: true,
            checks: vec![],
        };
        
        assert!(health.healthy);
        assert!(health.checks.is_empty());
    }

    #[test]
    fn test_health_check_many_services() {
        let mut checks = vec![];
        for i in 0..10 {
            checks.push(ServiceHealth {
                name: format!("service{}", i),
                healthy: i % 2 == 0, // Even services healthy
                message: None,
            });
        }
        
        let health = HealthCheck {
            healthy: false, // At least one service is unhealthy
            checks,
        };
        
        assert!(!health.healthy);
        assert_eq!(health.checks.len(), 10);
        assert_eq!(health.checks.iter().filter(|c| c.healthy).count(), 5);
        assert_eq!(health.checks.iter().filter(|c| !c.healthy).count(), 5);
    }

    #[test]
    fn test_service_health_name_not_empty() {
        let service = ServiceHealth {
            name: "db".to_string(),
            healthy: true,
            message: None,
        };
        
        assert!(!service.name.is_empty(), "Service name should not be empty");
        assert_eq!(service.name.len(), 2);
    }

    #[test]
    fn test_health_check_json_structure() {
        let health = HealthCheck {
            healthy: true,
            checks: vec![
                ServiceHealth {
                    name: "api".to_string(),
                    healthy: true,
                    message: Some("OK".to_string()),
                },
            ],
        };
        
        let json = serde_json::to_value(&health).expect("Test setup failed");
        
        assert!(json["healthy"].as_bool().expect("Test setup failed"));
        assert!(json["checks"].is_array());
        assert_eq!(json["checks"].as_array().expect("Test setup failed").len(), 1);
    }
}

