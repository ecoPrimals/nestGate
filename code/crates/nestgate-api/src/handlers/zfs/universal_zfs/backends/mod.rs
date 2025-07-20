//! ZFS Backend Implementations
//!
//! This module contains different backend implementations for the universal ZFS service:
//! - Native: Uses real ZFS commands
//! - Mock: Provides mock responses for testing
//! - Remote: Connects to remote ZFS services

pub mod mock;
pub mod native;
pub mod remote;

// Re-export backend implementations
pub use mock::MockZfsService;
pub use native::NativeZfsService;
pub use remote::RemoteZfsService;
