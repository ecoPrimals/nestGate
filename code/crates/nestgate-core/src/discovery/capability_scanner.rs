//! **CAPABILITY SCANNER**
//!
//! Environment capability detection implementation for the Infant Discovery Architecture.

use crate::error::NestGateError;
use std::collections::HashMap;
use std::future::Future;
use tracing::{debug, info, warn};

/// Information about a discovered capability
#[derive(Debug, Clone)]
/// Capabilityinfo
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
///
/// **MODERN CONCURRENT-SAFE DESIGN:**
/// This uses dependency injection with immutable configuration instead of
/// reading global environment variables at runtime. This eliminates race
/// conditions and makes the code truly thread-safe.
///
/// # Example
///
/// ```rust
/// use std::sync::Arc;
/// use nestgate_core::discovery::{EnvironmentDiscovery, EnvironmentDiscoveryConfig};
///
/// // Production: Load from environment once at startup
/// let discovery = EnvironmentDiscovery::from_env();
///
/// // Testing: Inject config (no env var pollution!)
/// let mut config = EnvironmentDiscoveryConfig::new();
/// config.set_endpoint("orchestration", "http://test:8080");
/// let discovery = EnvironmentDiscovery::with_config(Arc::new(config));
/// ```
#[derive(Debug, Clone)]
/// Environmentdiscovery
pub struct EnvironmentDiscovery {
    /// Immutable configuration (thread-safe via Arc)
    config: std::sync::Arc<super::capability_scanner_config::EnvironmentDiscoveryConfig>,
}

impl EnvironmentDiscovery {
    /// Create a new environment discovery scanner with default configuration
    ///
    /// **Note:** This loads environment variables once at construction time,
    /// not on every `discover()` call. This is thread-safe and efficient.
    #[must_use]
    pub fn new() -> Self {
        Self::from_env()
    }

    /// Create from environment variables (recommended for production)
    ///
    /// Loads configuration once from environment, then uses immutable config.
    #[must_use]
    pub fn from_env() -> Self {
        Self::with_config(super::capability_scanner_config::shared_config_from_env())
    }

    /// Create with explicit configuration (recommended for testing)
    ///
    /// This allows injecting test configuration without polluting the
    /// environment. Makes tests truly isolated and parallel-safe.
    #[must_use]
    pub fn with_config(
        config: std::sync::Arc<super::capability_scanner_config::EnvironmentDiscoveryConfig>,
    ) -> Self {
        Self { config }
    }

    /// Get the current configuration (for inspection/testing)
    #[must_use]
    pub fn config(&self) -> &super::capability_scanner_config::EnvironmentDiscoveryConfig {
        &self.config
    }
}

impl Default for EnvironmentDiscovery {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl DiscoveryMethod for EnvironmentDiscovery {
    /// Discover
    async fn discover(&self) -> Result<Vec<CapabilityInfo>, NestGateError> {
        let mut capabilities = Vec::new();

        debug!("Scanning configuration for capabilities");

        // Read from IMMUTABLE config, not global environment
        for (capability_type, endpoint) in self.config.endpoints() {
            info!("Found {} capability at: {}", capability_type, endpoint);

            let mut metadata = HashMap::new();
            metadata.insert("source".to_string(), "environment".to_string());

            // Get all metadata for this capability from config
            let cap_metadata = self.config.all_metadata(capability_type);
            for (key, value) in cap_metadata {
                metadata.insert(key, value);
            }

            capabilities.push(CapabilityInfo {
                capability_type: capability_type.clone(),
                endpoint: endpoint.clone(),
                confidence: 0.95, // High confidence for explicit configuration
                metadata,
            });
        }

        if capabilities.is_empty() {
            warn!("No capabilities found in configuration");
        } else {
            info!(
                "Found {} capabilities via environment discovery",
                capabilities.len()
            );
        }

        Ok(capabilities)
    }

    /// Method Name
    fn method_name(&self) -> &str {
        "environment"
    }
}

/// **DISCOVERY METHOD ENUM**
///
/// Enum dispatch for discovery methods - zero-cost alternative to `Box<dyn DiscoveryMethod>`.
/// This enables native async while maintaining runtime polymorphism through enum dispatch.
#[derive(Debug)]
/// Discoverymethodimpl
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
    /// Discover
    async fn discover(&self) -> Result<Vec<CapabilityInfo>, NestGateError> {
        match self {
            Self::Environment(method) => method.discover().await,
            Self::Dns(method) => method.discover().await,
            Self::Multicast(method) => method.discover().await,
            Self::PortScan(method) => method.discover().await,
        }
    }

    /// Method Name
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

        // Return the configured scanner
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
    /// Returns the default instance
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

// ==================== TESTS ====================
#[cfg(test)]
#[path = "capability_scanner_tests_v2.rs"]
mod capability_scanner_tests;
