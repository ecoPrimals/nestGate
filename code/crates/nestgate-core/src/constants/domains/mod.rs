// **DOMAIN-SPECIFIC CONSTANTS**
//! Module definitions and exports.
// This module organizes constants by domain for better maintainability
//! and to keep individual files under the 2000-line complexity limit.

pub mod api;
pub mod network;
pub mod storage;

// Re-export all domain constants for convenience
pub use api::ApiDomainConstants;
pub use network::NetworkDomainConstants;
pub use storage::StorageDomainConstants;

// Re-export convenience modules with different names to avoid conflicts
pub use api::api_defaults as api_constants;
pub use network::network_defaults as network_constants;
pub use storage::storage_defaults as storage_constants;
