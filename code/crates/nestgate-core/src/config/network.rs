// Removed unused error imports
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
// Removed unused std import

/// WebSocket configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketConfig {
    /// WebSocket port
    pub port: u16,
    /// Maximum connections
    pub max_connections: usize,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    /// Enable compression
    pub compression: bool,
    /// Heartbeat interval in seconds
    pub heartbeat_interval: u64,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            max_connections: 1000,
            connection_timeout: 30,
            compression: true,
            heartbeat_interval: 30,
        }
    }
}

/// HTTP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    /// HTTP port
    pub port: u16,
    /// Enable HTTPS
    pub https: bool,
    /// Request timeout in seconds
    pub request_timeout: u64,
    /// Maximum request body size in bytes
    pub max_body_size: usize,
    /// Enable CORS
    pub cors_enabled: bool,
    /// Allowed origins for CORS
    pub cors_origins: Vec<String>,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            https: false,
            request_timeout: 30,
            max_body_size: 1024 * 1024 * 16, // 16MB
            cors_enabled: true,
            cors_origins: vec!["*".to_string()],
        }
    }
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Bind address for the server
    pub bind_address: String,

    /// API server address
    pub api_address: String,

    /// Streaming RPC address
    pub streaming_rpc_address: String,

    /// WebSocket address
    pub websocket_address: String,

    /// Server hostname
    pub hostname: String,

    /// External hostname (for federation)
    pub external_hostname: String,

    /// API port
    pub api_port: u16,

    /// Streaming RPC port
    pub streaming_rpc_port: u16,

    /// WebSocket port
    pub websocket_port: u16,

    /// Web interface port
    pub web_port: u16,

    /// Maximum concurrent connections
    pub max_concurrent_connections: usize,

    /// Connection timeout in seconds
    pub connection_timeout_seconds: u64,

    /// Request timeout in seconds
    pub request_timeout_seconds: u64,
}

/// Service endpoints configuration (replaces hardcoded URLs)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoints {
    /// Service endpoint URLs
    pub services: HashMap<String, String>,

    /// API base URL
    pub api_base_url: String,

    /// WebSocket base URL
    pub websocket_base_url: String,

    /// Static files base URL
    pub static_base_url: String,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            bind_address: default_bind_address(),
            api_address: default_api_address(),
            streaming_rpc_address: default_streaming_rpc_address(),
            websocket_address: default_websocket_address(),
            hostname: default_hostname(),
            external_hostname: default_external_hostname(),
            api_port: default_api_port(),
            streaming_rpc_port: default_streaming_rpc_port(),
            websocket_port: default_websocket_port(),
            web_port: default_web_port(),
            max_concurrent_connections: default_max_concurrent_connections(),
            connection_timeout_seconds: default_connection_timeout_seconds(),
            request_timeout_seconds: default_request_timeout_seconds(),
        }
    }
}

impl NetworkConfig {
    /// Get the full API URL
    pub fn api_url(&self) -> String {
        format!("http://{}:{}", self.hostname, self.api_port)
    }

    /// Get the full WebSocket URL
    pub fn websocket_url(&self) -> String {
        format!("ws://{}:{}", self.hostname, self.websocket_port)
    }

    /// Get the full streaming RPC URL
    pub fn streaming_rpc_url(&self) -> String {
        format!("http://{}:{}", self.hostname, self.streaming_rpc_port)
    }

    /// Validate network configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate bind address
        if let Err(e) = self.bind_address.parse::<SocketAddr>() {
            return Err(format!("Invalid bind address: {e}"));
        }

        // Validate API address
        if let Err(e) = self.api_address.parse::<SocketAddr>() {
            return Err(format!("Invalid API address: {e}"));
        }

        // Validate streaming RPC address
        if let Err(e) = self.streaming_rpc_address.parse::<SocketAddr>() {
            return Err(format!("Invalid streaming RPC address: {e}"));
        }

        // Validate WebSocket address
        if let Err(e) = self.websocket_address.parse::<SocketAddr>() {
            return Err(format!("Invalid WebSocket address: {e}"));
        }

        // Validate ports
        if self.api_port == 0
            || self.streaming_rpc_port == 0
            || self.websocket_port == 0
            || self.web_port == 0
        {
            return Err("Ports must be non-zero".to_string());
        }

        // Validate hostnames
        if self.hostname.is_empty() {
            return Err("Hostname cannot be empty".to_string());
        }

        if self.external_hostname.is_empty() {
            return Err("External hostname cannot be empty".to_string());
        }

        // Validate timeouts
        if self.connection_timeout_seconds == 0 {
            return Err("Connection timeout must be greater than 0".to_string());
        }

        if self.request_timeout_seconds == 0 {
            return Err("Request timeout must be greater than 0".to_string());
        }

        // Validate max connections
        if self.max_concurrent_connections == 0 {
            return Err("Max concurrent connections must be greater than 0".to_string());
        }
        Ok(())
    }
}

impl Default for ServiceEndpoints {
    fn default() -> Self {
        let services = HashMap::new();

        // ✅ UNIVERSAL DATA CAPABILITIES: No hardcoded provider endpoints
        // Data providers register their capabilities dynamically through the universal adapter
        // External systems can provide genome data, model data, research data, etc.
        // without NestGate knowing their specific identity

        // ✅ CAPABILITY-BASED: All primal discovery goes through universal adapter
        // No hardcoded primal endpoints - primals register their capabilities dynamically
        // Use environment variables for discovery endpoints if needed:
        // ORCHESTRATION_DISCOVERY_URL, SECURITY_DISCOVERY_URL, etc.

        Self {
            services,
            api_base_url: std::env::var("NESTGATE_API_URL")
                .unwrap_or_else(|_| "http://localhost:8000".to_string()),
            websocket_base_url: std::env::var("NESTGATE_WEBSOCKET_URL")
                .unwrap_or_else(|_| "ws://localhost:8080".to_string()),
            static_base_url: std::env::var("NESTGATE_STATIC_URL")
                .unwrap_or_else(|_| "http://localhost:8000/static".to_string()),
        }
    }
}

impl ServiceEndpoints {
    /// Get a service endpoint URL
    pub fn get_service_url(&self, service: &str) -> Option<&str> {
        self.services.get(service).map(|s| s.as_str())
    }

    /// Add or update a service endpoint
    pub fn set_service_url(&mut self, service: String, url: String) {
        self.services.insert(service, url);
    }

    /// Remove a service endpoint
    pub fn remove_service(&mut self, service: &str) -> Option<String> {
        self.services.remove(service)
    }

    /// Get all service names
    pub fn service_names(&self) -> Vec<&str> {
        self.services.keys().map(|s| s.as_str()).collect()
    }

    /// Check if a service is configured
    pub fn has_service(&self, service: &str) -> bool {
        self.services.contains_key(service)
    }

    /// Validate service endpoints
    pub fn validate(&self) -> Result<(), String> {
        // Validate base URLs
        if self.api_base_url.is_empty() {
            return Err("API base URL cannot be empty".to_string());
        }

        if self.websocket_base_url.is_empty() {
            return Err("WebSocket base URL cannot be empty".to_string());
        }

        if self.static_base_url.is_empty() {
            return Err("Static base URL cannot be empty".to_string());
        }

        // Validate service URLs
        for (service, url) in &self.services {
            if url.is_empty() {
                return Err(format!("Service URL for '{service}' cannot be empty"));
            }
        }
        Ok(())
    }

    /// Register a discovered primal provider endpoint
    pub fn register_discovered_provider(
        &mut self,
        provider_type: &str,
        capabilities: &[String],
        endpoint: String,
    ) {
        // Create capability-based service name
        let service_name = if capabilities.contains(&"text-generation".to_string()) {
            "ai-text-generation"
        } else if capabilities.contains(&"embedding".to_string()) {
            "ai-embedding"
        } else if capabilities.contains(&"encryption".to_string()) {
            "security-encryption"
        } else if capabilities.contains(&"authentication".to_string()) {
            "security-auth"
        } else if capabilities.contains(&"service-discovery".to_string()) {
            "orchestration-discovery"
        } else if capabilities.contains(&"load-balancing".to_string()) {
            "orchestration-balancer"
        } else {
            provider_type
        };

        self.services.insert(service_name.to_string(), endpoint);
    }

    /// Get AI provider endpoints by capability
    pub fn get_ai_providers_with_capability(&self, capability: &str) -> Vec<&str> {
        let mut providers = Vec::new();

        match capability {
            "text-generation" => {
                if let Some(url) = self.services.get("ai-text-generation") {
                    providers.push(url.as_str());
                }
            }
            "embedding" => {
                if let Some(url) = self.services.get("ai-embedding") {
                    providers.push(url.as_str());
                }
            }
            _ => {
                // Look for any AI providers
                for (service, url) in &self.services {
                    if service.starts_with("ai-") {
                        providers.push(url.as_str());
                    }
                }
            }
        }

        providers
    }

    /// Get security provider endpoints by capability
    pub fn get_security_providers_with_capability(&self, capability: &str) -> Vec<&str> {
        let mut providers = Vec::new();

        match capability {
            "encryption" => {
                if let Some(url) = self.services.get("security-encryption") {
                    providers.push(url.as_str());
                }
            }
            "authentication" => {
                if let Some(url) = self.services.get("security-auth") {
                    providers.push(url.as_str());
                }
            }
            _ => {
                // Look for any security providers
                for (service, url) in &self.services {
                    if service.starts_with("security-") {
                        providers.push(url.as_str());
                    }
                }
            }
        }

        providers
    }

    /// Get orchestration provider endpoints by capability
    pub fn get_orchestration_providers_with_capability(&self, capability: &str) -> Vec<&str> {
        let mut providers = Vec::new();

        match capability {
            "service-discovery" => {
                if let Some(url) = self.services.get("orchestration-discovery") {
                    providers.push(url.as_str());
                }
            }
            "load-balancing" => {
                if let Some(url) = self.services.get("orchestration-balancer") {
                    providers.push(url.as_str());
                }
            }
            _ => {
                // Look for any orchestration providers
                for (service, url) in &self.services {
                    if service.starts_with("orchestration-") {
                        providers.push(url.as_str());
                    }
                }
            }
        }

        providers
    }

    /// Check if universal discovery is enabled
    pub fn is_universal_discovery_enabled(&self) -> bool {
        std::env::var("NESTGATE_ENABLE_PRIMAL_AUTO_DISCOVERY")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(true)
    }

    /// Get service endpoint by capability (replaces legacy primal-specific lookups)
    pub fn get_service_by_capability(&self, capability: &str) -> Option<&str> {
        // Map capabilities to service names - this is the correct sovereignty pattern
        let service_name = match capability {
            "ai-text-generation" | "artificial-intelligence" => "ai-text-generation",
            "ai-embedding" | "embedding" => "ai-embedding",
            "security-encryption" | "security" | "encryption" => "security-encryption",
            "orchestration-discovery" | "orchestration" | "discovery" => "orchestration-discovery",
            "storage" | "file-management" => "storage-service",
            "compute" | "processing" => "compute-service",
            _ => capability, // Direct capability name lookup
        };

        self.services.get(service_name).map(|s| s.as_str())
    }

    /// Get legacy endpoint for backward compatibility (deprecated - use get_service_by_capability)
    /// This method will be removed in future versions
    #[deprecated(
        note = "Use get_service_by_capability instead - this violates sovereignty principles"
    )]
    pub fn get_legacy_endpoint(&self, service: &str) -> Option<&str> {
        // Legacy mapping - DO NOT ADD NEW ENTRIES HERE
        // These mappings are deprecated and violate sovereignty principles
        match service {
            "squirrel" => {
                eprintln!("WARNING: Using deprecated primal name 'squirrel' - use capability 'ai-text-generation' instead");
                self.get_service_by_capability("ai-text-generation")
            }
            "toadstool" => {
                eprintln!("WARNING: Using deprecated primal name 'toadstool' - use capability 'ai-embedding' instead");
                self.get_service_by_capability("ai-embedding")
            }
            "beardog" => {
                eprintln!("WARNING: Using deprecated primal name 'beardog' - use capability 'security-encryption' instead");
                self.get_service_by_capability("security-encryption")
            }
            "songbird" => {
                eprintln!("WARNING: Using deprecated primal name 'songbird' - use capability 'orchestration-discovery' instead");
                self.get_service_by_capability("orchestration-discovery")
            }
            _ => self.services.get(service).map(|s| s.as_str()),
        }
    }
}

// Default value functions
pub fn default_bind_address() -> String {
    std::env::var("NESTGATE_BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8000".to_string())
}

pub fn default_api_address() -> String {
    std::env::var("NESTGATE_API_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8000".to_string())
}

pub fn default_streaming_rpc_address() -> String {
    std::env::var("NESTGATE_STREAMING_RPC_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8001".to_string())
}

pub fn default_websocket_address() -> String {
    std::env::var("NESTGATE_WEBSOCKET_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".to_string())
}

pub fn default_hostname() -> String {
    std::env::var("NESTGATE_HOSTNAME").unwrap_or_else(|_| "localhost".to_string())
}

pub fn default_external_hostname() -> String {
    std::env::var("NESTGATE_EXTERNAL_HOSTNAME").unwrap_or_else(|_| "localhost".to_string())
}

pub fn default_api_port() -> u16 {
    std::env::var("NESTGATE_API_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8000)
}

pub fn default_streaming_rpc_port() -> u16 {
    std::env::var("NESTGATE_STREAMING_RPC_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8001)
}

pub fn default_websocket_port() -> u16 {
    std::env::var("NESTGATE_WEBSOCKET_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080)
}

pub fn default_web_port() -> u16 {
    std::env::var("NESTGATE_WEB_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8000)
}

pub fn default_max_concurrent_connections() -> usize {
    std::env::var("NESTGATE_MAX_CONCURRENT_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000)
}

pub fn default_connection_timeout_seconds() -> u64 {
    std::env::var("NESTGATE_CONNECTION_TIMEOUT_SECONDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30)
}

pub fn default_request_timeout_seconds() -> u64 {
    std::env::var("NESTGATE_REQUEST_TIMEOUT_SECONDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(60)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_config_default() {
        let config = NetworkConfig::default();
        assert_eq!(config.hostname, "localhost");
        assert_eq!(config.api_port, 8000);
        assert_eq!(config.websocket_port, 8080);
        assert_eq!(config.max_concurrent_connections, 1000);
    }

    #[test]
    fn test_network_config_urls() {
        let config = NetworkConfig::default();
        assert_eq!(config.api_url(), "http://localhost:8000");
        assert_eq!(config.websocket_url(), "ws://localhost:8080");
        assert_eq!(config.streaming_rpc_url(), "http://localhost:8001");
    }

    #[test]
    fn test_service_endpoints_default() {
        let endpoints = ServiceEndpoints::default();

        // Universal architecture: External services are always available
        assert!(endpoints.has_service("huggingface"));
        assert!(endpoints.has_service("ncbi"));

        // Primal services are only available when legacy endpoints are enabled
        // By default, they should be discovered dynamically, not hardcoded
        // Test that deprecated primal names are not present by default
        assert!(!endpoints.has_service("deprecated-security"));
        assert!(!endpoints.has_service("deprecated-orchestration"));
        assert!(!endpoints.has_service("deprecated-ai-text"));
        assert!(!endpoints.has_service("deprecated-ai-embedding"));

        assert!(!endpoints.has_service("nonexistent"));
    }

    #[test]
    fn test_service_endpoints_operations() {
        let mut endpoints = ServiceEndpoints::default();

        // Test getting external service URL (always available)
        assert!(endpoints.get_service_url("huggingface").is_some());
        assert!(endpoints.get_service_url("ncbi").is_some());

        // Test that services are not hardcoded by default (universal architecture)
        assert!(endpoints.get_service_url("security-service").is_none());
        assert!(endpoints.get_service_url("nonexistent").is_none());

        // Test setting service URL (for dynamic discovery)
        endpoints.set_service_url(
            "security-service".to_string(),
            "http://discovered:8001".to_string(),
        );
        assert!(endpoints.has_service("security-service"));
        assert_eq!(
            endpoints.get_service_url("security-service"),
            Some("http://discovered:8001")
        );

        // Test setting custom service URL
        endpoints.set_service_url("custom".to_string(), "http://localhost:9000".to_string());
        assert!(endpoints.has_service("custom"));
        assert_eq!(
            endpoints.get_service_url("custom"),
            Some("http://localhost:9000")
        );

        // Test removing service
        let removed = endpoints.remove_service("custom");
        assert!(removed.is_some());
        assert!(!endpoints.has_service("custom"));

        // Test service names include external services
        let names = endpoints.names();
        assert!(names.contains(&"huggingface"));
        assert!(names.contains(&"ncbi"));
        // And dynamically added services
        assert!(names.contains(&"security-service"));
    }

    #[test]
    fn test_network_config_validation() {
        let mut config = NetworkConfig::default();

        // Valid configuration should pass
        assert!(config.validate().is_ok());

        // Invalid bind address should fail
        config.bind_address = "invalid".to_string();
        assert!(config.validate().is_err());

        // Reset and test zero port
        config.bind_address = "0.0.0.0:8000".to_string();
        config.api_port = 0;
        assert!(config.validate().is_err());

        // Reset and test empty hostname
        config.api_port = 8000;
        config.hostname = "".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_service_endpoints_validation() {
        let mut endpoints = ServiceEndpoints::default();

        // Valid configuration should pass
        assert!(endpoints.validate().is_ok());

        // Empty API base URL should fail
        endpoints.api_base_url = "".to_string();
        assert!(endpoints.validate().is_err());

        // Reset and test empty service URL
        endpoints.api_base_url = "http://localhost:8000".to_string();
        endpoints.set_service_url("test".to_string(), "".to_string());
        assert!(endpoints.validate().is_err());
    }

    #[test]
    fn test_environment_variables() {
        // Test default values when environment variables are not set
        assert_eq!(default_hostname(), "localhost");
        assert_eq!(default_api_port(), 8000);
        assert_eq!(default_websocket_port(), 8080);
        assert_eq!(default_max_concurrent_connections(), 1000);
    }
}
