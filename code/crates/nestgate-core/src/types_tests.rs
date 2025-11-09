//! Unit tests for core types
//!
//! These tests cover core enums and type functionality.

#[cfg(test)]
mod tests {
    use super::super::types::*;

    // ==================== AllocationStatus Tests ====================

    #[test]
    fn test_allocation_status_variants() {
        let _active = AllocationStatus::Active;
        let _inactive = AllocationStatus::Inactive;
        let _pending = AllocationStatus::Pending;
        let _failed = AllocationStatus::Failed;
        // All variants should compile
    }

    #[test]
    fn test_allocation_status_equality() {
        assert_eq!(AllocationStatus::Active, AllocationStatus::Active);
        assert_ne!(AllocationStatus::Active, AllocationStatus::Inactive);
    }

    #[test]
    fn test_allocation_status_clone() {
        let status1 = AllocationStatus::Active;
        let status2 = status1.clone();
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_allocation_status_serialization() {
        let status = AllocationStatus::Pending;
        let json = serde_json::to_string(&status).expect("Failed to serialize");
        assert!(!json.is_empty());
    }

    #[test]
    fn test_allocation_status_deserialization() {
        let status = AllocationStatus::Failed;
        let json = serde_json::to_string(&status).unwrap();
        let deserialized: AllocationStatus = serde_json::from_str(&json)
            .expect("Failed to deserialize");
        assert_eq!(status, deserialized);
    }

    // ==================== StorageTier Tests ====================

    #[test]
    fn test_storage_tier_display() {
        assert_eq!(format!("{}", StorageTier::Hot), "Hot");
        assert_eq!(format!("{}", StorageTier::Warm), "Warm");
        assert_eq!(format!("{}", StorageTier::Cold), "Cold");
        assert_eq!(format!("{}", StorageTier::Cache), "Cache");
        assert_eq!(format!("{}", StorageTier::Archive), "Archive");
    }

    #[test]
    fn test_storage_tier_all() {
        let all_tiers = StorageTier::all();
        assert_eq!(all_tiers.len(), 5);
        assert!(all_tiers.contains(&StorageTier::Hot));
        assert!(all_tiers.contains(&StorageTier::Archive));
    }

    #[test]
    fn test_storage_tier_priority() {
        assert_eq!(StorageTier::Hot.priority(), 0);
        assert_eq!(StorageTier::Warm.priority(), 1);
        assert_eq!(StorageTier::Cold.priority(), 2);
        assert_eq!(StorageTier::Cache.priority(), 3);
        assert_eq!(StorageTier::Archive.priority(), 4);
    }

    #[test]
    fn test_storage_tier_priority_order() {
        assert!(StorageTier::Hot.priority() < StorageTier::Warm.priority());
        assert!(StorageTier::Warm.priority() < StorageTier::Cold.priority());
    }

    #[test]
    fn test_storage_tier_as_str() {
        assert_eq!(StorageTier::Hot.as_str(), "hot");
        assert_eq!(StorageTier::Warm.as_str(), "warm");
        assert_eq!(StorageTier::Cold.as_str(), "cold");
        assert_eq!(StorageTier::Cache.as_str(), "cache");
        assert_eq!(StorageTier::Archive.as_str(), "archive");
    }

    #[test]
    fn test_storage_tier_default() {
        let default_tier = StorageTier::default();
        assert_eq!(default_tier, StorageTier::Hot);
    }

    #[test]
    fn test_storage_tier_clone() {
        let tier1 = StorageTier::Warm;
        let tier2 = tier1.clone();
        assert_eq!(tier1, tier2);
    }

    #[test]
    fn test_storage_tier_serialization() {
        let tier = StorageTier::Cold;
        let json = serde_json::to_string(&tier).expect("Failed to serialize");
        assert!(!json.is_empty());
    }

    // ==================== HealthStatus Tests ====================

    #[test]
    fn test_health_status_display() {
        assert_eq!(format!("{}", HealthStatus::Healthy), "Healthy");
        assert_eq!(format!("{}", HealthStatus::Degraded), "Degraded");
        assert_eq!(format!("{}", HealthStatus::Unhealthy), "Unhealthy");
        assert_eq!(format!("{}", HealthStatus::Unknown), "Unknown");
    }

    #[test]
    fn test_health_status_default() {
        let default_status = HealthStatus::default();
        assert_eq!(default_status, HealthStatus::Unknown);
    }

    #[test]
    fn test_health_status_equality() {
        assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Degraded);
    }

    #[test]
    fn test_health_status_clone() {
        let status1 = HealthStatus::Degraded;
        let status2 = status1.clone();
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_health_status_serialization() {
        let status = HealthStatus::Unhealthy;
        let json = serde_json::to_string(&status).expect("Failed to serialize");
        let deserialized: HealthStatus = serde_json::from_str(&json)
            .expect("Failed to deserialize");
        assert_eq!(status, deserialized);
    }

    // ==================== ServiceState Tests ====================

    #[test]
    fn test_service_state_display() {
        assert_eq!(format!("{}", ServiceState::Running), "Running");
        assert_eq!(format!("{}", ServiceState::Stopped), "Stopped");
        assert_eq!(format!("{}", ServiceState::Starting), "Starting");
        assert_eq!(format!("{}", ServiceState::Stopping), "Stopping");
        assert_eq!(format!("{}", ServiceState::Error), "Error");
        assert_eq!(format!("{}", ServiceState::Unknown), "Unknown");
    }

    #[test]
    fn test_service_state_default() {
        let default_state = ServiceState::default();
        assert_eq!(default_state, ServiceState::Unknown);
    }

    #[test]
    fn test_service_state_transitions() {
        let state = ServiceState::Starting;
        assert_ne!(state, ServiceState::Running);
        
        let new_state = ServiceState::Running;
        assert_eq!(new_state, ServiceState::Running);
    }

    #[test]
    fn test_service_state_serialization() {
        let state = ServiceState::Stopping;
        let json = serde_json::to_string(&state).expect("Failed to serialize");
        let deserialized: ServiceState = serde_json::from_str(&json)
            .expect("Failed to deserialize");
        assert_eq!(state, deserialized);
    }

    // ==================== PerformanceTier Tests ====================

    #[test]
    fn test_performance_tier_display() {
        assert_eq!(format!("{}", PerformanceTier::Ultra), "Ultra");
        assert_eq!(format!("{}", PerformanceTier::High), "High");
        assert_eq!(format!("{}", PerformanceTier::Standard), "Standard");
        assert_eq!(format!("{}", PerformanceTier::Economy), "Economy");
    }

    #[test]
    fn test_performance_tier_default() {
        let default_tier = PerformanceTier::default();
        assert_eq!(default_tier, PerformanceTier::Standard);
    }

    #[test]
    fn test_performance_tier_clone() {
        let tier1 = PerformanceTier::Ultra;
        let tier2 = tier1.clone();
        assert_eq!(tier1, tier2);
    }

    #[test]
    fn test_performance_tier_serialization() {
        let tier = PerformanceTier::High;
        let json = serde_json::to_string(&tier).expect("Failed to serialize");
        let deserialized: PerformanceTier = serde_json::from_str(&json)
            .expect("Failed to deserialize");
        assert_eq!(tier, deserialized);
    }

    // ==================== Integration Tests ====================

    #[test]
    fn test_storage_tier_workflow() {
        let hot_tier = StorageTier::Hot;
        assert_eq!(hot_tier.priority(), 0);
        assert_eq!(hot_tier.as_str(), "hot");
        assert_eq!(format!("{}", hot_tier), "Hot");
    }

    #[test]
    fn test_health_status_workflow() {
        let mut status = HealthStatus::default();
        assert_eq!(status, HealthStatus::Unknown);
        
        status = HealthStatus::Healthy;
        assert_ne!(status, HealthStatus::Unknown);
    }

    #[test]
    fn test_service_state_workflow() {
        let states = vec![
            ServiceState::Stopped,
            ServiceState::Starting,
            ServiceState::Running,
        ];
        
        assert_eq!(states.len(), 3);
        assert_eq!(states[2], ServiceState::Running);
    }

    // ==================== Debug Format Tests ====================

    #[test]
    fn test_allocation_status_debug() {
        let status = AllocationStatus::Active;
        let debug_str = format!("{:?}", status);
        assert!(debug_str.contains("Active"));
    }

    #[test]
    fn test_storage_tier_debug() {
        let tier = StorageTier::Warm;
        let debug_str = format!("{:?}", tier);
        assert!(debug_str.contains("Warm"));
    }

    #[test]
    fn test_health_status_debug() {
        let status = HealthStatus::Healthy;
        let debug_str = format!("{:?}", status);
        assert!(debug_str.contains("Healthy"));
    }

    #[test]
    fn test_service_state_debug() {
        let state = ServiceState::Running;
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("Running"));
    }

    // ==================== Hash Tests ====================

    #[test]
    fn test_allocation_status_hash() {
        use std::collections::HashSet;
        
        let mut set = HashSet::new();
        set.insert(AllocationStatus::Active);
        set.insert(AllocationStatus::Active);
        
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_storage_tier_hash() {
        use std::collections::HashSet;
        
        let mut set = HashSet::new();
        set.insert(StorageTier::Hot);
        set.insert(StorageTier::Warm);
        set.insert(StorageTier::Hot);
        
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_service_state_hash() {
        use std::collections::HashSet;
        
        let mut set = HashSet::new();
        set.insert(ServiceState::Running);
        set.insert(ServiceState::Stopped);
        
        assert!(set.contains(&ServiceState::Running));
        assert!(set.contains(&ServiceState::Stopped));
    }
}

