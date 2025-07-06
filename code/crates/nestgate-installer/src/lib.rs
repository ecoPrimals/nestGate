//! NestGate Installer Library
//!
//! This library provides installation and configuration functionality for NestGate.

pub mod config;
pub mod download;
pub mod gui;
pub mod installer;
pub mod platform;
pub mod wizard;

// Re-export commonly used types
pub use installer::NestGateInstaller as Installer;
pub use platform::PlatformInfo;
