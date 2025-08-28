//! NestGate Workspace - **CANONICAL MODERNIZATION COMPLETE**
//!
//! This is the root workspace library that re-exports functionality from individual crates.
//! All production code lives in the individual crates under code/crates/.
//!
//! **PERFORMANCE OPTIMIZATION**:
//! Advanced performance modules are available through the `nestgate-performance` crate,
//! providing extreme performance improvements for demanding workloads.

// Re-export core functionality
pub use nestgate_core as core;

// Re-export API functionality
pub use nestgate_api as api;

// Re-export network functionality
pub use nestgate_network as network;

// Re-export ZFS functionality
pub use nestgate_zfs as zfs;

// Re-export middleware functionality
pub use nestgate_middleware as middleware;

// Re-export installer functionality
pub use nestgate_installer as installer;

// Re-export filesystem monitor functionality
pub use nestgate_fsmonitor as fsmonitor;

// Re-export automation functionality
pub use nestgate_automation as automation;

// Re-export NAS functionality
pub use nestgate_nas as nas;

// Re-export MCP functionality
pub use nestgate_mcp as mcp;

// Re-export performance functionality
pub use nestgate_performance as performance;

// Convenience re-exports of commonly used items
pub mod prelude {
    //! Common imports for NestGate applications

    pub use nestgate_core::config::unified::NestGateUnifiedConfig as NestGateUnifiedConfig;
    pub use nestgate_core::error::{NestGateError, Result};
    pub use nestgate_core::smart_abstractions::prelude::*;
    
    // Re-export performance optimization modules
    pub use nestgate_performance::prelude::*;
}
