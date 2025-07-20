//! Universal Storage Module
//!
//! Multi-protocol storage system with real-time synchronization and distributed coordination.
//! This module contains the universal storage implementation split into logical submodules:
//! - manager: Main UniversalStorageManager and coordination
//! - coordinator: Storage coordination and routing logic
//! - events: Event broadcasting and subscription system
//! - replication: Data replication management
//! - sync: Real-time synchronization engine
//! - types: Supporting types, enums, and data structures

mod coordinator;
mod events;
mod manager;
mod replication;
mod sync;
mod types;

// Re-export the main components
pub use coordinator::StorageCoordinator;
pub use events::{StorageEvent, StorageEventBroadcaster};
pub use manager::{UniversalStorageConfig, UniversalStorageManager};
pub use replication::ReplicationManager;
pub use sync::SyncEngine;
pub use types::*;
