// **EVENTS SYSTEM TESTS**
//
// Comprehensive tests for event types, bus, routing, and pubsub functionality

#[cfg(test)]
mod canonical_event_tests {
    use crate::canonical_types::events::{Event, EventCategory, EventSeverity};
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::time::SystemTime;

    // ==================== EVENT CREATION TESTS ====================

    #[test]
    fn test_event_default() {
        let event = Event::default();
        assert!(!event.id.is_empty());
        assert_eq!(event.category, EventCategory::System);
        assert_eq!(event.severity, EventSeverity::Info);
        assert_eq!(event.message, "Default event");
        assert!(event.data.is_empty());
        assert!(event.tags.is_empty());
    }

    #[test]
    fn test_event_custom_creation() {
        let mut data = HashMap::new();
        data.insert("key1".to_string(), serde_json::json!("value1"));

        let event = Event {
            id: "custom-123".to_string(),
            timestamp: SystemTime::now(),
            category: EventCategory::Security,
            severity: EventSeverity::Warning,
            message: "Security warning".to_string(),
            source: "test-source".to_string(),
            data,
            tags: vec!["security".to_string(), "warning".to_string()],
        };

        assert_eq!(event.id, "custom-123");
        assert_eq!(event.category, EventCategory::Security);
        assert_eq!(event.severity, EventSeverity::Warning);
        assert_eq!(event.message, "Security warning");
        assert_eq!(event.source, "test-source");
        assert_eq!(event.data.len(), 1);
        assert_eq!(event.tags.len(), 2);
    }

    #[test]
    fn test_event_clone() {
        let event1 = Event::default();
        let event2 = event1.clone();
        assert_eq!(event1.id, event2.id);
        assert_eq!(event1.message, event2.message);
    }

    // ==================== EVENT SEVERITY TESTS ====================

    #[test]
    fn test_severity_ordering() {
        assert!(EventSeverity::Debug < EventSeverity::Info);
        assert!(EventSeverity::Info < EventSeverity::Warning);
        assert!(EventSeverity::Warning < EventSeverity::Error);
        assert!(EventSeverity::Error < EventSeverity::Critical);
    }

    #[test]
    fn test_severity_equality() {
        assert_eq!(EventSeverity::Info, EventSeverity::Info);
        assert_ne!(EventSeverity::Info, EventSeverity::Warning);
    }

    #[test]
    fn test_severity_clone() {
        let severity1 = EventSeverity::Critical;
        let severity2 = severity1.clone();
        assert_eq!(severity1, severity2);
    }

    // ==================== EVENT CATEGORY TESTS ====================

    #[test]
    fn test_category_variants() {
        let categories = [
            EventCategory::System,
            EventCategory::Security,
            EventCategory::Network,
            EventCategory::Storage,
            EventCategory::User,
            EventCategory::Application,
            EventCategory::Performance,
            EventCategory::Custom("TestCategory".to_string()),
        ];

        assert_eq!(categories.len(), 8);
    }

    #[test]
    fn test_category_equality() {
        assert_eq!(EventCategory::System, EventCategory::System);
        assert_ne!(EventCategory::System, EventCategory::Security);
    }

    #[test]
    fn test_category_custom() {
        let custom1 = EventCategory::Custom("Custom1".to_string());
        let custom2 = EventCategory::Custom("Custom1".to_string());
        let custom3 = EventCategory::Custom("Custom2".to_string());

        assert_eq!(custom1, custom2);
        assert_ne!(custom1, custom3);
    }

    #[test]
    fn test_category_clone() {
        let category1 = EventCategory::Network;
        let category2 = category1.clone();
        assert_eq!(category1, category2);
    }

    // ==================== EVENT DATA TESTS ====================

    #[test]
    fn test_event_data_manipulation() {
        let mut event = Event::default();

        event
            .data
            .insert("count".to_string(), serde_json::json!(42));
        event
            .data
            .insert("status".to_string(), serde_json::json!("active"));

        assert_eq!(event.data.len(), 2);
        assert_eq!(event.data.get("count"), Some(&serde_json::json!(42)));
    }

    #[test]
    fn test_event_complex_data() {
        let mut event = Event::default();

        let complex_data = serde_json::json!({
            "nested": {
                "key": "value",
                "array": [1, 2, 3]
            }
        });

        event
            .data
            .insert("complex".to_string(), complex_data.clone());
        assert_eq!(event.data.get("complex"), Some(&complex_data));
    }

    // ==================== EVENT TAG TESTS ====================

    #[test]
    fn test_event_tags_manipulation() {
        let mut event = Event::default();

        event.tags.push("tag1".to_string());
        event.tags.push("tag2".to_string());
        event.tags.push("tag3".to_string());

        assert_eq!(event.tags.len(), 3);
        assert!(event.tags.contains(&"tag1".to_string()));
    }

    #[test]
    fn test_event_tags_filtering() {
        let mut event = Event::default();
        event.tags = vec![
            "production".to_string(),
            "urgent".to_string(),
            "api".to_string(),
        ];

        let urgent_tags: Vec<_> = event.tags.iter().filter(|t| t.contains("urgent")).collect();

        assert_eq!(urgent_tags.len(), 1);
    }

    // ==================== EVENT SERIALIZATION TESTS ====================

    #[test]
    fn test_event_serialization() {
        let event = Event::default();
        let serialized = serde_json::to_string(&event);
        assert!(serialized.is_ok());
    }

    #[test]
    fn test_event_deserialization() {
        let json = r#"{
            "id": "test-123",
            "timestamp": {"secs_since_epoch": 1234567890, "nanos_since_epoch": 0},
            "category": "System",
            "severity": "Info",
            "message": "Test message",
            "source": "test-source",
            "data": {},
            "tags": []
        }"#;

        let result: Result<Event, _> = serde_json::from_str(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_severity_serialization() {
        let severity = EventSeverity::Critical;
        let serialized = serde_json::to_string(&severity).unwrap();
        assert!(serialized.contains("Critical"));
    }

    #[test]
    fn test_category_serialization() {
        let category = EventCategory::Security;
        let serialized = serde_json::to_string(&category).unwrap();
        assert!(serialized.contains("Security"));
    }

    // ==================== EVENT ARC SHARING TESTS ====================

    #[test]
    fn test_event_arc_sharing() {
        let event = Arc::new(Event::default());
        let event_ref1 = Arc::clone(&event);
        let event_ref2 = Arc::clone(&event);

        assert_eq!(event.id, event_ref1.id);
        assert_eq!(event_ref1.id, event_ref2.id);
        assert_eq!(Arc::strong_count(&event), 3);
    }

    #[test]
    fn test_event_thread_safety() {
        let event = Arc::new(Event::default());
        let event_clone = Arc::clone(&event);

        let handle = std::thread::spawn(move || event_clone.id.clone());

        let id = handle.join().unwrap();
        assert_eq!(id, event.id);
    }
}

#[cfg(test)]
mod service_stub_tests {
    use super::super::bus;
    use super::super::config;
    use super::super::pubsub;
    use super::super::routing;
    use crate::traits_root::service::Service;

    // ==================== BUS SERVICE TESTS ====================

    #[test]
    fn test_bus_config_default() {
        let config = bus::Config::default();
        assert!(config.enabled);
        assert!(config.max_connections > 0);
        assert!(config.buffer_size > 0);
    }

    #[test]
    fn test_bus_service_creation() {
        let config = bus::Config::default();
        let service = bus::DefaultService::new(config);
        assert!(format!("{:?}", service).contains("DefaultService"));
    }

    #[tokio::test]
    async fn test_bus_service_lifecycle() {
        let service = bus::create_service();
        let init_result = service.initialize().await;
        assert!(init_result.is_ok());

        let health = service.health_check().await;
        assert!(health.is_ok());

        let shutdown_result = service.shutdown().await;
        assert!(shutdown_result.is_ok());
    }

    #[tokio::test]
    async fn test_bus_validate_config() {
        let valid_config = bus::Config::default();
        assert!(bus::validate_config(&valid_config).await.is_ok());

        let invalid_config = bus::Config {
            enabled: true,
            timeout: std::time::Duration::from_secs(1),
            max_connections: 0,
            buffer_size: 1024,
        };
        assert!(bus::validate_config(&invalid_config).await.is_err());
    }

    #[tokio::test]
    async fn test_bus_metrics() {
        let service = bus::create_service();
        let metrics = service.get_metrics().await;
        assert_eq!(metrics.requests_processed, 0);
        assert_eq!(metrics.errors_encountered, 0);
    }

    // ==================== CONFIG SERVICE TESTS ====================

    #[test]
    fn test_config_service_default() {
        let config = config::Config::default();
        assert!(config.enabled);
        assert!(config.max_connections > 0);
    }

    #[tokio::test]
    async fn test_config_service_lifecycle() {
        let service = config::create_service();
        assert!(service.initialize().await.is_ok());
        assert!(service.health_check().await.is_ok());
        assert!(service.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_config_validate_buffer_size() {
        let invalid_config = config::Config {
            enabled: true,
            timeout: std::time::Duration::from_secs(1),
            max_connections: 100,
            buffer_size: 0,
        };
        assert!(config::validate_config(&invalid_config).await.is_err());
    }

    // ==================== PUBSUB SERVICE TESTS ====================

    #[test]
    fn test_pubsub_config_default() {
        let config = pubsub::Config::default();
        assert!(config.enabled);
        assert!(config.timeout > std::time::Duration::from_millis(0));
    }

    #[tokio::test]
    async fn test_pubsub_service_lifecycle() {
        let service = pubsub::create_service();
        assert!(service.initialize().await.is_ok());
        assert!(service.health_check().await.is_ok());
        assert!(service.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_pubsub_health_status() {
        let service = pubsub::create_service();
        let health = service.health_check().await.unwrap();
        // health_check() now returns bool, not HealthStatus
        assert!(health);
    }

    #[tokio::test]
    async fn test_pubsub_metrics_initial() {
        let service = pubsub::create_service();
        let metrics = service.get_metrics().await;
        assert_eq!(metrics.requests_processed, 0);
        assert_eq!(metrics.memory_usage_bytes, 0);
    }

    // ==================== ROUTING SERVICE TESTS ====================

    #[test]
    fn test_routing_config_default() {
        let config = routing::Config::default();
        assert!(config.enabled);
    }

    #[tokio::test]
    async fn test_routing_service_creation() {
        let service = routing::create_service();
        let metrics = service.get_metrics().await;
        assert_eq!(metrics.errors_encountered, 0);
    }

    #[tokio::test]
    async fn test_routing_service_lifecycle() {
        let service = routing::create_service();
        assert!(service.initialize().await.is_ok());
        assert!(service.health_check().await.is_ok());
        assert!(service.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_routing_validate_config() {
        let valid_config = routing::Config::default();
        assert!(routing::validate_config(&valid_config).await.is_ok());
    }

    // ==================== CROSS-SERVICE TESTS ====================

    #[test]
    fn test_all_services_have_consistent_defaults() {
        let bus_config = bus::Config::default();
        let config_config = config::Config::default();
        let pubsub_config = pubsub::Config::default();
        let routing_config = routing::Config::default();

        assert!(bus_config.enabled);
        assert!(config_config.enabled);
        assert!(pubsub_config.enabled);
        assert!(routing_config.enabled);
    }

    #[tokio::test]
    async fn test_all_services_initialize_successfully() {
        let bus_service = bus::create_service();
        let config_service = config::create_service();
        let pubsub_service = pubsub::create_service();
        let routing_service = routing::create_service();

        assert!(bus_service.initialize().await.is_ok());
        assert!(config_service.initialize().await.is_ok());
        assert!(pubsub_service.initialize().await.is_ok());
        assert!(routing_service.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_all_services_healthy_by_default() {
        let services = vec![
            bus::create_service().health_check().await,
            config::create_service().health_check().await,
            pubsub::create_service().health_check().await,
            routing::create_service().health_check().await,
        ];

        for health in services {
            assert!(health.is_ok());
        }
    }
}
