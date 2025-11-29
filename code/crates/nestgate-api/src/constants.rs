//
// **MIGRATION COMPLETE**: All API constants now use the canonical constant system
// from `nestgate_core::constants::domain_constants`.
//
// This eliminates constant duplication and provides a single source of truth.

// **CANONICAL MODERNIZATION**: Use unified constants from core
//! Constants module

pub use nestgate_core::canonical_modernization::canonical_constants::{
    api::*, 
    network::*,
    performance::*,
};

// Legacy re-exports removed - use nestgate_core::constants instead

