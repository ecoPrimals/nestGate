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
    ZfsPoolManagement,
    TieredStorage,
    DatasetOperations,
    SnapshotManagement,
    VolumeProvisioning,
    
    // Protocol support
    NfsProtocol,
    SmbProtocol,
    IscsiProtocol,
    HttpProtocol,
    
    // Advanced features
    Compression,
    Deduplication,
    Replication,
    BackupRestore,
    PerformanceOptimization,
    
    // Universal integration capabilities
    UniversalApi,
    RealTimeStreaming,
    MetricsCollection,
    EventBroadcasting,
    
    // AI integration capabilities (for any AI module)
    AiDataOptimization,
    IntelligentTiering,
    PredictiveAnalytics,
    
    // Security integration capabilities (for any security module)
    EncryptionSupport,
    AccessControl,
    AuditLogging,
    
    // Distribution capabilities (for any orchestration module)
    ServiceDiscovery,
    LoadBalancing,
    HealthMonitoring,
    
    // Compute integration capabilities (for any compute module)
    VolumeAttachment,
    ResourceAllocation,
    PerformanceMonitoring,
}

/// Universal primal types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrimalType {
    Storage,  // NestGate
    Security, // Any security module
    AI,       // Any AI module
    Compute,  // Any compute module
    Network,  // Any orchestration module
}

/// Universal request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalRequest {
    pub id: String,
    pub operation: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub requester: String,
    pub timestamp: u64,
}

/// Universal response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalResponse {
    pub id: String,
    pub success: bool,
    pub data: serde_json::Value,
    pub error: Option<String>,
    pub timestamp: u64,
}

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub uptime: u64,
    pub memory_usage: u64,
    pub disk_usage: u64,
    pub active_connections: u32,
}

/// Discovered primal information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredPrimal {
    pub id: String,
    pub primal_type: PrimalType,
    pub endpoint: String,
    pub capabilities: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl PrimalType {
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
    pub config: NestGatePrimalConfig,
    pub capabilities: Vec<StorageCapability>,
    pub metadata: HashMap<String, String>,
}

/// Configuration for NestGate primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestGatePrimalConfig {
    pub host: String,
    pub port: u16,
    pub discovery_enabled: bool,
    pub primal_registry_endpoint: Option<String>,
}
