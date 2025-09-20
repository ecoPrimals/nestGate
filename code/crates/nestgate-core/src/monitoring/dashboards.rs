// **DASHBOARD GENERATION AND MANAGEMENT - MODULARIZED**
//! Monitoring and observability functionality.
// This file has been refactored from a large monolithic implementation (878 lines)
//! into a clean modular structure for better maintainability and compliance with
//! the <2000 lines per file standard.
//! Monitoring and observability functionality.
// **MIGRATION**: All functionality has been moved to focused modules:
//! - `dashboards/types` - Core dashboard types and configuration structures
//! - `dashboards/generator` - Dashboard generation engine
//! - `dashboards/grafana` - Grafana-specific dashboard generation
//! - `dashboards/prometheus` - Prometheus dashboard integration
//! - `dashboards/panels` - Panel configuration and management
//! - `dashboards/templates` - Dashboard templates and presets
//! - `dashboards/export` - Dashboard export and serialization

// Re-export the modular dashboard system
pub use self::dashboards::*;

/// Modular dashboard generation and management
pub mod dashboards;
