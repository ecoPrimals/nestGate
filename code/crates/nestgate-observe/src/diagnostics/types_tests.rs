// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for diagnostics types module
//! Added: November 14, 2025 - Real Production Tests

#[cfg(test)]
mod diagnostics_types_tests {
    use crate::diagnostics::types::*;
    use serde_json;

    // DiagnosticLevel tests
    #[test]
    fn test_diagnostic_level_display() {
        assert_eq!(DiagnosticLevel::Info.to_string(), "INFO");
        assert_eq!(DiagnosticLevel::Warning.to_string(), "WARNING");
        assert_eq!(DiagnosticLevel::Error.to_string(), "ERROR");
        assert_eq!(DiagnosticLevel::Critical.to_string(), "CRITICAL");
    }

    #[test]
    fn test_diagnostic_level_equality() {
        assert_eq!(DiagnosticLevel::Info, DiagnosticLevel::Info);
        assert_ne!(DiagnosticLevel::Info, DiagnosticLevel::Warning);
        assert_ne!(DiagnosticLevel::Error, DiagnosticLevel::Critical);
    }

    #[test]
    fn test_diagnostic_level_serialization() {
        let level = DiagnosticLevel::Warning;
        let json = serde_json::to_string(&level).unwrap();
        assert!(json.contains("Warning"));

        let deserialized: DiagnosticLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, level);
    }

    #[test]
    fn test_diagnostic_level_clone() {
        let level = DiagnosticLevel::Critical;
        let cloned = level;
        assert_eq!(level, cloned);
    }

    // ComponentType tests
    #[test]
    fn test_component_type_display() {
        assert_eq!(ComponentType::Cpu.to_string(), "CPU");
        assert_eq!(ComponentType::Memory.to_string(), "Memory");
        assert_eq!(ComponentType::Storage.to_string(), "Storage");
        assert_eq!(ComponentType::Network.to_string(), "Network");
        assert_eq!(ComponentType::System.to_string(), "System");
        assert_eq!(ComponentType::Application.to_string(), "Application");
        assert_eq!(ComponentType::Database.to_string(), "Database");
        assert_eq!(ComponentType::Cache.to_string(), "Cache");
    }

    #[test]
    fn test_component_type_equality() {
        assert_eq!(ComponentType::Cpu, ComponentType::Cpu);
        assert_ne!(ComponentType::Cpu, ComponentType::Memory);
        assert_ne!(ComponentType::Network, ComponentType::Storage);
    }

    #[test]
    fn test_component_type_hash() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(ComponentType::Cpu, "CPU data");
        map.insert(ComponentType::Memory, "Memory data");

        assert_eq!(map.get(&ComponentType::Cpu), Some(&"CPU data"));
        assert_eq!(map.get(&ComponentType::Memory), Some(&"Memory data"));
    }

    #[test]
    fn test_component_type_serialization() {
        let component = ComponentType::Network;
        let json = serde_json::to_string(&component).unwrap();
        assert!(json.contains("Network"));

        let deserialized: ComponentType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, component);
    }

    // ServiceInfo tests
    #[test]
    fn test_service_info_default() {
        let info = ServiceInfo::default();

        assert_eq!(info.name, "unknown");
        assert_eq!(info.version, "0.0.0");
        assert_eq!(info.status, "unknown");
        assert!(info.start_time.is_none());
        assert!(info.pid.is_none());
        assert!(info.memory_bytes.is_none());
        assert!(info.description.is_none());
        assert!(info.dependencies.is_none());
        assert!(info.cpu_percent.is_none());
        assert!(info.command_line.is_none());
    }

    #[test]
    fn test_service_info_creation() {
        use std::time::SystemTime;

        let info = ServiceInfo {
            name: "test-service".to_string(),
            version: "1.0.0".to_string(),
            status: "running".to_string(),
            start_time: Some(SystemTime::now()),
            pid: Some(12345),
            memory_bytes: Some(1024 * 1024 * 100), // 100 MB
            description: Some("Test service".to_string()),
            dependencies: Some(vec!["dep1".to_string(), "dep2".to_string()]),
            cpu_percent: Some(5.5),
            command_line: Some("/usr/bin/test".to_string()),
        };

        assert_eq!(info.name, "test-service");
        assert_eq!(info.version, "1.0.0");
        assert_eq!(info.status, "running");
        assert!(info.start_time.is_some());
        assert_eq!(info.pid, Some(12345));
        assert_eq!(info.memory_bytes, Some(104857600));
        assert_eq!(info.description, Some("Test service".to_string()));
        assert_eq!(info.dependencies.as_ref().unwrap().len(), 2);
        assert_eq!(info.cpu_percent, Some(5.5));
        assert_eq!(info.command_line, Some("/usr/bin/test".to_string()));
    }

    #[test]
    fn test_service_info_serialization() {
        let info = ServiceInfo {
            name: "serialize-test".to_string(),
            version: "2.0.0".to_string(),
            status: "stopped".to_string(),
            start_time: None,
            pid: Some(9999),
            memory_bytes: Some(2048),
            description: Some("Serialization test".to_string()),
            dependencies: None,
            cpu_percent: Some(1.5),
            command_line: None,
        };

        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("serialize-test"));
        assert!(json.contains("2.0.0"));
        assert!(json.contains("9999"));

        let deserialized: ServiceInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, info.name);
        assert_eq!(deserialized.version, info.version);
        assert_eq!(deserialized.pid, info.pid);
    }

    #[test]
    fn test_service_info_clone() {
        let info = ServiceInfo::default();
        let cloned = info.clone();

        assert_eq!(info.name, cloned.name);
        assert_eq!(info.version, cloned.version);
        assert_eq!(info.status, cloned.status);
    }

    #[test]
    fn test_service_info_with_dependencies() {
        let info = ServiceInfo {
            name: "dependent-service".to_string(),
            version: "1.0.0".to_string(),
            status: "running".to_string(),
            start_time: None,
            pid: None,
            memory_bytes: None,
            description: None,
            dependencies: Some(vec![
                "service-a".to_string(),
                "service-b".to_string(),
                "service-c".to_string(),
            ]),
            cpu_percent: None,
            command_line: None,
        };

        let deps = info.dependencies.as_ref().unwrap();
        assert_eq!(deps.len(), 3);
        assert!(deps.contains(&"service-a".to_string()));
        assert!(deps.contains(&"service-b".to_string()));
        assert!(deps.contains(&"service-c".to_string()));
    }

    #[test]
    fn test_service_info_debug_format() {
        let info = ServiceInfo::default();
        let debug_str = format!("{:?}", info);

        assert!(debug_str.contains("ServiceInfo"));
        assert!(debug_str.contains("unknown"));
    }

    #[test]
    fn test_component_type_all_variants() {
        let components = [
            ComponentType::Cpu,
            ComponentType::Memory,
            ComponentType::Storage,
            ComponentType::Network,
            ComponentType::System,
            ComponentType::Application,
            ComponentType::Database,
            ComponentType::Cache,
        ];

        assert_eq!(components.len(), 8);

        // Verify all are unique
        use std::collections::HashSet;
        let set: HashSet<_> = components.iter().collect();
        assert_eq!(set.len(), 8);
    }

    #[test]
    fn test_diagnostic_level_all_variants() {
        let levels = [
            DiagnosticLevel::Info,
            DiagnosticLevel::Warning,
            DiagnosticLevel::Error,
            DiagnosticLevel::Critical,
        ];

        assert_eq!(levels.len(), 4);

        // Verify all are unique
        for (i, level1) in levels.iter().enumerate() {
            for (j, level2) in levels.iter().enumerate() {
                if i == j {
                    assert_eq!(level1, level2);
                } else {
                    assert_ne!(level1, level2);
                }
            }
        }
    }
}
