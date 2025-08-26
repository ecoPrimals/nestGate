//
// Pure data layer handlers for all NestGate API endpoints.
// These modules provide clean, authentication-free access to
// ZFS and storage data for biomeOS and other management systems.

pub mod monitoring;
pub mod storage;
pub mod system;
pub mod websocket;
pub mod zfs;

// Re-export commonly used handlers for convenience
pub use monitoring::*;
pub use storage::*;
pub use system::*;
pub use websocket::*;
pub use zfs::*;
