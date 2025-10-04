// **CONSOLIDATED CONSTANTS SYSTEM - MODULAR**
//! Consolidated Constants functionality and utilities.
// This module provides the unified constant system that consolidates all scattered
//! constant definitions across the `NestGate` ecosystem into a single, canonical source.
//! Consolidated Constants functionality and utilities.
// **CONSOLIDATES**:
//! - 564+ scattered constant definitions across all crates
//! - Duplicate DEFAULT_*, MAX_*, MIN_*, TIMEOUT* patterns
//! - Hardcoded values like ports, timeouts, sizes
//! - Magic numbers in performance calculations
//! - Domain-specific constant fragments
//!
//! Consolidated Constants functionality and utilities.
//!
// **PROVIDES**:
//! - Single source of truth for all constants
//! - Domain-organized constant hierarchies
//! - Type-safe constant definitions
//! - Environment-aware constants
//! - Migration utilities from legacy constants
//!
//! Consolidated Constants functionality and utilities.
//!
// **ARCHITECTURE**: Modular design with domain-specific modules
//! - Each domain module is under 2000 lines for maintainability
//! - Clear separation of concerns by domain
//! - Easy to extend and modify individual domains

// Import domain-specific constants
use super::domains::{ApiDomainConstants, NetworkDomainConstants, StorageDomainConstants};

// ==================== CORE DOMAIN CONSTANTS ====================

/// **CONSOLIDATED DOMAIN CONSTANTS**
///
/// This structure brings together all domain-specific constants
/// under a single, well-organized hierarchy.
#[derive(Debug, Clone, Default)]
pub struct ConsolidatedDomainConstants {
    /// Network and connectivity constants
    pub network: NetworkDomainConstants,

    /// Storage and ZFS constants
    pub storage: StorageDomainConstants,

    /// API and HTTP service constants
    pub api: ApiDomainConstants,

    /// Security and authentication constants
    pub security: SecurityDomainConstants,

    /// Performance and optimization constants
    pub performance: PerformanceDomainConstants,

    /// System and environment constants
    pub system: SystemDomainConstants,

    /// Testing and development constants
    pub testing: TestingDomainConstants,

    /// MCP protocol constants
    pub mcp: McpDomainConstants,

    /// Automation workflow constants
    pub automation: AutomationDomainConstants,
}

// ==================== PLACEHOLDER DOMAIN CONSTANTS ====================
// Domain constants are consolidated here for performance and consistency

/// Security domain constants - placeholder for future extraction
#[derive(Debug, Clone)]
pub struct SecurityDomainConstants {
    pub auth: AuthConstants,
    pub encryption: EncryptionConstants,
    pub session: SessionConstants,
    pub rate_limiting: RateLimitingConstants,
    pub policies: SecurityPolicyConstants,
}
impl Default for SecurityDomainConstants {
    fn default() -> Self {
        Self {
            auth: AuthConstants,
            encryption: EncryptionConstants,
            session: SessionConstants,
            rate_limiting: RateLimitingConstants,
            policies: SecurityPolicyConstants,
        }
    }
}

/// Performance domain constants - placeholder for future extraction
#[derive(Debug, Clone, Default)]
pub struct PerformanceDomainConstants {
    pub concurrency: ConcurrencyConstants,
    pub cache: CacheConstants,
    pub buffers: BufferConstants,
    pub benchmarks: BenchmarkConstants,
    pub limits: ResourceLimitConstants,
}

/// System domain constants - placeholder for future extraction
#[derive(Debug, Clone, Default)]
pub struct SystemDomainConstants {
    pub environment: EnvironmentConstants,
    pub service: ServiceConstants,
    pub logging: LoggingConstants,
    pub paths: PathConstants,
    pub limits: SystemLimitConstants,
}

/// Testing domain constants - placeholder for future extraction
#[derive(Debug, Clone, Default)]
pub struct TestingDomainConstants {
    pub timeouts: TestTimeoutConstants,
    pub data: TestDataConstants,
    pub environment: TestEnvironmentConstants,
    pub limits: TestLimitConstants,
}

/// MCP domain constants - placeholder for future extraction
#[derive(Debug, Clone, Default)]
pub struct McpDomainConstants {
    pub protocol: McpProtocolConstants,
    pub messages: McpMessageConstants,
    pub connections: McpConnectionConstants,
    pub streaming: McpStreamingConstants,
}

/// Automation domain constants - placeholder for future extraction
#[derive(Debug, Clone, Default)]
pub struct AutomationDomainConstants {
    pub workflows: WorkflowConstants,
    pub scheduling: SchedulingConstants,
    pub retry: RetryConstants,
    pub batch: BatchConstants,
}

// ==================== PLACEHOLDER STRUCTS ====================
// These will be moved to their respective domain modules

#[derive(Debug, Clone, Default)]
pub struct AuthConstants;

#[derive(Debug, Clone, Default)]
pub struct EncryptionConstants;

#[derive(Debug, Clone, Default)]
pub struct SessionConstants;

#[derive(Debug, Clone, Default)]
pub struct RateLimitingConstants;

#[derive(Debug, Clone, Default)]
pub struct SecurityPolicyConstants;

#[derive(Debug, Clone, Default)]
pub struct ConcurrencyConstants;

#[derive(Debug, Clone, Default)]
pub struct CacheConstants;

#[derive(Debug, Clone, Default)]
pub struct BufferConstants;

#[derive(Debug, Clone, Default)]
pub struct BenchmarkConstants;

#[derive(Debug, Clone, Default)]
pub struct ResourceLimitConstants;

#[derive(Debug, Clone, Default)]
pub struct EnvironmentConstants;

#[derive(Debug, Clone, Default)]
pub struct ServiceConstants;

#[derive(Debug, Clone, Default)]
pub struct LoggingConstants;

#[derive(Debug, Clone, Default)]
pub struct PathConstants;

#[derive(Debug, Clone, Default)]
pub struct SystemLimitConstants;

#[derive(Debug, Clone, Default)]
pub struct TestTimeoutConstants;

#[derive(Debug, Clone, Default)]
pub struct TestDataConstants;

#[derive(Debug, Clone, Default)]
pub struct TestEnvironmentConstants;

#[derive(Debug, Clone, Default)]
pub struct TestLimitConstants;

#[derive(Debug, Clone, Default)]
pub struct McpProtocolConstants;

#[derive(Debug, Clone, Default)]
pub struct McpMessageConstants;

#[derive(Debug, Clone, Default)]
pub struct McpConnectionConstants;

#[derive(Debug, Clone, Default)]
pub struct McpStreamingConstants;

#[derive(Debug, Clone, Default)]
pub struct WorkflowConstants;

#[derive(Debug, Clone, Default)]
pub struct SchedulingConstants;

#[derive(Debug, Clone, Default)]
pub struct RetryConstants;

#[derive(Debug, Clone, Default)]
pub struct BatchConstants;

// ==================== CONVENIENCE EXPORTS ====================

/// Get the global constants instance
#[must_use]
pub fn constants() -> ConsolidatedDomainConstants {
    ConsolidatedDomainConstants::default()
}
// Re-export domain-specific convenience modules
pub use super::domains::api_constants;
pub use super::domains::network_constants;
pub use super::domains::storage_constants;
