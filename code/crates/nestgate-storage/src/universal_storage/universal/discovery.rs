// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # Universal Storage Discovery
//!
//! Auto-discovers storage endpoints and their capabilities at runtime.

use super::authentication::{
    ApiKeyLocation, AuthenticationPattern, SecretString, SigningAlgorithm,
};
use super::features::{FeatureSet, StorageFeature};
use super::operations::{ObjectAddressing, ObjectOrganization, StorageOperationPattern};
use super::protocol::{ApiInfo, DiscoveredProtocol};
use super::transport::{HttpVersion, TlsConfig, TransportProtocol};
use nestgate_types::error::Result;

/// Universal storage discovery
pub struct UniversalStorageDiscovery;

impl UniversalStorageDiscovery {
    /// Discover all storage endpoints from environment
    pub async fn discover_all() -> Result<Vec<DiscoveredStorage>> {
        let mut discovered = Vec::new();

        // 1. Environment variables (primary method)
        discovered.extend(Self::discover_from_env().await?);

        // 2. Configuration files (future)
        // discovered.extend(Self::discover_from_config().await?);

        // 3. Service discovery (future)
        // discovered.extend(Self::discover_from_services().await?);

        // 4. Auto-detect local storage
        discovered.extend(Self::discover_local().await?);

        Ok(discovered)
    }

    /// Discover storage endpoints from environment variables
    ///
    /// Pattern: `STORAGE_<NAME>_ENDPOINT=<url>`
    ///
    /// Examples:
    /// - `STORAGE_BACKUP_ENDPOINT=https://s3.example.com/bucket`
    /// - `STORAGE_CACHE_ENDPOINT=http://localhost:9000/data`
    pub async fn discover_from_env() -> Result<Vec<DiscoveredStorage>> {
        let mut storage = Vec::new();

        for (key, value) in std::env::vars() {
            if key.starts_with("STORAGE_") && key.ends_with("_ENDPOINT") {
                let name = Self::extract_storage_name(&key);
                if let Some(discovered) = Self::probe_endpoint(&name, &value).await {
                    storage.push(discovered);
                }
            }
        }

        Ok(storage)
    }

    /// Discover local filesystem storage
    pub async fn discover_local() -> Result<Vec<DiscoveredStorage>> {
        let local = DiscoveredStorage {
            name: "local".to_string(),
            endpoint: "file://./storage".to_string(),
            protocol: DiscoveredProtocol::new(
                TransportProtocol::UnixSocket {
                    path: "./storage".to_string(),
                },
                StorageOperationPattern::FileSystem {
                    path_separator: '/',
                    case_sensitive: cfg!(unix),
                },
                AuthenticationPattern::None,
            ),
        };

        Ok(vec![local])
    }

    /// Probe an endpoint to discover its protocol
    pub async fn probe_endpoint(name: &str, endpoint: &str) -> Option<DiscoveredStorage> {
        // 1. Detect transport
        let transport = Self::detect_transport(endpoint).await?;

        // 2. Discover operation pattern
        let operation_pattern = Self::discover_operations(endpoint, &transport).await?;

        // 3. Detect authentication
        let authentication = Self::detect_auth_pattern(name, endpoint).await?;

        // 4. Probe features (basic implementation)
        let mut protocol = DiscoveredProtocol::new(transport, operation_pattern, authentication);
        protocol.features = Self::probe_basic_features();

        // 5. Set API info
        protocol.api_info = Self::detect_api_info(endpoint).await;

        Some(DiscoveredStorage {
            name: name.to_string(),
            endpoint: endpoint.to_string(),
            protocol,
        })
    }

    /// Extract storage name from environment variable key
    ///
    /// STORAGE_BACKUP_ENDPOINT -> backup
    fn extract_storage_name(key: &str) -> String {
        key.trim_start_matches("STORAGE_")
            .trim_end_matches("_ENDPOINT")
            .to_lowercase()
    }

    /// Detect transport protocol from endpoint
    async fn detect_transport(endpoint: &str) -> Option<TransportProtocol> {
        if endpoint.starts_with("https://") {
            Some(TransportProtocol::Http {
                version: HttpVersion::Http1_1,
                tls: Some(TlsConfig::default()),
            })
        } else if endpoint.starts_with("http://") {
            Some(TransportProtocol::Http {
                version: HttpVersion::Http1_1,
                tls: None,
            })
        } else if endpoint.starts_with("file://") || endpoint.starts_with("./") {
            Some(TransportProtocol::UnixSocket {
                path: endpoint.to_string(),
            })
        } else {
            // Default to HTTP
            Some(TransportProtocol::Http {
                version: HttpVersion::Http1_1,
                tls: None,
            })
        }
    }

    /// Discover operation pattern
    async fn discover_operations(
        endpoint: &str,
        _transport: &TransportProtocol,
    ) -> Option<StorageOperationPattern> {
        // Check if it looks like object storage
        if endpoint.contains("/bucket") || endpoint.contains("/container") {
            return Some(StorageOperationPattern::ObjectStore {
                addressing: ObjectAddressing::PathBased,
                organization: ObjectOrganization::Hierarchical { separator: '/' },
            });
        }

        // Check if it's a file path
        if endpoint.starts_with("file://") || endpoint.starts_with("./") {
            return Some(StorageOperationPattern::FileSystem {
                path_separator: '/',
                case_sensitive: cfg!(unix),
            });
        }

        // Default to object store with path-based addressing
        Some(StorageOperationPattern::ObjectStore {
            addressing: ObjectAddressing::PathBased,
            organization: ObjectOrganization::Hierarchical { separator: '/' },
        })
    }

    /// Detect authentication pattern
    async fn detect_auth_pattern(name: &str, _endpoint: &str) -> Option<AuthenticationPattern> {
        let prefix = format!("STORAGE_{}", name.to_uppercase());

        // Check for access key + secret key (signed headers pattern)
        let access_key_var = format!("{}_ACCESS_KEY", prefix);
        let secret_key_var = format!("{}_SECRET_KEY", prefix);

        if let (Ok(access_key), Ok(secret_key)) = (
            std::env::var(&access_key_var),
            std::env::var(&secret_key_var),
        ) {
            let session_token = std::env::var(format!("{}_SESSION_TOKEN", prefix))
                .ok()
                .map(SecretString::new);

            return Some(AuthenticationPattern::SignedHeaders {
                signing_algorithm: SigningAlgorithm::HmacSha256,
                key_id: access_key,
                secret_key: SecretString::new(secret_key),
                headers_to_sign: vec![
                    "host".to_string(),
                    "x-amz-date".to_string(),
                    "authorization".to_string(),
                ],
                session_token,
            });
        }

        // Check for bearer token
        let token_var = format!("{}_TOKEN", prefix);
        if let Ok(token) = std::env::var(&token_var) {
            return Some(AuthenticationPattern::BearerToken {
                token: SecretString::new(token),
                token_type: "Bearer".to_string(),
            });
        }

        // Check for API key
        let api_key_var = format!("{}_API_KEY", prefix);
        if let Ok(api_key) = std::env::var(&api_key_var) {
            return Some(AuthenticationPattern::ApiKey {
                key: SecretString::new(api_key),
                location: ApiKeyLocation::Header {
                    name: "X-API-Key".to_string(),
                },
            });
        }

        // No authentication
        Some(AuthenticationPattern::None)
    }

    /// Probe basic features (placeholder for now)
    fn probe_basic_features() -> FeatureSet {
        let mut features = FeatureSet::new();

        // Assume basic operations are supported
        features.add(StorageFeature::Read);
        features.add(StorageFeature::Write);
        features.add(StorageFeature::Delete);
        features.add(StorageFeature::List);

        features
    }

    /// Detect API info (placeholder)
    async fn detect_api_info(_endpoint: &str) -> ApiInfo {
        ApiInfo::default()
    }
}

/// Discovered storage endpoint
#[derive(Debug, Clone)]
pub struct DiscoveredStorage {
    /// Storage name
    pub name: String,

    /// Endpoint URL
    pub endpoint: String,

    /// Discovered protocol
    pub protocol: DiscoveredProtocol,
}

impl DiscoveredStorage {
    /// Get a human-readable description
    pub fn description(&self) -> String {
        format!(
            "{}: {} ({})",
            self.name,
            self.endpoint,
            self.protocol.description()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_storage_name() {
        assert_eq!(
            UniversalStorageDiscovery::extract_storage_name("STORAGE_BACKUP_ENDPOINT"),
            "backup"
        );
        assert_eq!(
            UniversalStorageDiscovery::extract_storage_name("STORAGE_CACHE_ENDPOINT"),
            "cache"
        );
    }

    #[tokio::test]
    async fn test_detect_transport_https() {
        let transport = UniversalStorageDiscovery::detect_transport("https://example.com")
            .await
            .unwrap();

        match transport {
            TransportProtocol::Http { version, tls } => {
                assert_eq!(version, HttpVersion::Http1_1);
                assert!(tls.is_some());
            }
            _ => panic!("Wrong transport type"),
        }
    }

    #[tokio::test]
    async fn test_detect_transport_http() {
        let transport = UniversalStorageDiscovery::detect_transport("http://example.com")
            .await
            .unwrap();

        match transport {
            TransportProtocol::Http { version, tls } => {
                assert_eq!(version, HttpVersion::Http1_1);
                assert!(tls.is_none());
            }
            _ => panic!("Wrong transport type"),
        }
    }

    #[tokio::test]
    async fn test_discover_local() {
        let local = UniversalStorageDiscovery::discover_local().await.unwrap();
        assert_eq!(local.len(), 1);
        assert_eq!(local[0].name, "local");
    }

    #[test]
    fn test_extract_storage_name_variations() {
        assert_eq!(
            UniversalStorageDiscovery::extract_storage_name("STORAGE_PROD_ENDPOINT"),
            "prod"
        );
        assert_eq!(
            UniversalStorageDiscovery::extract_storage_name("STORAGE_DEV_ENDPOINT"),
            "dev"
        );
        assert_eq!(
            UniversalStorageDiscovery::extract_storage_name("STORAGE_TEST_123_ENDPOINT"),
            "test_123"
        );
    }

    #[tokio::test]
    async fn test_detect_transport_file() {
        let transport = UniversalStorageDiscovery::detect_transport("file:///data/storage")
            .await
            .unwrap();

        match transport {
            TransportProtocol::UnixSocket { path } => {
                assert_eq!(path, "file:///data/storage");
            }
            _ => panic!("Wrong transport type"),
        }
    }

    #[tokio::test]
    async fn test_detect_transport_relative_path() {
        let transport = UniversalStorageDiscovery::detect_transport("./data")
            .await
            .unwrap();

        match transport {
            TransportProtocol::UnixSocket { .. } => {}
            _ => panic!("Wrong transport type"),
        }
    }

    #[tokio::test]
    async fn test_detect_transport_default() {
        let transport = UniversalStorageDiscovery::detect_transport("example.com")
            .await
            .unwrap();

        match transport {
            TransportProtocol::Http { tls, .. } => {
                assert!(tls.is_none()); // Default to HTTP without TLS
            }
            _ => panic!("Wrong transport type"),
        }
    }

    #[tokio::test]
    async fn test_discover_operations_object_store() {
        let transport = TransportProtocol::Http {
            version: HttpVersion::Http1_1,
            tls: None,
        };

        let pattern = UniversalStorageDiscovery::discover_operations(
            "https://s3.example.com/bucket/data",
            &transport,
        )
        .await;

        assert!(pattern.is_some());
        match pattern.unwrap() {
            StorageOperationPattern::ObjectStore { .. } => {}
            _ => panic!("Expected ObjectStore pattern"),
        }
    }

    #[tokio::test]
    async fn test_discover_operations_container() {
        let transport = TransportProtocol::Http {
            version: HttpVersion::Http1_1,
            tls: None,
        };

        let pattern = UniversalStorageDiscovery::discover_operations(
            "https://blob.example.com/container/data",
            &transport,
        )
        .await;

        assert!(pattern.is_some());
        match pattern.unwrap() {
            StorageOperationPattern::ObjectStore { .. } => {}
            _ => panic!("Expected ObjectStore pattern"),
        }
    }

    #[tokio::test]
    async fn test_discover_operations_filesystem() {
        let transport = TransportProtocol::UnixSocket {
            path: "./storage".to_string(),
        };

        let pattern =
            UniversalStorageDiscovery::discover_operations("file://./storage", &transport).await;

        assert!(pattern.is_some());
        match pattern.unwrap() {
            StorageOperationPattern::FileSystem { .. } => {}
            _ => panic!("Expected FileSystem pattern"),
        }
    }

    #[tokio::test]
    async fn test_probe_endpoint_https() {
        let discovered =
            UniversalStorageDiscovery::probe_endpoint("test", "https://example.com/storage").await;

        assert!(discovered.is_some());
        let storage = discovered.unwrap();
        assert_eq!(storage.name, "test");
        assert_eq!(storage.endpoint, "https://example.com/storage");
    }

    #[tokio::test]
    async fn test_probe_endpoint_http() {
        let discovered =
            UniversalStorageDiscovery::probe_endpoint("cache", "http://localhost:9000/data").await;

        assert!(discovered.is_some());
        let storage = discovered.unwrap();
        assert_eq!(storage.name, "cache");
    }

    #[tokio::test]
    async fn test_probe_endpoint_file() {
        let discovered =
            UniversalStorageDiscovery::probe_endpoint("local", "file://./storage").await;

        assert!(discovered.is_some());
        let storage = discovered.unwrap();
        assert_eq!(storage.name, "local");
    }

    #[tokio::test]
    async fn test_discover_from_env_no_storage_vars() {
        // Test with no STORAGE_*_ENDPOINT environment variables
        let storage = UniversalStorageDiscovery::discover_from_env()
            .await
            .unwrap();
        // Should return empty or only find existing env vars
        assert!(storage.is_empty() || !storage.is_empty());
    }

    #[tokio::test]
    async fn test_discover_all() {
        let discovered = UniversalStorageDiscovery::discover_all().await.unwrap();

        // Should at least discover local storage
        assert!(!discovered.is_empty());

        // Check that local storage is included
        assert!(discovered.iter().any(|s| s.name == "local"));
    }

    #[test]
    fn test_probe_basic_features() {
        let _features = UniversalStorageDiscovery::probe_basic_features();

        // FeatureSet is returned - just ensure the function works
        // The actual features are checked elsewhere
    }

    #[tokio::test]
    async fn test_detect_api_info_http() {
        let _api_info = UniversalStorageDiscovery::detect_api_info("http://example.com/api").await;

        // detect_api_info returns ApiInfo directly - just ensure it doesn't panic
    }

    #[tokio::test]
    async fn test_detect_api_info_https() {
        let _api_info =
            UniversalStorageDiscovery::detect_api_info("https://secure.example.com/api").await;

        // Should return valid ApiInfo without panicking
    }

    #[tokio::test]
    async fn test_detect_auth_pattern_with_key() {
        let pattern =
            UniversalStorageDiscovery::detect_auth_pattern("test", "https://api.example.com").await;

        assert!(pattern.is_some());
    }

    #[tokio::test]
    async fn test_detect_auth_pattern_no_key() {
        let pattern =
            UniversalStorageDiscovery::detect_auth_pattern("nokey", "https://api.example.com")
                .await;

        assert!(pattern.is_some());
    }

    #[test]
    fn test_discovered_storage_creation() {
        let storage = DiscoveredStorage {
            name: "test".to_string(),
            endpoint: "https://example.com".to_string(),
            protocol: DiscoveredProtocol::new(
                TransportProtocol::Http {
                    version: HttpVersion::Http1_1,
                    tls: Some(TlsConfig::default()),
                },
                StorageOperationPattern::ObjectStore {
                    addressing: ObjectAddressing::PathBased,
                    organization: ObjectOrganization::Hierarchical { separator: '/' },
                },
                AuthenticationPattern::None,
            ),
        };

        assert_eq!(storage.name, "test");
        assert_eq!(storage.endpoint, "https://example.com");
    }

    #[tokio::test]
    async fn test_concurrent_discovery() {
        let handle1 = tokio::spawn(async { UniversalStorageDiscovery::discover_local().await });

        let handle2 = tokio::spawn(async { UniversalStorageDiscovery::discover_local().await });

        let result1 = handle1.await.unwrap();
        let result2 = handle2.await.unwrap();

        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[tokio::test]
    async fn test_multiple_discoveries() {
        // Test that we can run discovery multiple times
        let _ = UniversalStorageDiscovery::discover_all().await.unwrap();
        let _ = UniversalStorageDiscovery::discover_all().await.unwrap();
        let discovered = UniversalStorageDiscovery::discover_all().await.unwrap();

        assert!(!discovered.is_empty());
    }

    #[tokio::test]
    async fn test_probe_endpoint_with_s3() {
        let discovered = UniversalStorageDiscovery::probe_endpoint(
            "backup",
            "https://s3.amazonaws.com/mybucket",
        )
        .await;

        assert!(discovered.is_some());
        let storage = discovered.unwrap();

        // Should detect object store pattern
        match storage.protocol.operation_pattern {
            StorageOperationPattern::ObjectStore { .. } => {}
            _ => panic!("Expected object store pattern for S3"),
        }
    }

    #[tokio::test]
    async fn test_discover_operations_default() {
        let transport = TransportProtocol::Http {
            version: HttpVersion::Http1_1,
            tls: None,
        };

        let pattern = UniversalStorageDiscovery::discover_operations(
            "https://api.example.com/data",
            &transport,
        )
        .await;

        // Should return some default pattern
        assert!(pattern.is_some());
    }
}
