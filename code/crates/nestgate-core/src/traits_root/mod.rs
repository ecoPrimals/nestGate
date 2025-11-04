//! Core traits for the Orchestration Orchestrator
//!
//! This module defines the fundamental traits that enable universal service orchestration
//! across different project types and deployment environments.
pub mod balancer;
pub mod communication;
pub mod config;
pub mod discovery;
pub mod health;
pub mod service;
// Re-export all trait types
pub use communication::CommunicationLayer;
pub use config::ConfigProvider;
pub use discovery::{ServiceDiscovery, ServiceEvent, ServiceQuery};
pub use health::{HealthCheck, HealthMonitor, HealthState, HealthStatus};
pub use service::*;
