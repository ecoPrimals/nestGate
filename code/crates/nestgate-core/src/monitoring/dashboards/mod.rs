// **DASHBOARD GENERATION AND MANAGEMENT - MODULARIZED**
//! Module definitions and exports.
// This module has been refactored from a large monolithic file (878 lines) into
//! focused, maintainable modules for better code organization and compliance
//! with the <2000 lines per file standard.
//! Module definitions and exports.
// **MODULAR STRUCTURE**:
//! - `types`: Core dashboard types and configuration structures
//! - `generator`: Dashboard generation engine
//! - `grafana`: Grafana-specific dashboard generation
//! - `prometheus`: Prometheus dashboard integration
//! - `panels`: Panel configuration and management
//! - `templates`: Dashboard templates and presets
//! - `export`: Dashboard export and serialization

// ==================== MODULAR ORGANIZATION ====================

// Core dashboard types and configuration structures
pub mod types;
// Dashboard generation engine
pub mod generator;
// Grafana-specific dashboard generation
pub mod grafana;
// Prometheus dashboard integration
pub mod prometheus;
// Panel configuration and management
pub mod panels;
// Dashboard templates and presets
pub mod templates;
// Dashboard export and serialization
pub mod export;
// ==================== RE-EXPORTS FOR COMPATIBILITY ====================

pub use types::*;
pub use generator::*;
pub use grafana::*;
pub use prometheus::*;
pub use panels::*;
pub use templates::*;
pub use export::*;
