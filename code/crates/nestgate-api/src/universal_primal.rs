//! Universal Primal Architecture for NestGate
//!
//! This module provides the universal primal integration system that allows
//! NestGate to integrate seamlessly with any ecosystem: orchestration, security,
//! AI, compute, or future systems.
//!
//! Key features:
//! - Auto-discovery of compatible ecosystem modules
//! - Dynamic capability negotiation
//! - Universal communication protocols
//! - Graceful fallback when modules are unavailable
//!
//! The system is designed to be ecosystem-agnostic and future-proof.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use tracing::debug;
use tracing::info;
use tracing::warn;

/// Helper function to safely get current timestamp
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or(std::time::Duration::from_secs(0))
        .as_secs()
}

/// Universal Storage Primal Provider
/// Implements the same pattern as security, AI, and other universal primal modules.
#[async_trait]
pub trait StoragePrimalProvider: Send + Sync {
    /// Unique primal identifier (always "nestgate")
    fn primal_id(&self) -> &str;

    /// Primal type category (always "storage")
    fn primal_type(&self) -> PrimalType;

    /// Available storage capabilities
    fn capabilities(&self) -> Vec<StorageCapability>;

    /// Process a universal request
    async fn handle_request(&self, request: UniversalRequest) -> Result<UniversalResponse, String>;

    /// Health check
    async fn health_check(&self) -> HealthStatus;

    /// Register with ecosystem modules
    async fn register_with_ecosystem(&self) -> Result<(), String>;

    /// Get primal metadata
    fn metadata(&self) -> HashMap<String, String>;
}

/// Storage capabilities that NestGate provides
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StorageCapability {
    // Core storage capabilities
    /// ZFS pool management and administration
    ZfsPoolManagement,
    /// Multi-tier storage optimization
    TieredStorage,
    /// Dataset creation and management operations
    DatasetOperations,
    /// Snapshot creation and management
    SnapshotManagement,
    /// Volume provisioning and allocation
    VolumeProvisioning,

    // Protocol support
    /// Network File System protocol support
    NfsProtocol,
    /// Server Message Block protocol support
    SmbProtocol,
    /// Internet Small Computer Systems Interface protocol
    IscsiProtocol,
    /// HTTP-based storage access protocol
    HttpProtocol,

    // Advanced features
    /// Data compression capabilities
    Compression,
    /// Data deduplication features
    Deduplication,
    /// Data replication across systems
    Replication,
    /// Backup and restore functionality
    BackupRestore,
    /// Performance optimization features
    PerformanceOptimization,

    // Universal integration capabilities
    /// Universal API for cross-system integration
    UniversalApi,
    /// Real-time data streaming capabilities
    RealTimeStreaming,
    /// System metrics collection and reporting
    MetricsCollection,
    /// Event broadcasting and notification system
    EventBroadcasting,

    // AI integration capabilities (for any AI module)
    /// AI-powered data optimization and analysis
    AiDataOptimization,
    /// Intelligent storage tier management
    IntelligentTiering,
    /// Predictive analytics for system optimization
    PredictiveAnalytics,

    // Security integration capabilities (for any security module)
    /// Encryption and decryption support
    EncryptionSupport,
    /// Access control and permission management
    AccessControl,
    /// Audit logging and compliance tracking
    AuditLogging,

    // Distribution capabilities (for any orchestration module)
    /// Service discovery and registration
    ServiceDiscovery,
    /// Load balancing and traffic distribution
    LoadBalancing,
    /// Health monitoring and status tracking
    HealthMonitoring,

    // Compute integration capabilities (for any compute module)
    /// Volume attachment and storage mounting
    VolumeAttachment,
    /// Resource allocation and management
    ResourceAllocation,
    /// Performance monitoring and metrics
    PerformanceMonitoring,
}

/// Universal primal types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrimalType {
    /// Storage services (NestGate)
    Storage,
    /// Security services (Any security module)
    Security,
    /// AI services (Any AI module)
    AI,
    /// Compute services (Any compute module)
    Compute,
    /// Network orchestration services (Any orchestration module)
    Network,
}

/// Universal request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalRequest {
    /// Unique request identifier
    pub id: String,
    /// Operation to be performed
    pub operation: String,
    /// Request parameters as key-value pairs
    pub parameters: HashMap<String, serde_json::Value>,
    /// Identity of the requester
    pub requester: String,
    /// Request timestamp (Unix epoch)
    pub timestamp: u64,
}

/// Universal response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalResponse {
    /// Response identifier matching the request
    pub id: String,
    /// Whether the operation succeeded
    pub success: bool,
    /// Response data payload
    pub data: serde_json::Value,
    /// Error message if operation failed
    pub error: Option<String>,
    /// Response timestamp (Unix epoch)
    pub timestamp: u64,
}

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Current health status (e.g., "healthy", "degraded", "unhealthy")
    pub status: String,
    /// System uptime in seconds
    pub uptime: u64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Disk usage in bytes
    pub disk_usage: u64,
    /// Number of active connections
    pub active_connections: u32,
}

/// Discovered primal information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredPrimal {
    /// Unique primal identifier
    pub id: String,
    /// Type of primal service
    pub primal_type: PrimalType,
    /// Network endpoint for communication
    pub endpoint: String,
    /// List of supported capabilities
    pub capabilities: Vec<String>,
    /// Additional metadata about the primal
    pub metadata: HashMap<String, String>,
}

impl PrimalType {
    /// Convert primal type to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            PrimalType::Storage => "storage",
            PrimalType::Security => "security",
            PrimalType::AI => "ai",
            PrimalType::Compute => "compute",
            PrimalType::Network => "network",
        }
    }
}

/// NestGate Universal Storage Primal
pub struct NestGateStoragePrimal {
    /// Primal configuration settings
    pub config: NestGatePrimalConfig,
    /// Storage capabilities provided by this primal
    pub capabilities: Vec<StorageCapability>,
    /// Additional metadata about this primal instance
    pub metadata: HashMap<String, String>,
}

/// Configuration for NestGate primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestGatePrimalConfig {
    /// Host address for the primal service
    pub host: String,
    /// Port number for the primal service
    pub port: u16,
    /// Whether primal discovery is enabled
    pub discovery_enabled: bool,
    /// Optional endpoint for primal registry
    pub primal_registry_endpoint: Option<String>,
}

impl Default for NestGateStoragePrimal {
    fn default() -> Self {
        Self::new()
    }
}

impl NestGateStoragePrimal {
    /// Create a new NestGate storage primal with default configuration
    pub fn new() -> Self {
        Self {
            config: NestGatePrimalConfig {
                host: "127.0.0.1".to_string(),
                port: nestgate_core::config::defaults::NetworkPortDefaults::get_api_port(),
                discovery_enabled: true,
                primal_registry_endpoint: None,
            },
            capabilities: vec![
                StorageCapability::ZfsPoolManagement,
                StorageCapability::TieredStorage,
                StorageCapability::DatasetOperations,
                StorageCapability::SnapshotManagement,
                StorageCapability::UniversalApi,
                StorageCapability::RealTimeStreaming,
                StorageCapability::MetricsCollection,
                StorageCapability::ServiceDiscovery,
            ],
            metadata: {
                let mut metadata = HashMap::new();
                metadata.insert("version".into(), "1.0.0".into());
                metadata.insert("name".into(), "NestGate".into());
                metadata.insert("description".into(), "ZFS Storage Primal".into());
                metadata.insert("primal_type".into(), "storage".into());
                metadata.insert("api_version".into(), "universal/v1".into());
                metadata
            },
        }
    }
}

#[async_trait]
impl StoragePrimalProvider for NestGateStoragePrimal {
    fn primal_id(&self) -> &str {
        "nestgate"
    }

    fn primal_type(&self) -> PrimalType {
        PrimalType::Storage
    }

    fn capabilities(&self) -> Vec<StorageCapability> {
        self.capabilities.clone()
    }

    async fn handle_request(&self, request: UniversalRequest) -> Result<UniversalResponse, String> {
        // Handle universal requests from other primals
        match request.operation.as_str() {
            "health_check" => {
                let health = self.health_check().await;
                Ok(UniversalResponse {
                    id: request.id,
                    success: true,
                    data: serde_json::to_value(health).map_err(|e| e.to_string())?,
                    error: None,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                })
            }
            "get_capabilities" => {
                let capabilities = self.capabilities();
                Ok(UniversalResponse {
                    id: request.id,
                    success: true,
                    data: serde_json::to_value(capabilities).map_err(|e| e.to_string())?,
                    error: None,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                })
            }
            "get_metadata" => {
                let metadata = self.metadata();
                Ok(UniversalResponse {
                    id: request.id,
                    success: true,
                    data: serde_json::to_value(metadata).map_err(|e| e.to_string())?,
                    error: None,
                    timestamp: current_timestamp(),
                })
            }
            _ => Ok(UniversalResponse {
                id: request.id,
                success: false,
                data: serde_json::Value::Null,
                error: Some(format!("Unknown operation: {}", request.operation)),
                timestamp: current_timestamp(),
            }),
        }
    }

    async fn health_check(&self) -> HealthStatus {
        HealthStatus {
            status: "healthy".to_string(),
            uptime: current_timestamp(),
            memory_usage: 0,       // Would need actual memory monitoring
            disk_usage: 0,         // Would need actual disk monitoring
            active_connections: 0, // Would need actual connection monitoring
        }
    }

    async fn register_with_ecosystem(&self) -> Result<(), String> {
        // Removed unused tracing import

        info!("🌐 Starting ecosystem registration for NestGate storage primal");

        // Check if registry endpoint is configured
        if let Some(registry_endpoint) = &self.config.primal_registry_endpoint {
            info!(
                "📡 Registering with primal registry at: {}",
                registry_endpoint
            );

            // Create registration payload
            let registration_payload = serde_json::json!({
                "primal_id": self.primal_id(),
                "primal_type": self.primal_type(),
                "capabilities": self.capabilities(),
                "metadata": self.metadata(),
                "endpoint": format!("{}:{}", self.config.host, self.config.port),
                "timestamp": current_timestamp()
            });

            // Attempt to register with the primal registry
            match reqwest::Client::new()
                .post(format!("{registry_endpoint}/register"))
                .json(&registration_payload)
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        info!("✅ Successfully registered with primal registry");
                        return Ok(());
                    } else {
                        warn!(
                            "⚠️ Registry registration failed with status: {}",
                            response.status()
                        );
                    }
                }
                Err(e) => {
                    warn!("⚠️ Failed to connect to primal registry: {}", e);
                }
            }
        }

        // Fallback to local ecosystem discovery
        info!("🔍 Attempting local ecosystem discovery");

        // Try to discover other primals on the local network
        let discovery_result = self.discover_local_primals().await;
        match discovery_result {
            Ok(discovered_primals) => {
                info!("🌐 Discovered {} local primals", discovered_primals.len());
                for primal in discovered_primals {
                    info!(
                        "   - {} ({}): {}",
                        primal.id,
                        primal.primal_type.as_str(),
                        primal.endpoint
                    );
                }
                Ok(()) // Success - we logged the discovered primals
            }
            Err(e) => {
                warn!("⚠️ Local discovery failed: {}", e);
                info!("🔄 Continuing in standalone mode");
                Ok(()) // Don't fail if discovery fails
            }
        }
    }

    fn metadata(&self) -> HashMap<String, String> {
        self.metadata.clone()
    }
}

impl NestGateStoragePrimal {
    /// Discover other primals on the local network
    async fn discover_local_primals(&self) -> Result<Vec<DiscoveredPrimal>, String> {
        // Removed unused tracing import

        let mut discovered_primals = Vec::new();

        // Check common primal ports (8080-8090, 3000-3010)
        let common_ports = vec![8080, 8081, 8082, 8090, 3000, 3001, 3002, 3010];

        for port in common_ports {
            if port == self.config.port {
                continue; // Skip our own port
            }

            let endpoint = format!("http://{}:{}", self.config.host, port);
            debug!("🔍 Checking for primal at: {}", endpoint);

            // Try to connect and get primal info
            match reqwest::Client::new()
                .get(format!("{endpoint}/primal/info"))
                .timeout(std::time::Duration::from_secs(2))
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        if let Ok(primal_info) = response.json::<DiscoveredPrimal>().await {
                            info!("✅ Discovered primal: {} at {}", primal_info.id, endpoint);
                            discovered_primals.push(primal_info);
                        }
                    }
                }
                Err(_) => {
                    // Silent failure - just means no primal at this endpoint
                    debug!("❌ No primal found at: {}", endpoint);
                }
            }
        }

        Ok(discovered_primals)
    }
}
