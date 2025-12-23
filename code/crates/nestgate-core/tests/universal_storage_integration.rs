//! Integration tests for Universal Agnostic Storage
//!
//! Tests the complete flow: discovery → protocol detection → adapter → operations

use nestgate_core::universal_storage::{
    AuthenticationPattern, DiscoveredProtocol, StorageFeature, StorageOperationPattern,
    TransportProtocol, UniversalStorageAdapter, UniversalStorageDiscovery,
};
use std::env;

#[tokio::test]
async fn test_local_filesystem_discovery() {
    // Test discovery of local filesystem storage
    let local = UniversalStorageDiscovery::discover_local()
        .await
        .expect("Should discover local storage");

    assert_eq!(local.len(), 1);
    assert_eq!(local[0].name, "local");
    assert!(local[0].endpoint.contains("storage"));
}

#[tokio::test]
async fn test_endpoint_probe_https() {
    // Test probing HTTPS endpoint
    let discovered =
        UniversalStorageDiscovery::probe_endpoint("test", "https://storage.example.com/bucket")
            .await;

    assert!(discovered.is_some());
    let storage = discovered.unwrap();

    // Should detect HTTPS transport
    match &storage.protocol.transport {
        TransportProtocol::Http { version: _, tls } => {
            assert!(tls.is_some(), "Should detect TLS for HTTPS");
        }
        _ => panic!("Should detect HTTP transport"),
    }

    // Should detect object store pattern
    match &storage.protocol.operation_pattern {
        StorageOperationPattern::ObjectStore { .. } => {
            // Expected
        }
        _ => panic!("Should detect object store pattern"),
    }
}

#[tokio::test]
async fn test_endpoint_probe_http() {
    // Test probing HTTP endpoint (no TLS)
    let discovered =
        UniversalStorageDiscovery::probe_endpoint("test", "http://localhost:9000/bucket").await;

    assert!(discovered.is_some());
    let storage = discovered.unwrap();

    // Should detect HTTP without TLS
    match &storage.protocol.transport {
        TransportProtocol::Http { tls, .. } => {
            assert!(tls.is_none(), "Should not have TLS for HTTP");
        }
        _ => panic!("Should detect HTTP transport"),
    }
}

#[tokio::test]
async fn test_auth_detection_access_key() {
    // Set environment variables for access key authentication
    env::set_var("STORAGE_TEST_ACCESS_KEY", "test_access_key");
    env::set_var("STORAGE_TEST_SECRET_KEY", "test_secret_key");

    let discovered =
        UniversalStorageDiscovery::probe_endpoint("test", "https://storage.example.com/bucket")
            .await
            .expect("Should discover endpoint");

    // Should detect signed headers authentication
    match &discovered.protocol.authentication {
        AuthenticationPattern::SignedHeaders { key_id, .. } => {
            assert_eq!(key_id, "test_access_key");
        }
        _ => panic!("Should detect signed headers authentication"),
    }

    // Cleanup
    env::remove_var("STORAGE_TEST_ACCESS_KEY");
    env::remove_var("STORAGE_TEST_SECRET_KEY");
}

#[tokio::test]
async fn test_auth_detection_bearer_token() {
    // Set environment variable for bearer token
    env::set_var("STORAGE_TOKEN_TOKEN", "test_bearer_token");

    let discovered =
        UniversalStorageDiscovery::probe_endpoint("token", "https://api.example.com/storage")
            .await
            .expect("Should discover endpoint");

    // Should detect bearer token authentication
    match &discovered.protocol.authentication {
        AuthenticationPattern::BearerToken { token, .. } => {
            assert_eq!(token.expose_secret(), "test_bearer_token");
        }
        _ => panic!("Should detect bearer token authentication"),
    }

    // Cleanup
    env::remove_var("STORAGE_TOKEN_TOKEN");
}

#[tokio::test]
async fn test_auth_detection_api_key() {
    // Set environment variable for API key
    env::set_var("STORAGE_API_API_KEY", "test_api_key");

    let discovered = UniversalStorageDiscovery::probe_endpoint("api", "https://api.example.com/v1")
        .await
        .expect("Should discover endpoint");

    // Should detect API key authentication
    match &discovered.protocol.authentication {
        AuthenticationPattern::ApiKey { key, .. } => {
            assert_eq!(key.expose_secret(), "test_api_key");
        }
        _ => panic!("Should detect API key authentication"),
    }

    // Cleanup
    env::remove_var("STORAGE_API_API_KEY");
}

#[tokio::test]
async fn test_filesystem_adapter_operations() {
    use tempfile::TempDir;

    // Create temporary directory for testing
    let temp_dir = TempDir::new().expect("Should create temp dir");
    let temp_path = temp_dir.path().to_str().unwrap();

    // Create protocol for filesystem

    let protocol = DiscoveredProtocol::new(
        TransportProtocol::UnixSocket {
            path: temp_path.to_string(),
        },
        StorageOperationPattern::FileSystem {
            path_separator: '/',
            case_sensitive: true,
        },
        AuthenticationPattern::None,
    );

    // Create adapter
    let adapter = UniversalStorageAdapter::new(temp_path, protocol);

    // Test write
    let test_data = b"Hello, Universal Storage!";
    adapter
        .write("test.txt", test_data)
        .await
        .expect("Should write file");

    // Test read
    let read_data = adapter.read("test.txt").await.expect("Should read file");
    assert_eq!(read_data, test_data);

    // Test list
    let keys = adapter.list("").await.expect("Should list files");
    assert!(!keys.is_empty());

    // Test delete
    adapter
        .delete("test.txt")
        .await
        .expect("Should delete file");

    // Verify deletion
    let result = adapter.read("test.txt").await;
    assert!(result.is_err(), "File should be deleted");
}

#[tokio::test]
async fn test_multiple_storage_discovery() {
    // Set up multiple storage endpoints
    env::set_var("STORAGE_BACKUP_ENDPOINT", "https://s3.example.com/backup");
    env::set_var("STORAGE_CACHE_ENDPOINT", "http://localhost:9000/cache");
    env::set_var(
        "STORAGE_ARCHIVE_ENDPOINT",
        "https://archive.example.com/data",
    );

    let discovered = UniversalStorageDiscovery::discover_from_env()
        .await
        .expect("Should discover storage");

    // Should discover at least the ones we configured
    assert!(discovered.len() >= 3);

    // Verify names were extracted correctly
    let names: Vec<_> = discovered.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"backup"));
    assert!(names.contains(&"cache"));
    assert!(names.contains(&"archive"));

    // Cleanup
    env::remove_var("STORAGE_BACKUP_ENDPOINT");
    env::remove_var("STORAGE_CACHE_ENDPOINT");
    env::remove_var("STORAGE_ARCHIVE_ENDPOINT");
}

#[tokio::test]
async fn test_protocol_description() {
    let discovered =
        UniversalStorageDiscovery::probe_endpoint("test", "https://storage.example.com/bucket")
            .await
            .expect("Should discover endpoint");

    let description = discovered.description();

    // Description should include key information
    assert!(description.contains("test"));
    assert!(description.contains("storage.example.com"));
    assert!(description.contains("HTTP"));
}

#[tokio::test]
async fn test_feature_discovery() {
    let discovered =
        UniversalStorageDiscovery::probe_endpoint("test", "https://storage.example.com/bucket")
            .await
            .expect("Should discover endpoint");

    // Should discover basic features
    assert!(discovered.protocol.has_feature(&StorageFeature::Read));
    assert!(discovered.protocol.has_feature(&StorageFeature::Write));
    assert!(discovered.protocol.has_feature(&StorageFeature::Delete));
    assert!(discovered.protocol.has_feature(&StorageFeature::List));
}

#[tokio::test]
async fn test_security_detection() {
    // HTTPS endpoint should have secure transport
    let https_storage =
        UniversalStorageDiscovery::probe_endpoint("secure", "https://storage.example.com/bucket")
            .await
            .expect("Should discover endpoint");

    // Transport should be secure (has TLS)
    assert!(
        https_storage.protocol.transport.is_secure(),
        "HTTPS transport should be secure"
    );

    // HTTP endpoint should not have secure transport
    let http_storage =
        UniversalStorageDiscovery::probe_endpoint("insecure", "http://storage.example.com/bucket")
            .await
            .expect("Should discover endpoint");

    // Transport should not be secure (no TLS)
    assert!(
        !http_storage.protocol.transport.is_secure(),
        "HTTP transport should not be secure"
    );
}

#[test]
fn test_protocol_serialization() {
    use nestgate_core::universal_storage::universal::{
        HttpVersion, ObjectAddressing, ObjectOrganization,
    };

    // Create a protocol
    let protocol = DiscoveredProtocol::new(
        TransportProtocol::Http {
            version: HttpVersion::Http1_1,
            tls: None,
        },
        StorageOperationPattern::ObjectStore {
            addressing: ObjectAddressing::PathBased,
            organization: ObjectOrganization::Hierarchical { separator: '/' },
        },
        AuthenticationPattern::None,
    );

    // Should be serializable
    let json = serde_json::to_string(&protocol).expect("Should serialize");
    assert!(!json.is_empty());

    // Should be deserializable
    let _deserialized: DiscoveredProtocol =
        serde_json::from_str(&json).expect("Should deserialize");
}
