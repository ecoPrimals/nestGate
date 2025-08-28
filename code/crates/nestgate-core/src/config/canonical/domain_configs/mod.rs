use crate::Result;
/// **CANONICAL DOMAIN CONFIGURATION SYSTEM**
///
/// This module provides the single source of truth for all domain-specific configurations
/// across the NestGate ecosystem, replacing 80+ scattered configuration structures.
///
/// **CONSOLIDATES**:
/// - Test configurations (UnifiedTestConfig, TestExecutionConfig, etc.)
/// - Storage configurations (StorageConfig, BackendConfig, etc.)
/// - Network configurations (NetworkConfig, ServerConfig, etc.)
/// - Security configurations (SecurityConfig, AuthConfig, etc.)
/// - Performance configurations (PerformanceConfig, MetricsConfig, etc.)
/// - Service configurations (ServiceConfig, DiscoveryConfig, etc.)
///
/// **PROVIDES**:
/// - Unified configuration traits and builders
/// - Environment-driven configuration loading
/// - Compile-time validation
/// - Domain-specific configuration hierarchies
use serde::{Deserialize, Serialize};

// ==================== SECTION ====================

/// **THE** canonical configuration trait that all domain configurations must implement
pub trait CanonicalDomainConfig:
    Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static
{
    /// Configuration domain identifier
    fn domain() -> &'static str;

    /// Validate configuration consistency
    fn validate(&self) -> Result<()>;

    /// Merge with another configuration of the same type
    fn merge(self, other: Self) -> Self;

    /// Load from environment variables with domain prefix
    fn from_environment() -> Result<Self>;

    /// Get configuration schema for documentation
    fn schema() -> serde_json::Value;
}

// ==================== SECTION ====================

pub mod network_configs;
pub mod performance_configs;
pub mod security_configs;
pub mod service_configs;
pub mod storage_configs;
pub mod test_configs;

// ==================== SECTION ====================

// Test configurations
pub use test_configs::{
    BiomeOSTestSettings, CanonicalTestConfig, ChaosBlastRadius, ChaosType, LoadPattern,
    MockConsistencyLevel, MockGlobalSettings, MockServiceConfig, StressLimits, TestChaos,
    TestCleanupStrategy, TestExecution, TestIntegration, TestIsolationLevel, TestMocking,
    TestNetwork, TestPerformance, TestResourceLimits, TestSecurity, ZfsTestSettings,
};

// Storage configurations
pub use storage_configs::{
    BlockBackend, CanonicalStorageConfig, ConflictResolutionStrategy, FilesystemBackend,
    MemoryBackend, NetworkBackend, ObjectBackend, ReplicationSyncMode, StorageBackends,
    StorageMonitoring, StoragePerformance, StorageReplication, StorageSecurity, StorageTiers,
    TierConfig, ZfsBackend, ZfsCompression,
};

// Network configurations
pub use network_configs::{
    CanonicalNetworkConfig, GrpcCompression, GrpcProtocolConfig, HttpProtocolConfig, HttpVersion,
    HttpsProtocolConfig, NetworkClient, NetworkDiscovery, NetworkPerformance, NetworkProtocols,
    NetworkSecurity, NetworkServer, TlsVersion, WebSocketProtocolConfig,
};

// Import NetworkProtocol from storage_configs where it's defined
pub use storage_configs::NetworkProtocol;

// Security configurations
pub use security_configs::{
    AuthMethod, CanonicalSecurityConfig, CipherMode, EncryptionAlgorithm, SecurityAccessControl,
    SecurityAudit, SecurityAuthentication, SecurityAuthorization, SecurityEncryption,
    SecurityKeyManagement,
};

// Performance configurations
pub use performance_configs::{
    CanonicalPerformanceConfig, PerformanceAlerts, PerformanceBenchmarks, PerformanceLimits,
    PerformanceMetrics, PerformanceMonitoring, PerformanceOptimization,
};

// Service configurations
pub use service_configs::{
    AutomationAction, AutomationRule, AutomationTrigger, CanonicalServiceConfig, DependencyType,
    FederationSettings, ScalingAction, ScalingActionType, ScalingPolicy, ServiceAutomation,
    ServiceDependency, ServiceDiscovery, ServiceEcosystem, ServiceHealth, ServiceIdentity,
    ServiceLifecycle,
};
