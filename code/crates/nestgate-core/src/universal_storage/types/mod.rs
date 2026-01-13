//! # Universal Storage Types - Modular Architecture
//!
//! **Refactored from**: `consolidated_types.rs` (1,014 lines → <200 lines per module)
//!
//! This module provides a clean, domain-focused separation of storage types:
//! - **providers**: Storage types, protocols, and cloud providers
//! - **resources**: Storage resources, capabilities, and permissions
//! - **metrics**: Performance metrics and health monitoring
//! - **protocol**: Request/Response types for storage operations
//! - **config**: Configuration and requirements
//! - **events**: Event types for storage system notifications
//! - **items**: Storage items, metadata, and supporting structures
//!
//! ## Design Philosophy
//! - **Domain Separation**: Each module focuses on a single concern
//! - **Backward Compatibility**: All public types re-exported here
//! - **Maintainability**: Each file <250 lines for easy navigation
//! - **Type Safety**: Strong typing with compile-time guarantees
//!
//! ## Usage
//! ```rust
//! use nestgate_core::universal_storage::types::{
//!     UniversalStorageType,
//!     UniversalStorageResource,
//!     StoragePerformanceMetrics,
//! };
//! ```

// Module declarations
pub mod config;
pub mod events;
pub mod items;
pub mod metrics;
pub mod protocol;
pub mod providers;
pub mod resources;

// Re-export all public types for backward compatibility
pub use config::*;
pub use events::*;
pub use items::*;
pub use metrics::*;
pub use protocol::*;
pub use providers::*;
pub use resources::*;
