// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Storage Operations Modules
//!
//! Logical separation of storage operations by domain:
//! - **datasets**: Dataset management (create, list, delete)
//! - **objects**: Object storage (store, retrieve, delete)
//!
//! **Phase 3: Smart Refactoring** - Extracted from monolithic `service.rs` (Jan 30, 2026)

pub mod datasets;
pub mod objects;
