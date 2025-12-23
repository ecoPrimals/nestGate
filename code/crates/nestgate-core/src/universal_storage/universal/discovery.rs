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
use crate::Result;

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
}
