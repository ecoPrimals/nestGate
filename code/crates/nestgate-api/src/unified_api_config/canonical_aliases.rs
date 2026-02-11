//! Canonical type aliases for backward compatibility during config migration.
//!
//! All these aliases point to `CanonicalNetworkConfig` while deprecated structs
//! are gradually migrated. Timeline: maintained until v0.12.0 (May 2026).

#[allow(deprecated)]
use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

/// Type alias for ZfsHandlerConfig canonical migration
pub type ZfsHandlerConfigCanonical = CanonicalNetworkConfig;

/// Type alias for PerformanceHandlerConfig canonical migration
pub type PerformanceHandlerConfigCanonical = CanonicalNetworkConfig;

/// Type alias for DashboardHandlerConfig canonical migration
pub type DashboardHandlerConfigCanonical = CanonicalNetworkConfig;

/// Type alias for PerformanceAlertConfig canonical migration
pub type PerformanceAlertConfigCanonical = CanonicalNetworkConfig;

/// Type alias for AuthenticationConfig canonical migration
pub type AuthenticationConfigCanonical = CanonicalNetworkConfig;

/// Type alias for AuthHandlerConfig canonical migration
pub type AuthHandlerConfigCanonical = CanonicalNetworkConfig;

/// Type alias for AuthorizationConfig canonical migration
pub type AuthorizationConfigCanonical = CanonicalNetworkConfig;

/// Type alias for SessionConfig canonical migration
pub type SessionConfigCanonical = CanonicalNetworkConfig;

/// Type alias for WorkspaceSecurityConfig canonical migration
pub type WorkspaceSecurityConfigCanonical = CanonicalNetworkConfig;
