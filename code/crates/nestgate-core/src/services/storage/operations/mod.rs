//! # Storage Operations Modules
//!
//! Logical separation of storage operations by domain:
//! - **datasets**: Dataset management (create, list, delete)
//! - **objects**: Object storage (store, retrieve, delete)
//!
//! **Phase 3: Smart Refactoring** - Extracted from monolithic `service.rs` (Jan 30, 2026)

pub mod datasets;
pub mod objects;
