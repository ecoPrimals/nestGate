// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Automation system for intelligent dataset management
//!
//! Provides predictive analytics and automated optimization for storage systems.

pub mod analysis;
pub mod error;
pub mod lifecycle;
pub mod manager;
pub mod types;

// Re-export commonly used types
pub use error::{AutomationError, Result};
pub use manager::IntelligentDatasetManager;
