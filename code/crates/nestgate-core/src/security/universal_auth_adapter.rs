// Universal Authentication Adapter
// **MODERNIZED**: Updated to use direct method calls instead of deprecated trait patterns

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Simple storage access request to security capability
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for StorageAccess operation
pub struct StorageAccessRequest {
    /// Token to validate (from security capability authentication)
    pub token: String,
    /// Storage operation being requested
    /// Storage resource being accessed
    /// Additional context from storage system
    pub context: HashMap<String, String>,
}
/// Simple response from security capability about storage access
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for StorageAccess operation
pub struct StorageAccessResponse {
    /// Whether access is allowed
    pub allowed: bool,
    /// Granted permissions for storage operations
    pub permissions: Vec<String>,
    /// Security capability metadata
    pub metadata: HashMap<String, String>,
    /// Response message from security capability
    pub message: String,
}
/// Standalone authentication configuration for fallback mode
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for StandaloneAuth
pub struct StandaloneAuthConfig {
    /// Enable standalone mode (no security capability required)
    pub enabled: bool,
    /// Default permissions for standalone mode
    pub default_permissions: Vec<String>,
    /// Standalone mode security level
    pub security_level: String,
    /// Token validation settings
    pub token_validation: bool,
}
impl Default for StandaloneAuthConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true, // Conservative default - enable standalone for development
            default_permissions: vec!["read".to_string()], // Read-only by default
            security_level: "development".to_string(),
            token_validation: false, // Simplified validation in standalone
        }
    }
}

/// Authentication Adapter using Universal Adapter Architecture
#[derive(Debug)]
/// Universalauthadapter
pub struct UniversalAuthAdapter {
    /// Reference to the universal adapter for ecosystem communication
    adapter: Option<crate::universal_adapter::PrimalAgnosticAdapter>,
    /// Maximum number of authentication attempts before lockout
    #[allow(dead_code)]
    max_auth_attempts: u32,
    /// Authentication timeout in seconds
    #[allow(dead_code)]
    auth_timeout: u64,
    /// Whether to enable secure authentication features
    secure_mode: bool,
}
impl UniversalAuthAdapter {
    /// Creates a new UniversalAuthAdapter with the provided universal adapter
    pub fn new(
        adapter: Option<crate::universal_adapter::PrimalAgnosticAdapter>,
    ) -> Self {
        Self {
            adapter,
            max_auth_attempts: 3,
            auth_timeout: 60,
            secure_mode: true,
        }
    }

    /// Find security capability or compatible security primal
    async fn find_security_capability(&self) -> Result<String, crate::error::NestGateError> {
        // Use universal adapter to discover available security capabilities
        // This replaces hardcoded primal discovery with dynamic capability discovery

        if let Some(adapter) = &self.adapter {
            debug!("🔍 Discovering security capabilities via universal adapter");

            // Query for security capability providers
            match adapter
                .execute(
                    "discover_capabilities",
                    serde_json::json!({"capability": "security"}),
                )
                .await
            {
                Ok(capabilities_result) => {
                    let empty_vec = vec![];
                    let capabilities = capabilities_result.as_array().unwrap_or(&empty_vec);
                    if let Some(security_capability) = capabilities.first() {
                        info!("✅ Found security capability: {}", security_capability);
                        // Use ServiceDiscoveryConfig for endpoint construction
                        let config = crate::config::discovery_config::ServiceDiscoveryConfig::default();
                        let base_endpoint = config.build_endpoint(config.discovery_base_port);
                        return Ok(format!(
                            "{}/security/{}", base_endpoint, security_capability
                        ));
                    }
                }
                Err(e) => {
                    warn!("⚠️ Security capability discovery failed: {}", e);
                }
            }
        }

        // Fallback to standalone mode
        warn!("🔄 No security capability found, using standalone mode");
        Err(crate::error::NestGateError::configuration(
            
            
        ))
    }

    /// Delegate storage access validation to security capability
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn validate_storage_access(
        &self,
        request: StorageAccessRequest,
    ) -> Result<StorageAccessResponse, crate::error::NestGateError>  {
        // Try to use security capability via universal adapter
        match self.find_security_capability().await {
            Ok(security_endpoint) => {
                info!("🛡️ Delegating storage access validation to security capability");

                if let Some(adapter) = &self.adapter {
                    // Use universal adapter to communicate with security capability
                    match adapter.execute("send_security_request", serde_json::json!({"endpoint": &security_endpoint, "request": &request})).await {
                        Ok(response) => {
                            info!("✅ Security capability validation successful");
                            return Ok(StorageAccessResponse {
                                allowed: response.get("authorized").and_then(|v| v.as_bool()).unwrap_or(false),
                                permissions: response.get("permissions")
                                    .and_then(|v| v.as_array())
                                    .map(|arr| arr.iter()
                                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                        .collect())
                                    .unwrap_or_default(),
                                metadata: response.get("metadata")
                                    .and_then(|v| v.as_object())
                                    .map(|obj| obj.iter()
                                        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                                        .collect())
                                    .unwrap_or_default(),
                                message: "Access validated via security capability".to_string(),
                            });
                        }
                        Err(e) => {
                            warn!("⚠️ Security capability communication failed: {}", e);
    }
    }
                }
            }
            Err(_) => {
                debug!("🔄 Security capability not available, using standalone mode");
            }
        }

        // Fallback to standalone mode validation
        self.validate_standalone_access(&request).await
    }

    /// Validate access in standalone mode (no security capability)
    async fn validate_standalone_access(
        &self,
        request: &StorageAccessRequest,
    ) -> Result<StorageAccessResponse, crate::error::NestGateError> {
        if !self.secure_mode {
            return Err(crate::error::NestGateError::Security(Box::new(
                crate::error::SecurityErrorData {
                    message: "Secure mode disabled and no security capability available"
                        .to_string(),
                    principal: None},
            )));
        }

        info!("🔓 Using standalone authentication mode");

        // Basic token validation in standalone mode
        let mut permissions = vec!["read".to_string(), "list".to_string()]; // Default permissions
        let mut allowed = true;

        if self.secure_mode && request.token.is_empty() {
            allowed = false;
            permissions.clear();
        }

        // Apply security level restrictions
        match "high" {
            // Default security level
            "production" => {
                // Production mode - more restrictive
                permissions.retain(|p| p == "read");
                if request.operation == "write" || request.operation == "delete" {
                    allowed = false;
                }
            }
            "development" => {
                // Development mode - permissive
                if !permissions.contains(&"write".to_string()) {
                    permissions.push("write".to_string());
                }
            }
            _ => {
                // Unknown security level - conservative
                permissions = vec!["read".to_string()];
            }
        }

        Ok(StorageAccessResponse {
            allowed,
            permissions,
            message:
                "Standalone mode authentication - configure security capability for production"
                    .to_string(),
        })
    }

    /// Check if security capability is available
    pub fn security_capability_available(&self) -> bool {
        // Use universal adapter to check security capability availability
        // This replaces hardcoded primal availability checks

        if let Some(adapter) = &self.adapter {
            match adapter
                .execute(
                    "discover_capabilities",
                    serde_json::json!({"capability": "security"}),
                )
                .await
            {
                Ok(capabilities_result) => {
                    let empty_vec = vec![];
                    let capabilities = capabilities_result.as_array().unwrap_or(&empty_vec);
                    !capabilities.is_empty()
                }
                Err(_) => false,
            }
        } else {
            false // No adapter configured
        }
    }

    /// Configure the universal adapter for security capability communication
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn configure_adapter(
        &mut self,
        adapter: crate::universal_adapter::PrimalAgnosticAdapter,
    ) -> Result<(), crate::error::NestGateError>  {
        info!("🔧 Configuring universal adapter for security capability integration");
        self.adapter = Some(adapter);

        // Test the connection
        if self.security_capability_available().await {
            info!("✅ Security capability integration configured successfully");
        } else {
            warn!("⚠️ No security capabilities detected - standalone mode will be used");
        }
        Ok(())
    }

    /// Configure standalone mode settings
    pub fn configure_standalone(&mut self, _config: StandaloneAuthConfig) {
        info!("🔧 Configuring standalone authentication mode");
        // Configuration updated - using secure_mode field instead
    }

    /// Get authentication status without security capability (standalone mode)
    pub fn standalone_auth_status(&self) -> StorageAccessResponse {
        StorageAccessResponse {
            allowed: true, // Allow in standalone mode
            permissions: vec!["read".to_string(), "write".to_string()],
            metadata: HashMap::new(),
            message: "Standalone mode - configure universal adapter to enable security capability delegation".to_string(),
    }
    }
}
