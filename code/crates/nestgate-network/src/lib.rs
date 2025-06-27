//! NestGate Network Layer
//! 
//! This crate provides network protocol implementations and API services
//! for the NestGate system. Supports both standalone operation and optional
//! ecosystem integration.

pub mod api;
pub mod nfs;
pub mod smb;
pub mod protocol;
pub mod vlan;
pub mod songbird;
pub mod connection_manager;

// Re-export main types
pub use api::{NetworkApi, ServiceInstance, ServiceStatus, ApiResponse, SongbirdClient};
pub use nfs::{NfsServer, NfsExport, MountRequest as NfsMountRequest, MountResponse as NfsMountResponse};
pub use smb::{SmbServer, SmbShare, SmbMountRequest, SmbMountResponse};
pub use protocol::{Protocol, ProtocolConfig, ProtocolManager, PerformancePreference};
pub use vlan::{VlanManager, VlanConfig};
pub use songbird::{SongbirdIntegration, SongbirdConfig, ServiceRegistration};
pub use connection_manager::{
    SongbirdConnectionManager, ConnectionType, ConnectionRequest, 
    ConnectionResponse, ActiveConnection
};

/// Network layer result type
pub type Result<T> = nestgate_core::Result<T>; 