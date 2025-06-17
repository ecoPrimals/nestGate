//! NestGate Network Layer
//! 
//! This crate provides network protocol implementations and API services
//! for the NestGate system.

pub mod api;
pub mod nfs;
pub mod smb;
pub mod protocol;
pub mod vlan;

// Re-export main types
pub use api::{NetworkApi, ServiceInstance, ServiceStatus, ApiResponse};
pub use nfs::{NfsServer, NfsExport, MountRequest as NfsMountRequest, MountResponse as NfsMountResponse};
pub use smb::{SmbServer, SmbShare, SmbMountRequest, SmbMountResponse};
pub use protocol::{Protocol, ProtocolConfig, ProtocolManager, PerformancePreference};
pub use vlan::{VlanManager, VlanConfig};

/// Network layer result type
pub type Result<T> = nestgate_core::Result<T>; 