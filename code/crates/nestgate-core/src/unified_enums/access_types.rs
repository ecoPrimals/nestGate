///
/// Unified access type definitions for consistent access control
/// across all NestGate components.
use serde::{Deserialize, Serialize};

/// Unified access types for consistent permission management
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UnifiedAccessType {
    /// Read-only access
    Read,
    /// Write access (includes read)
    Write,
    /// Execute access
    Execute,
    /// Administrative access
    Admin,
    /// Full access (all permissions)
    Full,
    /// No access
    None,
}

impl Default for UnifiedAccessType {
    fn default() -> Self {
        Self::Read
    }
}

impl UnifiedAccessType {
    /// Check if this access type includes read permissions
    pub fn can_read(&self) -> bool {
        matches!(self, Self::Read | Self::Write | Self::Admin | Self::Full)
    }

    /// Check if this access type includes write permissions
    pub fn can_write(&self) -> bool {
        matches!(self, Self::Write | Self::Admin | Self::Full)
    }

    /// Check if this access type includes execute permissions
    pub fn can_execute(&self) -> bool {
        matches!(self, Self::Execute | Self::Admin | Self::Full)
    }

    /// Check if this access type includes admin permissions
    pub fn is_admin(&self) -> bool {
        matches!(self, Self::Admin | Self::Full)
    }
}

/// Access permission levels for different resource types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccessLevel {
    /// Public access - no authentication required
    Public,
    /// Authenticated access - valid token required
    Authenticated,
    /// Authorized access - specific permissions required
    Authorized(UnifiedAccessType),
    /// Restricted access - special approval required
    Restricted,
}

impl Default for AccessLevel {
    fn default() -> Self {
        Self::Authenticated
    }
}
