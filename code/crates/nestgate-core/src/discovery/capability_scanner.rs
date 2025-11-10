//! **CAPABILITY SCANNER**
//!
//! Environment capability detection implementation for the Infant Discovery Architecture.

use crate::error::NestGateError;
use std::collections::HashMap;
use std::env;
use std::future::Future;
use tracing::{debug, info, warn};

/// Information about a discovered capability
#[derive(Debug, Clone)]
pub struct CapabilityInfo {
    /// Type of capability (orchestration, security, ai, etc.)
    pub capability_type: String,
    /// Endpoint URL for the capability
    pub endpoint: String,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
    /// Additional metadata about the capability
    pub metadata: HashMap<String, String>,
}

/// Trait for capability discovery methods
///
/// **NATIVE ASYNC**: Uses `impl Future` for zero-cost abstractions (no boxing overhead)
pub trait DiscoveryMethod: Send + Sync {
    /// Discover capabilities using this method - native async, no boxing
    fn discover(&self) -> impl Future<Output = Result<Vec<CapabilityInfo>, NestGateError>> + Send;

    /// Get the name of this discovery method
    fn method_name(&self) -> &str;
}

/// Environment variable discovery method
#[derive(Debug)]
pub struct EnvironmentDiscovery {
    /// Known capability patterns to scan for
    capability_patterns: Vec<String>,
}

impl EnvironmentDiscovery {
    /// Create a new environment discovery scanner
    #[must_use]
    pub fn new() -> Self {
        Self {
            capability_patterns: vec![
                "ORCHESTRATION_DISCOVERY_ENDPOINT".to_string(),
                "SECURITY_DISCOVERY_ENDPOINT".to_string(),
                "AI_DISCOVERY_ENDPOINT".to_string(),
                "STORAGE_DISCOVERY_ENDPOINT".to_string(),
                "MONITORING_DISCOVERY_ENDPOINT".to_string(),
                "COMPUTE_DISCOVERY_ENDPOINT".to_string(),
                "NETWORK_DISCOVERY_ENDPOINT".to_string(),
            ],
        }
    }

    /// Add a custom capability pattern to scan for
    pub fn add_pattern(&mut self, pattern: String) {
        self.capability_patterns.push(pattern);
    }
}

impl Default for EnvironmentDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

impl DiscoveryMethod for EnvironmentDiscovery {
    fn discover(&self) -> impl Future<Output = Result<Vec<CapabilityInfo>, NestGateError>> + Send {
        async move {
            let mut capabilities = Vec::new();

            debug!("Scanning environment variables for capabilities");

            for pattern in &self.capability_patterns {
                if let Ok(endpoint) = env::var(pattern) {
                    let capability_type = pattern
                        .strip_suffix("_DISCOVERY_ENDPOINT")
                        .unwrap_or(pattern)
                        .to_lowercase();

                    info!("Found {} capability at: {}", capability_type, endpoint);

                    let mut metadata = HashMap::new();
                    metadata.insert("source".to_string(), "environment".to_string());
                    metadata.insert("pattern".to_string(), pattern.clone());

                    // Check for additional metadata environment variables
                    let auth_key = format!("{}_AUTH_KEY", capability_type.to_uppercase());
                    if let Ok(auth) = env::var(&auth_key) {
                        metadata.insert("auth_key".to_string(), auth);
                    }

                    let timeout_key = format!("{}_TIMEOUT_MS", capability_type.to_uppercase());
                    if let Ok(timeout) = env::var(&timeout_key) {
                        metadata.insert("timeout_ms".to_string(), timeout);
                    }

                    capabilities.push(CapabilityInfo {
                        capability_type,
                        endpoint,
                        confidence: 0.95, // High confidence for explicit env vars
                        metadata,
                    });
                }
            }

            if capabilities.is_empty() {
                warn!("No capabilities found in environment variables");
            } else {
                info!(
                    "Found {} capabilities via environment discovery",
                    capabilities.len()
                );
            }

            Ok(capabilities)
        }
    }

    fn method_name(&self) -> &str {
        "environment"
    }
}

/// **DISCOVERY METHOD ENUM**
///
/// Enum dispatch for discovery methods - zero-cost alternative to `Box<dyn DiscoveryMethod>`.
/// This enables native async while maintaining runtime polymorphism through enum dispatch.
#[derive(Debug)]
pub enum DiscoveryMethodImpl {
    /// Environment variable discovery
    Environment(EnvironmentDiscovery),
    /// DNS-SRV discovery (requires network_discovery module)
    #[allow(dead_code)]
    Dns(super::network_discovery::DnsServiceDiscovery),
    /// Multicast discovery (requires network_discovery module)
    #[allow(dead_code)]
    Multicast(super::network_discovery::MulticastDiscovery),
    /// Port scan discovery (requires network_discovery module)
    #[allow(dead_code)]
    PortScan(super::network_discovery::PortScanDiscovery),
}

impl DiscoveryMethod for DiscoveryMethodImpl {
    fn discover(&self) -> impl Future<Output = Result<Vec<CapabilityInfo>, NestGateError>> + Send {
        async move {
            match self {
                Self::Environment(method) => method.discover().await,
                Self::Dns(method) => method.discover().await,
                Self::Multicast(method) => method.discover().await,
                Self::PortScan(method) => method.discover().await,
            }
        }
    }

    fn method_name(&self) -> &str {
        match self {
            Self::Environment(method) => method.method_name(),
            Self::Dns(method) => method.method_name(),
            Self::Multicast(method) => method.method_name(),
            Self::PortScan(method) => method.method_name(),
        }
    }
}

/// Capability scanner that orchestrates multiple discovery methods
pub struct CapabilityScanner {
    /// Discovery methods to use (enum dispatch for zero-cost async)
    discovery_methods: Vec<DiscoveryMethodImpl>,
    /// Cache of discovered capabilities
    capability_cache: HashMap<String, CapabilityInfo>,
}

impl CapabilityScanner {
    /// Create a new capability scanner with default methods
    #[must_use]
    pub fn new() -> Self {
        let mut scanner = Self {
            discovery_methods: Vec::new(),
            capability_cache: HashMap::new(),
        };

        // Add default discovery methods (using enum dispatch for zero-cost async)
        scanner.add_discovery_method(DiscoveryMethodImpl::Environment(EnvironmentDiscovery::new()));

        scanner
    }

    /// Add a discovery method (using enum dispatch)
    pub fn add_discovery_method(&mut self, method: DiscoveryMethodImpl) {
        self.discovery_methods.push(method);
    }

    /// Scan for all capabilities using all configured methods
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn scan_capabilities(&mut self) -> Result<Vec<CapabilityInfo>, NestGateError> {
        let mut all_capabilities = Vec::new();

        info!(
            "Starting capability discovery with {} methods",
            self.discovery_methods.len()
        );

        for method in &self.discovery_methods {
            debug!("Running discovery method: {}", method.method_name());

            match method.discover().await {
                Ok(capabilities) => {
                    info!(
                        "Method '{}' found {} capabilities",
                        method.method_name(),
                        capabilities.len()
                    );
                    all_capabilities.extend(capabilities);
                }
                Err(e) => {
                    warn!("Discovery method '{}' failed: {}", method.method_name(), e);
                    // Continue with other methods
                }
            }
        }

        // Update cache
        for capability in &all_capabilities {
            self.capability_cache
                .insert(capability.capability_type.clone(), capability.clone());
        }

        info!(
            "Capability discovery complete. Found {} total capabilities",
            all_capabilities.len()
        );

        Ok(all_capabilities)
    }

    /// Get a specific capability from cache
    #[must_use]
    pub fn get_capability(&self, capability_type: &str) -> Option<&CapabilityInfo> {
        self.capability_cache.get(capability_type)
    }

    /// Get all cached capabilities
    #[must_use]
    pub fn get_all_capabilities(&self) -> Vec<&CapabilityInfo> {
        self.capability_cache.values().collect()
    }

    /// Clear the capability cache
    pub fn clear_cache(&mut self) {
        self.capability_cache.clear();
    }
}

impl Default for CapabilityScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    #[tokio::test]
    #[serial]
    async fn test_environment_discovery() {
        // Clean up all possible discovery endpoint environment variables first
        let discovery_patterns = vec![
            "ORCHESTRATION_DISCOVERY_ENDPOINT",
            "SECURITY_DISCOVERY_ENDPOINT",
            "AI_DISCOVERY_ENDPOINT",
            "STORAGE_DISCOVERY_ENDPOINT",
            "MONITORING_DISCOVERY_ENDPOINT",
            "COMPUTE_DISCOVERY_ENDPOINT",
            "NETWORK_DISCOVERY_ENDPOINT",
        ];

        // Clean up any existing environment variables from other tests
        for pattern in &discovery_patterns {
            env::remove_var(pattern);
        }

        // Also clean up any auth/timeout related vars that might interfere
        for pattern in &discovery_patterns {
            let capability_type = pattern
                .strip_suffix("_DISCOVERY_ENDPOINT")
                .unwrap_or(pattern)
                .to_uppercase();
            env::remove_var(format!("{capability_type}_AUTH_KEY"));
            env::remove_var(format!("{capability_type}_TIMEOUT_MS"));
        }

        // Set up only the test environment variables we want
        env::set_var("ORCHESTRATION_DISCOVERY_ENDPOINT", "http://songbird:8080");
        env::set_var("SECURITY_DISCOVERY_ENDPOINT", "http://beardog:9000");

        let discovery = EnvironmentDiscovery::new();
        let capabilities = discovery.discover().await.expect("Operation failed");

        // Debug output to help troubleshoot
        if capabilities.len() != 2 {
            eprintln!("Expected 2 capabilities, got {}", capabilities.len());
            for cap in &capabilities {
                eprintln!("  - {} at {}", cap.capability_type, cap.endpoint);
            }
        }

        assert_eq!(
            capabilities.len(),
            2,
            "Expected exactly 2 capabilities, found {}: {:?}",
            capabilities.len(),
            capabilities
                .iter()
                .map(|c| (&c.capability_type, &c.endpoint))
                .collect::<Vec<_>>()
        );

        let orchestration = capabilities
            .iter()
            .find(|c| c.capability_type == "orchestration")
            .expect("Operation failed");
        assert_eq!(orchestration.endpoint, "http://songbird:8080");
        assert_eq!(orchestration.confidence, 0.95);

        // Clean up all discovery environment variables and related metadata
        for pattern in &discovery_patterns {
            env::remove_var(pattern);
            let capability_type = pattern
                .strip_suffix("_DISCOVERY_ENDPOINT")
                .unwrap_or(pattern)
                .to_uppercase();
            env::remove_var(format!("{capability_type}_AUTH_KEY"));
            env::remove_var(format!("{capability_type}_TIMEOUT_MS"));
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_capability_scanner() {
        // Clean up all possible environment variables first
        let discovery_patterns = vec![
            "ORCHESTRATION_DISCOVERY_ENDPOINT",
            "SECURITY_DISCOVERY_ENDPOINT",
            "AI_DISCOVERY_ENDPOINT",
            "STORAGE_DISCOVERY_ENDPOINT",
            "MONITORING_DISCOVERY_ENDPOINT",
            "COMPUTE_DISCOVERY_ENDPOINT",
            "NETWORK_DISCOVERY_ENDPOINT",
        ];

        for pattern in &discovery_patterns {
            env::remove_var(pattern);
        }

        // Set up test environment BEFORE creating scanner
        env::set_var("AI_DISCOVERY_ENDPOINT", "http://squirrel:7000");
        env::set_var("STORAGE_DISCOVERY_ENDPOINT", "http://storage:8080");

        let mut scanner = CapabilityScanner::new();

        let capabilities = scanner.scan_capabilities().await.expect("Operation failed");
        assert!(
            !capabilities.is_empty(),
            "Expected to find capabilities, but found none. Check environment variable discovery."
        );
        assert_eq!(
            capabilities.len(),
            2,
            "Expected to find 2 capabilities (AI and STORAGE)"
        );

        let ai_capability = scanner.get_capability("ai");
        assert!(
            ai_capability.is_some(),
            "AI capability should be discovered"
        );
        assert_eq!(
            ai_capability.expect("Operation failed").endpoint,
            "http://squirrel:7000"
        );

        let storage_capability = scanner.get_capability("storage");
        assert!(
            storage_capability.is_some(),
            "STORAGE capability should be discovered"
        );
        assert_eq!(
            storage_capability.expect("Operation failed").endpoint,
            "http://storage:8080"
        );

        // Clean up all discovery environment variables
        for pattern in &discovery_patterns {
            env::remove_var(pattern);
        }
    }
}
