/// **RETURN BUILDERS - TESTS MODULE**
/// Contains unit tests for all builder functions.
/// Extracted from the large return_builders.rs to achieve file size compliance.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::NestGateError;
    use serde_json::Value as JsonValue;

    #[test]
    fn test_success_response_builder() {
        // ✅ CATCHES STATUS FIELD MUTATIONS
        let response = crate::return_builders::build_success_response(
            "test_request_123".to_string(),
            JsonValue::String("test_data".to_string()),
        );

        assert_eq!(response.request_id, "test_request_123");
        assert!(matches!(
            response.status,
            crate::traits::UniversalResponseStatus::Success
        ));
        assert!(response.data.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_error_response_builder() {
        // ✅ CATCHES ERROR FIELD MUTATIONS
        let response = crate::return_builders::build_error_response(
            "test_request_456".to_string(),
            500,
            "Internal server error".to_string(),
        );

        assert_eq!(response.request_id, "test_request_456");
        assert!(matches!(
            response.status,
            crate::traits::UniversalResponseStatus::Error
        ));
        assert!(response.data.is_none());
        assert!(response.error.is_some());
        assert_eq!(
            response.error.unwrap_or_else(|| {
                tracing::error!("Error field is missing from response");
                return Err(NestGateError::InternalError(format!(
                    "Response error field should be present in test"
                )));
            }),
            "Internal server error"
        );
    }

    #[test]
    fn test_json_response_builder() {
        // ✅ CATCHES JSON FIELD MUTATIONS (success structure)
        let success_response = crate::return_builders::build_json_response(
            true,
            Some("Operation successful".to_string()),
            None,
        );

        assert_eq!(success_response["success"], JsonValue::Bool(true));
        assert_eq!(
            success_response["message"],
            JsonValue::String("Operation successful".to_string())
        );
        assert!(success_response.get("error").is_none());
        assert!(success_response.get("timestamp").is_some());
    }

    #[test]
    fn test_mock_resource_allocation_builder() {
        // ✅ CATCHES MOCK DATA FIELD MUTATIONS
        let allocation =
            crate::return_builders::build_mock_resource_allocation(4, 16384, 1000, 1000);

        assert_eq!(allocation.cpu_cores, 4);
        assert_eq!(allocation.memory_mb, 16384); // 16GB in MB
        assert_eq!(allocation.disk_gb, 1000);
        assert_eq!(allocation.network_bandwidth_mbps, 1000);
        assert!(allocation.id.len() > 0);
        assert!(allocation.allocated_at > std::time::SystemTime::UNIX_EPOCH);
    }

    #[test]
    fn test_access_grant_builder() {
        // ✅ CATCHES ACCESS GRANT FIELD MUTATIONS
        let permissions = vec!["read".to_string(), "write".to_string()];
        let consensus_nodes = vec!["node1".to_string(), "node2".to_string()];
        let grant = crate::return_builders::build_access_grant(
            &permissions, // Zero-copy: pass by reference instead of cloning
            1234567890,
            "test_proof_data",
            &consensus_nodes, // Zero-copy: pass by reference instead of cloning
            0.85,
        );

        assert_eq!(grant.permissions, permissions);
        assert_eq!(grant.valid_until, 1234567890);
        assert_eq!(grant.consensus_nodes, consensus_nodes);
        assert_eq!(grant.consensus_percentage, 0.85);
        assert!(grant.proof_hash.len() > 0);
    }
}
