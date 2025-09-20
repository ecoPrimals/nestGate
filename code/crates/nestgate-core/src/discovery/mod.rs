//! **DISCOVERY MODULE**
//!
//! Runtime capability discovery system implementing the Infant Discovery Architecture.
//!
//! This module provides zero-knowledge startup capabilities, allowing NestGate to
//! discover and connect to external services at runtime without hardcoded dependencies.

pub mod capability_scanner;
pub mod network_discovery;
pub mod universal_adapter;

// Re-export main types for convenience
pub use capability_scanner::{
    CapabilityInfo, CapabilityScanner, DiscoveryMethod, EnvironmentDiscovery,
};
pub use network_discovery::{DnsServiceDiscovery, IpRange, MulticastDiscovery, PortScanDiscovery};
pub use universal_adapter::{
    Connection, ConnectionMetadata, HealthStatus, Request, Response, UniversalAdapter,
};
