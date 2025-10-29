//! **REST API HANDLERS**
//!
//! HTTP request handlers for various `NestGate` API endpoints including
//! monitoring, storage, system management, WebSocket, and ZFS operations.

/// **MONITORING HANDLERS**
///
/// API endpoints for system monitoring and health checks.
pub mod monitoring;

/// **STORAGE HANDLERS**
///
/// API endpoints for storage operations and management.
pub mod storage;

/// **SYSTEM HANDLERS**
///
/// API endpoints for system-level operations and configuration.
pub mod system;

/// **WEBSOCKET HANDLERS**
///
/// WebSocket connection handlers for real-time communication.
pub mod websocket;

/// **ZFS HANDLERS**
///
/// API endpoints for ZFS filesystem operations and management.
pub mod zfs;

// Re-export commonly used handlers for convenience
pub use monitoring::*;
pub use storage::*;
pub use system::*;
pub use websocket::*;
pub use zfs::*;

#[cfg(test)]
mod storage_tests;

#[cfg(test)]
mod websocket_tests;
