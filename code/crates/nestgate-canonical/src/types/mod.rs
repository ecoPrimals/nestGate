// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Canonical Types Module
//!
//! This module organizes canonical types into logical sub-modules for better maintainability.
//! Each sub-module focuses on a specific domain of types.

// Sub-modules
mod config_types;
mod health_types;
mod request_response;
mod service_types;
mod storage_types;

// Re-export all types for backward compatibility
pub use config_types::*;
pub use health_types::*;
pub use request_response::*;
pub use service_types::*;
pub use storage_types::*;

// Type alias for backward compatibility
// Note: This type alias is commented out temporarily to resolve module path issues.
// The canonical configuration system is available via nestgate_core but the exact
// module path needs to be verified.
// #[allow(deprecated)]
// pub type SecurityConfigCanonical =
//     nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
