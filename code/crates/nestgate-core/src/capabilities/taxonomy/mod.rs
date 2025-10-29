//! 🌱 **CAPABILITY TAXONOMY**
//!
//! Complete taxonomy of capabilities for the ecoPrimals ecosystem.
//! This replaces ALL hardcoded primal names, vendor names, and service names.
//!
//! ## Infant Discovery Philosophy
//!
//! Each primal only knows itself. All other capabilities are discovered at runtime
//! through the Universal Adapter. No hardcoded knowledge of:
//! - Which primal provides what
//! - Which vendor technology is used
//! - Which ports or endpoints exist
//!
//! Everything is discovered, nothing is assumed.
//!
//! ## Module Structure
//!
//! - `types` - Core capability types and categories
//! - `capability` - Capability struct and builder pattern
//!
//! ## Example
//!
//! ```rust
//! use nestgate_core::capabilities::taxonomy::{Capability, CapabilityType};
//!
//! // Create a discovered capability
//! let cap = Capability::new(
//!     CapabilityType::DataStorage,
//!     "http://discovered-storage:9000".to_string(),
//! )
//! .with_provider("discovered-provider".to_string())
//! .with_version("1.0.0".to_string())
//! .with_confidence(0.95);
//! ```

pub mod capability;
pub mod types;

// Re-export main types
pub use capability::Capability;
pub use types::{CapabilityCategory, CapabilityType};
