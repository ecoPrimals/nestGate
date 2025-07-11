//! Core traits for the Songbird Orchestrator
//!
//! This module defines the fundamental traits that enable universal service orchestration
//! across different project types and deployment environments.

pub mod service;
pub mod discovery;
pub mod health;
pub mod config;
pub mod communication;
pub mod load_balancer;

// Re-export all trait types
pub use service::*;
pub use discovery::{ServiceDiscovery, ServiceQuery, ServiceEvent};
pub use health::{HealthMonitor, HealthCheck, HealthStatus, HealthState};
pub use config::ConfigProvider;
pub use communication::CommunicationLayer;