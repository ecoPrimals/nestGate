//! Discovery backend implementations
//!
//! This module provides concrete implementations of the DiscoveryBackend trait
//! for various service discovery mechanisms.

/// In-memory backend for testing and local development
pub mod memory;

/// mDNS backend for local network discovery
pub mod mdns;

// Re-exports
pub use mdns::MdnsDiscoveryBackend;
pub use memory::InMemoryDiscoveryBackend;
