//
// This module contains different backend implementations for the universal ZFS service:
// - Native: Uses real ZFS commands
// - Native Real: Modular native ZFS implementation
// - Mock: Provides mock responses for testing
// - Remote: Connects to remote ZFS services

pub mod mock;
pub mod native;
pub mod native_real;
pub mod remote;

// Re-export backend implementations
pub use mock::MockZfsService;
pub use native::NativeZfsService;
pub use remote::RemoteZfsService;

// Re-export native_real for modular implementation
pub use native_real::NativeZfsService as NativeRealZfsService;
