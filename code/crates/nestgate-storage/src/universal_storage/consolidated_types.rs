// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # Consolidated Storage Types - Backward Compatibility Layer
//!
//! **Status**: REFACTORED into modular structure
//! **New Location**: `universal_storage/types/`
//!
//! This file now serves as a backward compatibility layer, re-exporting all types
//! from the new modular structure.
//!
//! ## Migration (Optional - Recommended)
//!
//! Both paths refer to the same type; use the preferred path:
//!
//! ```rust,ignore
//! use nestgate_core::universal_storage::types::UniversalStorageType;
//!
//! let _storage = UniversalStorageType::Local;
//! ```
//!
//! ## New Modular Structure
//! - `types/providers` - Storage types and cloud providers
//! - `types/resources` - Resources, capabilities, permissions
//! - `types/metrics` - Performance metrics and health
//! - `types/protocol` - Request/Response types
//! - `types/config` - Configuration structures
//! - `types/events` - Event types
//! - `types/items` - Storage items and metadata
//!
//! **Refactored**: January 12, 2026
//! **Original Size**: 1,014 lines
//! **New Structure**: 7 focused modules, each <250 lines

// Re-export everything from the new modular structure
pub use super::types::*;
