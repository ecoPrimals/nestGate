// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **CANONICAL UNIFIED TRAIT SYSTEM**
//!
//! Unified trait system consolidating all NestGate functionality into coherent,
//! composable traits with modern async Rust patterns.
//!
//! # Modern Modular Architecture (Refactored Nov 13, 2025)
//!
//! This module was refactored from a single 1,100-line file into focused trait modules:
//!
//! - **`base`** - Core service traits (CanonicalService, CanonicalProvider) ~150 lines
//! - **`storage`** - Storage provider traits and operations ~220 lines
//! - **`network`** - Network service traits and capabilities ~200 lines
//! - **`security`** - Security and authentication traits ~300 lines
//! - **`types`** - Supporting types, enums, and structures ~190 lines
//!
//! ## Architecture Benefits
//!
//! ✅ **Trait Cohesion** - Related traits grouped by domain  
//! ✅ **Clear Hierarchy** - Base → Specialized trait progression visible  
//! ✅ **Reduced Complexity** - Each file <300 lines, focused purpose  
//! ✅ **Single Import** - Use `canonical_unified::*` for all traits  
//! ✅ **Type Safety** - Complete trait API with modern async patterns
//!
//! ## Usage
//!
//! ```rust,ignore
//! // Import all traits (recommended)
//! use nestgate_core::traits::canonical_unified::*;
//!
//! // Or import specific domains
//! use nestgate_core::traits::canonical_unified::base::*;
//! use nestgate_core::traits::canonical_unified::storage::*;
//!
//! // Implement the canonical service trait
//! impl CanonicalService for MyService {
//!     type Config = MyConfig;
//!     type Health = MyHealth;
//!     // ...
//! }
//! ```
//!
//! ## Trait Hierarchy
//!
//! ```text
//! CanonicalService (base trait)
//!   ├─> CanonicalStorage
//!   ├─> CanonicalNetwork
//!   └─> CanonicalSecurity
//!
//! CanonicalProvider<T> (generic provider)
//!   └─> Storage/Network/Security provider implementations
//! ```
//!
//! **Refactored**: November 13, 2025 (Technical Debt Elimination)  
//! **Previous Size**: 1,100 lines (single file)  
//! **Current Size**: 5 focused modules (~200 lines each)

// Re-export all traits for convenient importing
pub mod base;
pub mod storage;
pub mod network;
pub mod security;
pub mod types;

pub use base::*;
pub use storage::*;
pub use network::*;
pub use security::*;
pub use types::*;

